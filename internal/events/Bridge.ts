/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BridgeErrorResponseDetails,
	BridgeErrorResponseMessage,
	BridgeEventsDeclaration,
	BridgeEventsDeclarationToInstances,
	BridgeHeartbeatExceededOptions,
	BridgeMessage,
	BridgeOptions,
	BridgeRequestMessage,
	BridgeResponseMessage,
	BridgeSuccessResponseMessage,
	BridgeType,
	EventSubscription,
} from "./types";
import BridgeError from "./BridgeError";
import {
	BridgeEvent,
	BridgeEventBidirectional,
	BridgeEventCallOnly,
	BridgeEventListenOnly,
} from "./BridgeEvent";
import Event from "./Event";
import {
	ErrorWithFrames,
	StructuredError,
	getErrorStructure,
	setErrorFrames,
} from "@internal/v8";
import {AnyMarkups, concatMarkup, markup} from "@internal/markup";
import {AsyncVoidCallback} from "@internal/typescript-helpers";
import prettyFormat from "@internal/pretty-format";
import {NodeSystemError} from "@internal/node";
import {RSERObject, RSERStream, RSERValue} from "@internal/codec-binary-serial";
import {ExtendedMap} from "@internal/collections";

type ErrorSerial<Data extends RSERValue> = {
	serialize: (err: Error) => Data;
	hydrate: (err: StructuredError, obj: Data) => NodeSystemError;
};

export default class Bridge<
	ListenEvents extends BridgeEventsDeclaration,
	CallEvents extends BridgeEventsDeclaration,
	SharedEvents extends BridgeEventsDeclaration
> {
	constructor(
		opts: BridgeOptions,
		listenEvents: ListenEvents,
		callEvents: CallEvents,
		SharedEvents: SharedEvents,
	) {
		this.errorTransports = new Map();

		this.alive = true;
		this.hasHandshook = false;
		this.endError = undefined;
		this.debugName = opts.debugName;
		this.type = opts.type;

		this.messageIdCounter = 0;
		this.eventsMap = new ExtendedMap("events");

		this.sendMessageEvent = new Event({
			name: "Bridge.sendMessageEvent",
		});
		this.handshakeEvent = new Event({
			name: "Bridge.handshake",
		});
		this.endEvent = new Event({
			name: "Bridge.end",
			serial: true,
		});
		this.updatedListenersEvent = new Event({
			name: "Bridge.updatedListenersEvent",
		});

		// A Set of event names that are being listened to on the other end
		// We track this to avoid sending over subscriptions that aren't needed
		this.listeners = new Set();

		this.prioritizedResponses = new Set();
		this.deprioritizedResponseQueue = [];

		this.postHandshakeQueue = [];

		this.heartbeatEvent = new BridgeEventBidirectional("Bridge.heartbeat", this);
		this.registerEvent(this.heartbeatEvent);
		this.heartbeatEvent.subscribe(() => {
			return undefined;
		});

		this.teardownEvent = new BridgeEventBidirectional("Bridge.teardown", this);
		this.registerEvent(this.teardownEvent);
		this.teardownEvent.subscribe(async () => {
			await this.end("Graceful teardown requested", false);
		});

		// @ts-ignore: Cannot safely type this but the code to build it is fine
		this.events = {};

		for (const name in callEvents) {
			const event = new BridgeEventCallOnly(name, this);
			// @ts-ignore
			this.events[name] = event;
			this.registerEvent(event);
		}

		for (const name in listenEvents) {
			const event = new BridgeEventListenOnly(name, this);
			// @ts-ignore
			this.events[name] = event;
			this.registerEvent(event);
		}

		for (const name in SharedEvents) {
			const event = new BridgeEventBidirectional(name, this);
			// @ts-ignore
			this.events[name] = event;
			this.registerEvent(event);
		}

		this.init();
	}

	private teardownEvent: BridgeEventBidirectional<void, void>;

	private heartbeatEvent: BridgeEventBidirectional<void, void>;
	private heartbeatTimeout: undefined | NodeJS.Timeout;

	private prioritizedResponses: Set<number>;
	private deprioritizedResponseQueue: BridgeResponseMessage[];

	private postHandshakeQueue: BridgeMessage[];
	private handshakeEvent: Event<void, void>;
	private hasHandshook: boolean;

	public events: BridgeEventsDeclarationToInstances<
		ListenEvents,
		CallEvents,
		SharedEvents
	>;

	public sendMessageEvent: Event<BridgeMessage, void>;
	public endEvent: Event<Error, void>;
	public alive: boolean;
	private endError: undefined | Error;
	protected debugName: string;
	public type: BridgeType;

	private messageIdCounter: number;

	// rome-ignore lint/ts/noExplicitAny: future cleanup
	private eventsMap: ExtendedMap<string, BridgeEvent<any, any>>;

	public listeners: Set<string>;
	public updatedListenersEvent: Event<Set<string>, void>;

	// rome-ignore lint/ts/noExplicitAny: future cleanup
	private errorTransports: Map<string, ErrorSerial<any>>;

	public getDisplayName(): string {
		return `${this.debugName}(${this.type})`;
	}

	public attachEndSubscriptionRemoval(subscription: EventSubscription) {
		this.endEvent.subscribe(async () => {
			await subscription.unsubscribe();
		});
	}

	private getPendingRequestsSummary(): AnyMarkups {
		const summaries: AnyMarkups = [];

		for (const event of this.eventsMap.values()) {
			const requestCount = event.requestCallbacks.size;
			if (requestCount > 0) {
				let list = Array.from(
					event.requestCallbacks.values(),
					({param}) => {
						return markup`<li>${prettyFormat(param)}</li>`;
					},
				);

				summaries.push(
					markup`<emphasis>${event.name}</emphasis> x ${requestCount}\n<ul>${concatMarkup(
						list,
					)}</ul>`,
				);
			}
		}

		return summaries;
	}

	public monitorHeartbeat(
		timeout: number,
		onExceeded: AsyncVoidCallback<[BridgeHeartbeatExceededOptions]>,
		{
			iterations,
			initialTime,
		}: {
			iterations: number;
			initialTime: number;
		} = {iterations: 0, initialTime: 0},
	) {
		const start = Date.now();

		this.heartbeatTimeout = setTimeout(
			async () => {
				try {
					await this.heartbeatEvent.call(undefined, {timeout});
				} catch (err) {
					if (err instanceof BridgeError) {
						if (this.alive) {
							const took = Date.now() - start;
							onExceeded({
								summary: this.getPendingRequestsSummary(),
								iterations,
								totalTime: initialTime + took,
							});
						}
					} else {
						throw err;
					}
				} finally {
					const took = Date.now() - start;
					this.monitorHeartbeat(
						timeout,
						onExceeded,
						{
							initialTime: initialTime + took,
							iterations: iterations + 1,
						},
					);
				}
			},
			1_000,
		);
	}

	private clearPrioritization(id: number) {
		this.prioritizedResponses.delete(id);

		if (this.prioritizedResponses.size === 0) {
			for (const msg of this.deprioritizedResponseQueue) {
				this.sendMessage(msg);
			}
			this.deprioritizedResponseQueue = [];
		}
	}

	private sendHandshakeMessages() {
		this.sendMessage({
			type: "handshake",
			subscriptions: this.getSubscriptions(),
		});
	}

	public async handshake(
		opts: {
			timeout?: number;
		} = {},
	): Promise<void> {
		if (this.hasHandshook) {
			throw new Error("Already performed handshake");
		}

		const {timeout} = opts;

		// Clients will always be the first to send the handshake
		const isClient = this.type === "client";
		const isServer = this.type === "server";

		if (isClient) {
			this.sendHandshakeMessages();
		}

		// Reject if the bridge ends while waiting on our handshake
		let endSub: undefined | EventSubscription;
		let endPromise = new Promise((resolve, reject) => {
			endSub = this.endEvent.subscribe((err) => {
				reject(err);
			});
		});

		// Wait for a handshake from the other end
		await Promise.race([
			this.handshakeEvent.wait(undefined, timeout),
			endPromise,
		]);

		if (endSub !== undefined) {
			await endSub.unsubscribe();
		}

		if (isServer) {
			this.sendHandshakeMessages();
		}

		this.hasHandshook = true;

		for (const msg of this.postHandshakeQueue) {
			this.sendMessage(msg);
		}
		this.postHandshakeQueue = [];
	}

	public getSubscriptions(): Set<string> {
		const names: Set<string> = new Set();
		for (const event of this.eventsMap.values()) {
			if (event.hasSubscriptions()) {
				names.add(event.name);
			}
		}
		return names;
	}

	public sendSubscriptions(): void {
		if (!this.hasHandshook) {
			// If we haven't had the handshake then no point sending them. They'll be sent all at once after
			return;
		}

		// Nobody to send an update to
		if (!this.alive) {
			return;
		}

		// Notify the other side of what we're currently subscribed to
		// We send over a list of all of our subscriptions every time
		// This is fine since we don't change subscriptions often and they aren't very large
		// If we have a lot of subscriptions, or are changing them a lot in the future then this could be optimized
		this.sendMessage({
			type: "subscriptions",
			names: this.getSubscriptions(),
		});
	}

	private receivedSubscriptions(names: Set<string>): void {
		this.listeners = names;
		this.updatedListenersEvent.send(this.listeners);
	}

	public attachRSER(): RSERStream {
		const buf = new RSERStream(this.type);

		buf.errorEvent.subscribe((err) => {
			this.endWithError(err, false);
		});

		buf.valueEvent.subscribe((value) => {
			process.nextTick(() => {
				this.handleMessage(value as BridgeMessage);
			});
		});

		this.sendMessageEvent.subscribe((data) => {
			buf.sendValue(data);
		});

		return buf;
	}

	public init(): void {
		// This method can be overridden by subclasses, it allows you to add logic such as error serializers
	}

	private clear(): void {
		for (const [, event] of this.eventsMap) {
			event.clear();
		}
	}

	public getNextMessageId(): number {
		return ++this.messageIdCounter;
	}

	public registerEvent<Param extends RSERValue, Ret extends RSERValue>(
		event: BridgeEvent<Param, Ret>,
	): BridgeEvent<Param, Ret> {
		if (this.eventsMap.has(event.name)) {
			throw new Error("Duplicate event");
		}

		this.eventsMap.set(event.name, event);
		return event;
	}

	//# Connection death
	public assertAlive(): void {
		if (this.endError !== undefined) {
			throw this.endError;
		}
	}

	public async endWithError(err: Error, gracefulTeardown: boolean = true) {
		if (!this.alive) {
			return;
		}

		// Reject any pending requests
		for (const [, event] of this.eventsMap) {
			event.end(err);
		}
		this.clear();

		// Create another sneaky request to request a teardown
		let gracefulTeardownPromise;
		if (gracefulTeardown) {
			gracefulTeardownPromise = this.teardownEvent.call();
		}

		// Then don't allow anymore
		this.alive = false;
		this.endError = err;

		// Wait on other teardown if necessary as if we call our end listeners at the same time then it will
		// close the connection
		if (gracefulTeardownPromise !== undefined) {
			await gracefulTeardownPromise;
		}

		// Clear any currently processing heartbeat
		if (this.heartbeatTimeout !== undefined) {
			clearTimeout(this.heartbeatTimeout);
		}

		// Notify listeners
		await this.endEvent.callOptional(err);
	}

	public async end(
		message: string = "Connection died",
		gracefulTeardown: boolean = true,
	) {
		this.endWithError(new BridgeError(message, this), gracefulTeardown);
	}

	//# Error serialization

	public hydrateError({value: struct, metadata}: BridgeErrorResponseDetails) {
		const transport = this.errorTransports.get(struct.name);
		if (transport === undefined) {
			const err: ErrorWithFrames = new Error(struct.message);
			err.name = struct.name || "Error";
			err.stack = struct.stack;
			setErrorFrames(err, struct.frames);
			return err;
		} else {
			return transport.hydrate(struct, metadata);
		}
	}

	public serializeError(errRaw: unknown): BridgeErrorResponseDetails {
		// Just in case something that wasn't an Error was thrown
		const err = errRaw instanceof Error ? errRaw : new Error(String(errRaw));

		// Fetch some metadata for hydration
		const tranport = this.errorTransports.get(err.name);
		const metadata: RSERObject =
			tranport === undefined ? {} : tranport.serialize(err);

		return {
			value: getErrorStructure(err),
			metadata,
		};
	}

	private buildErrorResponse(
		id: number,
		event: string,
		errRaw: unknown,
	): BridgeErrorResponseMessage {
		return {
			id,
			event,
			type: "response",
			responseStatus: "error",
			...this.serializeError(errRaw),
		};
	}

	// rome-ignore lint/ts/noExplicitAny: future cleanup
	public addErrorTransport(name: string, transport: ErrorSerial<any>) {
		this.errorTransports.set(name, transport);
	}

	//# Message transmission

	public sendMessage(msg: BridgeMessage) {
		// There's no try-catch gated around sendMessage because the call stack here will include some other error handler
		// We need to be specific for handleMessage because it could come from anywhere
		if (msg.type !== "handshake" && !this.hasHandshook) {
			this.postHandshakeQueue.push(msg);
			return;
		}

		this.assertAlive();

		if (msg.type === "response") {
			if (
				this.prioritizedResponses.size > 0 &&
				!this.prioritizedResponses.has(msg.id)
			) {
				this.deprioritizedResponseQueue.push(msg);
				return;
			}

			if (this.prioritizedResponses.has(msg.id)) {
				this.clearPrioritization(msg.id);
			}
		}

		this.sendMessageEvent.call(msg);
	}

	public async handleMessage(msg: BridgeMessage) {
		try {
			this.assertAlive();

			if (msg.type === "handshake") {
				this.receivedSubscriptions(msg.subscriptions);
				await this.handshakeEvent.call();
			}

			if (msg.type === "subscriptions") {
				this.receivedSubscriptions(msg.names);
			}

			if (msg.type === "request") {
				await this.handleMessageRequest(msg);
			}

			if (msg.type === "response") {
				this.handleMessageResponse(msg);
			}
		} catch (err) {
			this.endWithError(err);
		}
	}

	private handleMessageResponse(
		data: BridgeSuccessResponseMessage | BridgeErrorResponseMessage,
	) {
		const {id, event} = data;
		if (id === undefined) {
			throw new Error("Expected id");
		}
		if (event === undefined) {
			throw new Error("Expected event");
		}

		const eventHandler = this.eventsMap.assert(event);
		eventHandler.dispatchResponse(id, data);
	}

	private async handleMessageRequest(data: BridgeRequestMessage) {
		const {id, event, param, priority} = data;
		if (event === undefined) {
			throw new Error("Expected event in message request but received none");
		}

		const eventHandler = this.eventsMap.assert(event);

		if (id === undefined) {
			await eventHandler.dispatchRequest(param).catch((err) => {
				this.endWithError(err);
			});
		} else {
			if (priority) {
				this.prioritizedResponses.add(id);
			}

			await eventHandler.dispatchRequest(param).then(
				(value) => {
					this.sendMessage({
						event,
						id,
						type: "response",
						responseStatus: "success",
						value,
					});
				},
				(err) => {
					this.sendMessage(this.buildErrorResponse(id, event, err));
				},
			);
		}
	}
}
