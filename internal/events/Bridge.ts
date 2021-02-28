/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BridgeDefinition,
	BridgeErrorDetails,
	BridgeErrorResponseMessage,
	BridgeEventsDeclaration,
	BridgeEventsDeclarationToInstances,
	BridgeHandshakeMessage,
	BridgeHeartbeatExceededOptions,
	BridgeMessage,
	BridgeMessageCodes,
	BridgeRequestCallMessage,
	BridgeRequestSendMessage,
	BridgeResponseMessage,
	BridgeType,
} from "./types";
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
	NodeSystemError,
} from "@internal/errors";
import {AnyMarkups, concatMarkup, markup} from "@internal/markup";
import prettyFormat from "@internal/pretty-format";
import {RSERObject, RSERStream, RSERValue} from "@internal/binary";
import {ExtendedMap} from "@internal/collections";
import { createRuntimeDiagnosticError, DIAGNOSTIC_CATEGORIES } from "@internal/diagnostics";
import { BridgeTimeoutError } from "./errors";
import { createResourceFromCallback, Resource } from "@internal/resources";
import { isBridgeResponseMessage, isBridgeDisconnectedDiagnosticError } from "./utils";
import { Duration, DurationMeasurer } from "@internal/numbers";

type ErrorSerial<Data extends RSERObject> = {
	serialize: (err: Error) => Data;
	hydrate: (err: StructuredError, obj: Data) => NodeSystemError;
};

// rome-ignore lint/ts/noExplicitAny: future cleanup
export type AnyBridgeEvent = BridgeEvent<any, any>;

export default class Bridge<
	ListenEvents extends BridgeEventsDeclaration,
	CallEvents extends BridgeEventsDeclaration,
	SharedEvents extends BridgeEventsDeclaration
> {
	constructor(
		type: BridgeType,
		def: BridgeDefinition<{}, {}, SharedEvents>,
		listenEvents: ListenEvents,
		callEvents: CallEvents,
		SharedEvents: SharedEvents,
	) {
		this.customErrorTransports = new Map();

		this.open = true;
		this.connected = true;

		this.hasHandshook = false;
		this.endError = undefined;
		this.debugName = def.debugName;
		this.type = type;

		this.requestIdCounter = 0;
		this.nameToEventMap = new ExtendedMap("nameToEventMap");
		this.idToEventMap = new ExtendedMap("eventIdToEventMap");
		this.requestIdToEvent = new ExtendedMap("requestIdToEvent");

		const debugPrefix = `Bridge.${type}<${this.debugName}>`;
		this.receivedMessageEvent = new Event(`${debugPrefix}.receivedMessageEvent`);
		this.sendMessageEvent = new Event(`${debugPrefix}.sendMessageEvent`);
		this.handshakeEvent = new Event(`${debugPrefix}.handshake`);
		this.heartbeatExceededEvent = new Event(`${debugPrefix}.heartbeatExceededEvent`);
		this.heartbeatEvent = new Event(`${debugPrefix}.heartbeat`);
		this.endEvent = new Event(`${debugPrefix}.endEvent`);
		this.disconnectEvent = new Event(`${debugPrefix}.disconnectEvent`);

		this.resources = createResourceFromCallback(debugPrefix, async () => {
			await this.end("Resource released");
		});

		// A Set of event names that are being listened to on the other end
		// We track this to avoid sending over subscriptions that aren't needed
		this.listeners = new Set();
		this.lastSentSubscriptionChange = new Map();

		this.prioritizedResponses = new Set();
		this.deprioritizedResponseQueue = [];

		this.postHandshakeQueue = [];

		// @ts-ignore: Cannot safely type this but the code to build it is fine
		this.events = {};
		this.eventsIdCounter = 0;

		this.teardownEvent = this.createInternalBiEvent(`teardown`);
		this.teardownEvent.subscribe(async () => {
			await this.end("Graceful teardown requested", false);
		});

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

		if (def.init !== undefined) {
			def.init(this);
		}
	}

	private teardownEvent: BridgeEventBidirectional<void, void>;

	private heartbeatEvent: Event<void, void>;
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
	public eventsIdCounter: number;

	public heartbeatExceededEvent: Event<BridgeHeartbeatExceededOptions, void>;
	public receivedMessageEvent: Event<BridgeMessage, void>;
	public sendMessageEvent: Event<BridgeMessage, void>;
	public resources: Resource;
	public disconnectEvent: Event<void, void>;
	public endEvent: Event<Error, void>;
	
	public open: boolean;
	private connected: boolean;
	private endError: undefined | Error;

	public type: BridgeType;
	protected debugName: string;

	private requestIdCounter: number;

	private nameToEventMap: ExtendedMap<string, AnyBridgeEvent>;
	private idToEventMap: ExtendedMap<number, AnyBridgeEvent>;
	private requestIdToEvent: ExtendedMap<number, AnyBridgeEvent>;

	public listeners: Set<number>;
	private lastSentSubscriptionChange: Map<number, boolean>;
	private customErrorTransports: Map<string, ErrorSerial<RSERObject>>;

	public getDisplayName(): string {
		return `${this.debugName}(${this.type})`;
	}

	private createInternalBiEvent<Param extends RSERValue, Ret extends RSERValue>(name: string): BridgeEventBidirectional<Param, Ret> {
		const event: BridgeEventBidirectional<Param, Ret> = new BridgeEventBidirectional(`@${name}`, this);
		this.registerEvent(event);
		return event;
	}

	private getPendingRequestsSummary(): AnyMarkups {
		const summaries: AnyMarkups = [];

		for (const event of this.nameToEventMap.values()) {
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

	private startHeartbeatMonitor(timeout: Duration) {
		function checkFresh() {
			// We use setTimeout so that check is called without a call stack
			setTimeout(() => { check(); }, 0);
		}

		const check = async ({attempts, startTime}: {
			attempts: number;
			startTime: DurationMeasurer;
		} = {
			attempts: 1,
			startTime: new DurationMeasurer(),
		}) => {
			try {
				await this.heartbeatEvent.wait(undefined, timeout);
				checkFresh();
			} catch (err) {
				if (err instanceof BridgeTimeoutError) {
					if (this.open) {
						this.heartbeatExceededEvent.send({
							summary: this.getPendingRequestsSummary(),
							attempts,
							totalTime: startTime.since(),
						});
						check({
							startTime,
							attempts: attempts + 1,
						});
					}
				} else {
					this.endWithError(err);
				}
			}
		};

		checkFresh();
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
			monitorHeartbeat?: Duration;
			timeout?: Duration;
		} = {},
	): Promise<void> {
		if (this.hasHandshook) {
			throw new Error("Already performed handshake");
		}

		const {timeout, monitorHeartbeat} = opts;

		// Clients will always be the first to send the handshake
		const isClient = this.type === "client";
		const isServer = this.type === "server";

		if (isClient) {
			const idMap: Map<number, string> = new Map();
			for (const [name, event] of this.nameToEventMap) {
				idMap.set(event.id, name);
			}

			this.sendMessage([BridgeMessageCodes.CLIENT_HANDSHAKE, monitorHeartbeat, this.getSubscriptionsForHandshake(), idMap]);
		}

		// Reject if the bridge ends while waiting on our handshake
		let endSub: undefined | Resource;
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
			await endSub.release();
		}

		if (isServer) {
			this.sendMessage([BridgeMessageCodes.SERVER_HANDSHAKE, monitorHeartbeat, this.getSubscriptionsForHandshake()]);
		}

		this.hasHandshook = true;

		if (monitorHeartbeat !== undefined) {
			this.startHeartbeatMonitor(monitorHeartbeat);
		}

		for (const msg of this.postHandshakeQueue) {
			this.sendMessage(msg);
		}
		this.postHandshakeQueue = [];
	}

	public getSubscriptionsForHandshake(): Set<number> {
		const ids: Set<number> = new Set();
		for (const event of this.nameToEventMap.values()) {
			if (event.hasSubscriptions()) {
				ids.add(event.id);
				this.lastSentSubscriptionChange.set(event.id, true);
			}
		}
		return ids;
	}

	public onSubscriptionChange(id: number, hasSubscriptions: boolean): void {
		if (!this.hasHandshook) {
			// If we haven't had the handshake then no point sending them. They'll be sent all at once after
			return;
		}

		// Nobody to send an update to
		if (!this.open) {
			return;
		}

		// If the other end already knows of this subscription then no point sending it
		if (this.lastSentSubscriptionChange.get(id) === hasSubscriptions) {
			return;
		}

		if (hasSubscriptions) {
			this.sendMessage([BridgeMessageCodes.SUBSCRIBED, id]);
		} else {
			this.sendMessage([BridgeMessageCodes.UNSUBSCRIBED, id]);
		}
		this.lastSentSubscriptionChange.set(id, hasSubscriptions);
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

	public assignRequestId(event: AnyBridgeEvent): number {
		// Reset counter when idle with no messages
		if (this.requestIdToEvent.size === 0) {
			this.requestIdCounter = 0;
		}

		const id = ++this.requestIdCounter;
		this.requestIdToEvent.set(id, event);
		return id;
	}

	public registerEvent<Param extends RSERValue, Ret extends RSERValue>(
		event: BridgeEvent<Param, Ret>,
	): BridgeEvent<Param, Ret> {
		if (this.nameToEventMap.has(event.name)) {
			throw new Error(`Duplicate event name "${event.name}"`);
		}
		this.nameToEventMap.set(event.name, event);

		if (this.idToEventMap.has(event.id)) {
			throw new Error(`Duplicate event id "${event.id}"`);
		}
		this.idToEventMap.set(event.id, event);

		return event;
	}

	//# Connection death
	public assertOpen(): void {
		if (this.endError !== undefined) {
			throw this.endError;
		}
	}

	public async endWithError(err: Error, gracefulTeardown: boolean = true) {
		if (!this.open) {
			return;
		}

		// Reject any pending requests
		for (const event of this.nameToEventMap.values()) {
			event.end(err);
		}

		// Clear any currently processing heartbeat
		if (this.heartbeatTimeout !== undefined) {
			clearTimeout(this.heartbeatTimeout);
		}

		// Request a teardown if necessary
		let gracefulTeardownPromise;
		if (gracefulTeardown) {
			gracefulTeardownPromise = this.teardownEvent.call();
		}

		// Do this after we dispatch the teardown event request
		this.open = false;
		this.endError = err;

		// Wait on other teardown if necessary as if we call our end listeners at the same time then it will
		// close the connection
		if (gracefulTeardownPromise !== undefined) {
			try {
				await gracefulTeardownPromise;
			} catch (err) {
				// We expect the bridge to disconnect as the indicator that it has finished
				if (!isBridgeDisconnectedDiagnosticError(err)) {
					throw err;
				}
			}
		}

		// Notify listeners
		await this.endEvent.callOptional(err);
		await this.resources.release();
	}

	public async disconnected(message: string) {
		if (!this.connected) {
			return;
		}

		this.connected = false;
		await this.endWithError(createRuntimeDiagnosticError({
			description: {
				message,
				category: DIAGNOSTIC_CATEGORIES["bridge/disconnected"],
			}
		}), false);
		await this.disconnectEvent.callOptional();
	}

	public async end(
		message: string = "Connection died",
		gracefulTeardown: boolean = true,
	) {
		await this.endWithError(createRuntimeDiagnosticError({
			description: {
				message,
				category: DIAGNOSTIC_CATEGORIES["bridge/closed"],
			}
		}), gracefulTeardown);
	}

	//# Error serialization

	public hydrateCustomError(details: BridgeErrorDetails) {
		if (details.errorType === "native") {
			return details.value;
		}

		const {value: struct, metadata} = details;
		const transport = this.customErrorTransports.get(struct.name);
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

	public serializeCustomError(errRaw: unknown): BridgeErrorDetails {
		// Just in case something that wasn't an Error was thrown
		const err = errRaw instanceof Error ? errRaw : new Error(String(errRaw));

		// If we have no custom error transport then we'll just rely on the binary codec transport
		const transport = this.customErrorTransports.get(err.name);
		if (transport === undefined) {
			return {
				errorType: "native",
				value: err,
			};
		}

		return {
			errorType: "custom",
			value: getErrorStructure(err),
			metadata: transport.serialize(err),
		};
	}

	private buildErrorResponse(
		id: number,
		errRaw: unknown,
	): BridgeErrorResponseMessage {
		const details = this.serializeCustomError(errRaw);
		if (details.errorType === "custom") {
			return [BridgeMessageCodes.RESPONSE_ERROR_CUSTOM, id, details.value, details.metadata];
		} else {
			return [BridgeMessageCodes.RESPONSE_ERROR_NATIVE, id, details.value];
		}
	}

	public addCustomErrorTransport<T extends RSERObject>(name: string, transport: ErrorSerial<T>) {
		// @ts-ignore: Dynamicism
		this.customErrorTransports.set(name, transport);
	}

	//# Message transmission

	// There's no try-catch gated around sendMessage because the call stack here will include some other error handler
	// We need to be specific for handleMessage because it could come from anywhere
	public sendMessage(msg: BridgeMessage) {
		this.assertOpen();
		
		if (msg[0] !== BridgeMessageCodes.CLIENT_HANDSHAKE && msg[0] !== BridgeMessageCodes.SERVER_HANDSHAKE && !this.hasHandshook) {
			this.postHandshakeQueue.push(msg);
			return;
		}

		if (isBridgeResponseMessage(msg)) {
			const id = msg[1];
			if (
				this.prioritizedResponses.size > 0 &&
				!this.prioritizedResponses.has(id)
			) {
				this.deprioritizedResponseQueue.push(msg);
				return;
			}

			if (this.prioritizedResponses.has(id)) {
				this.clearPrioritization(id);
			}
		}

		this.sendMessageEvent.call(msg);
	}

	public async handleMessage(msg: BridgeMessage) {
		try {
			this.assertOpen();
			this.receivedMessageEvent.send(msg);

			switch (msg[0]) {
				case BridgeMessageCodes.SERVER_HANDSHAKE:
				case BridgeMessageCodes.CLIENT_HANDSHAKE:{
					await this.handleHandshakeMessage(msg);
					break;
				}

				case BridgeMessageCodes.HEARTBEAT: {
					this.heartbeatEvent.send();
					break;
				}

				case BridgeMessageCodes.SUBSCRIBED: {
					this.listeners.add(msg[1]);
					break;
				}

				case BridgeMessageCodes.UNSUBSCRIBED: {
					this.listeners.delete(msg[1]);
					break;
				}

				case BridgeMessageCodes.PRIORITY_CALL:
				case BridgeMessageCodes.CALL: {
					await this.handleCallMessage(msg);
					break;
				}

				case BridgeMessageCodes.SEND: {
					await this.handleSendMessage(msg);
					break;
				}

				case BridgeMessageCodes.RESPONSE_SUCCESS:
				case BridgeMessageCodes.RESPONSE_ERROR_CUSTOM:
				case BridgeMessageCodes.RESPONSE_ERROR_NATIVE: {
					this.handleResponseMessage(msg);
					break;
				}
			}
		} catch (err) {
			this.endWithError(err);
		}
	}

	public getDebugMessage(raw: BridgeMessage): unknown[] {
		// Clone
		const msg: unknown[] = raw.slice();

		// Map message ID
		const msgId = raw[0];
		let msgName = BridgeMessageCodes[msgId];
		msg[0] = [msgId, msgName];

		// Map message event
		switch (raw[0]) {
			case BridgeMessageCodes.CALL:
			case BridgeMessageCodes.PRIORITY_CALL:
			case BridgeMessageCodes.SEND: {
				const eventId = raw[1];
				const eventInst = this.idToEventMap.get(eventId);
				const eventName = eventInst === undefined ? "undefined" : eventInst.name;
				msg[1] = [eventId, eventName];
				break;
			}

			case BridgeMessageCodes.RESPONSE_ERROR_CUSTOM:
			case BridgeMessageCodes.RESPONSE_ERROR_NATIVE:
			case BridgeMessageCodes.RESPONSE_SUCCESS: {
				const requestId = raw[1];
				const eventHandler = this.requestIdToEvent.assert(requestId);
				msg[1] = [requestId, eventHandler.name];
				break;
			}
		}
	
		return msg;
	}

	private handleResponseMessage(
		data: BridgeResponseMessage,
	) {
		const id = data[1];
		if (id === undefined) {
			throw new Error("Expected id");
		}

		const eventHandler = this.requestIdToEvent.assert(id);
		this.requestIdToEvent.delete(id);
		eventHandler.dispatchResponse(id, data);
	}

	private async handleSendMessage(data: BridgeRequestSendMessage): Promise<void> {
		const eventId = data[1];
		const param = data[2];

		const eventHandler = this.idToEventMap.assert(eventId);
		await eventHandler.dispatchRequest(param).catch((err) => {
			this.endWithError(err);
		});
	}

	private async handleHandshakeMessage(msg: BridgeHandshakeMessage): Promise<void> {
		const heartbeatTimeout = msg[1];
		if (heartbeatTimeout !== undefined) {
			// The heartbeatTimeout indicates at least how often we expect to receive a heartbeat. We send them
			// more often than that to keep the connection alive and mitigate an overloaded event loop.
			this.resources.addTimeout("Heartbeat", heartbeatTimeout.divide(2).setInterval(() => {
				this.heartbeatEvent.send();
			}));
		}

		this.listeners = msg[2];

		// If we received a client handshake, then make sure our internal event IDs match what it gave us
		// This is so later we can support a backwards compatible bridge for a backend service that has multiple supported interfaces
		if (msg[0] === BridgeMessageCodes.CLIENT_HANDSHAKE) {
			const unusedEvents: Set<AnyBridgeEvent> = new Set(this.nameToEventMap.values());
		
			// Reset eventsIdCounter
			this.eventsIdCounter = 0;

			// Reassign the IDs of supported events
			for (const [id, name] of msg[3]) {
				const event = this.nameToEventMap.get(name);
				if (event === undefined) {
					// NB: Might be better to error in this case? The client calling this will result in an error.
					continue;
				}

				// We don't use eventsIdCounter after all the initial events have been initialized but let's do it anyway for
				// data purity
				if (id > this.eventsIdCounter) {
					this.eventsIdCounter = id;
				}

				// Reassign id
				event.id = id;
				this.idToEventMap.set(id, event);
				unusedEvents.delete(event);
			}

			// Remove unused events entirely as they should never be allowed to be called
			for (const event of unusedEvents) {
				this.nameToEventMap.delete(event.name);

				// Only delete from idToEventMap if it hasn't already been reassigned
				if (this.idToEventMap.get(event.id) === event) {
					this.idToEventMap.delete(event.id);
				}
			}
		}

		await this.handshakeEvent.call();
	}

	private async handleCallMessage(msg: BridgeRequestCallMessage): Promise<void> {
		const eventId = msg[1];
		const id = msg[2];
		const param = msg[3];
		const priority = msg[0] === BridgeMessageCodes.PRIORITY_CALL;

		const eventHandler = this.idToEventMap.assert(eventId);

		if (priority) {
			this.prioritizedResponses.add(id);
		}

		await eventHandler.dispatchRequest(param).then(
			(value) => {
				this.sendMessage([BridgeMessageCodes.RESPONSE_SUCCESS, id, value]);
			},
			(err) => {
				this.sendMessage(this.buildErrorResponse(id, err));
			},
		);
	}
}
