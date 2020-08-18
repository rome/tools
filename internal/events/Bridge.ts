/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BridgeErrorResponseDetails,
	BridgeErrorResponseMessage,
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
import BridgeEvent, {BridgeEventOptions} from "./BridgeEvent";
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

export default class Bridge {
	constructor(opts: BridgeOptions) {
		this.errorTransports = new Map();

		this.alive = true;
		this.hasHandshook = false;
		this.endError = undefined;
		this.type = opts.type;
		this.opts = opts;

		this.messageIdCounter = 0;
		this.events = new ExtendedMap("events");

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

		this.heartbeatEvent = this.createEvent({
			name: "Bridge.heartbeat",
			direction: "server<->client",
		});

		if (this.type !== "server&client") {
			this.heartbeatEvent.subscribe(() => {
				return undefined;
			});
		}

		this.init();
	}

	private heartbeatEvent: BridgeEvent<void, void>;
	private heartbeatTimeout: undefined | NodeJS.Timeout;

	private prioritizedResponses: Set<number>;
	private deprioritizedResponseQueue: Array<BridgeResponseMessage>;

	private postHandshakeQueue: Array<BridgeMessage>;
	private handshakeEvent: Event<
		{
			first: boolean;
			subscriptions: Array<string>;
		},
		void
	>;
	private hasHandshook: boolean;

	public endEvent: Event<Error, void>;
	public alive: boolean;
	private endError: undefined | Error;
	public type: BridgeType;

	private messageIdCounter: number;

	// rome-ignore lint/ts/noExplicitAny
	private events: ExtendedMap<string, BridgeEvent<any, any>>;

	public listeners: Set<string>;
	public updatedListenersEvent: Event<Set<string>, void>;

	private opts: BridgeOptions;

	// rome-ignore lint/ts/noExplicitAny
	private errorTransports: Map<string, ErrorSerial<any>>;

	public attachEndSubscriptionRemoval(subscription: EventSubscription) {
		this.endEvent.subscribe(async () => {
			await subscription.unsubscribe();
		});
	}

	private getPendingRequestsSummary(): AnyMarkups {
		const summaries: AnyMarkups = [];

		for (const event of this.events.values()) {
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
		if (this.type === "server&client") {
			// No point in monitoring this since we're the same process
			return;
		}

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

	public async handshake(
		opts: {
			timeout?: number;
			second?: boolean;
		} = {},
	): Promise<void> {
		if (this.hasHandshook) {
			throw new Error("Already performed handshake");
		}

		const {timeout, second = false} = opts;

		// Send a handshake in case we were the first
		if (!second) {
			this.sendMessage({
				type: "handshake",
				first: true,
				subscriptions: this.getSubscriptions(),
			});
		}

		// Wait for a handshake from the other end
		const res = await this.handshakeEvent.wait(undefined, timeout);

		if (res.first) {
			// Send the handshake again, as it wouldn't have received the first
			this.sendMessage({
				type: "handshake",
				first: false,
				subscriptions: this.getSubscriptions(),
			});
		}

		this.receivedSubscriptions(res.subscriptions);

		this.hasHandshook = true;

		for (const msg of this.postHandshakeQueue) {
			this.sendMessage(msg);
		}
		this.postHandshakeQueue = [];
	}

	public getSubscriptions(): Array<string> {
		const names = [];
		for (const event of this.events.values()) {
			if (event.hasSubscriptions()) {
				names.push(event.name);
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

	private receivedSubscriptions(names: Array<string>): void {
		this.listeners = new Set(names);
		this.updatedListenersEvent.send(this.listeners);
	}

	public attachRSER(): RSERStream {
		const buf = new RSERStream();

		buf.errorEvent.subscribe((err) => {
			this.endWithError(err);
		});

		buf.valueEvent.subscribe((value) => {
			process.nextTick(() => {
				this.handleMessage((value as BridgeMessage));
			});
		});

		return buf;
	}

	public init(): void {
		// This method can be overridden by subclasses, it allows you to add logic such as error serializers
	}

	private clear(): void {
		for (const [, event] of this.events) {
			event.clear();
		}
	}

	public getNextMessageId(): number {
		return ++this.messageIdCounter;
	}

	public createEvent<Param extends RSERValue, Ret extends RSERValue>(
		opts: BridgeEventOptions,
	): BridgeEvent<Param, Ret> {
		if (this.events.has(opts.name)) {
			throw new Error("Duplicate event");
		}

		const event = new BridgeEvent<Param, Ret>(opts, this);
		this.events.set(opts.name, event);
		return event;
	}

	//# Connection death
	public assertAlive(): void {
		if (this.endError !== undefined) {
			throw this.endError;
		}
	}

	public endWithError(err: Error): void {
		if (!this.alive) {
			return;
		}

		this.alive = false;
		this.endError = err;

		// Reject any pending requests
		for (const [, event] of this.events) {
			event.end(err);
		}
		this.clear();

		// Clear any currently processing heartbeat
		if (this.heartbeatTimeout !== undefined) {
			clearTimeout(this.heartbeatTimeout);
		}

		// Notify listeners
		this.endEvent.send(err, true);
	}

	public end(message: string = "Connection died") {
		this.endWithError(new BridgeError(message, this));
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

	// rome-ignore lint/ts/noExplicitAny
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

		const {opts} = this;
		opts.sendMessage(msg);
		if (opts.onSendMessage !== undefined) {
			opts.onSendMessage(msg);
		}
	}

	public handleMessage(msg: BridgeMessage) {
		try {
			this.assertAlive();

			if (msg.type === "handshake") {
				this.handshakeEvent.send({
					subscriptions: msg.subscriptions,
					first: msg.first,
				});
			}

			if (msg.type === "subscriptions") {
				this.receivedSubscriptions(msg.names);
			}

			if (msg.type === "request") {
				this.handleMessageRequest(msg);
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

		const eventHandler = this.events.assert(event);
		eventHandler.dispatchResponse(id, data);
	}

	private handleMessageRequest(data: BridgeRequestMessage) {
		const {id, event, param, priority} = data;
		if (event === undefined) {
			throw new Error("Expected event in message request but received none");
		}

		const eventHandler = this.events.assert(event);

		if (id === undefined) {
			eventHandler.dispatchRequest(param).catch((err) => {
				this.endWithError(err);
			});
		} else {
			if (priority) {
				this.prioritizedResponses.add(id);
			}

			eventHandler.dispatchRequest(param).then(
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
			).catch((err) => this.endWithError(err));
		}
	}
}
