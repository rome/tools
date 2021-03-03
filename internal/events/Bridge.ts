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
	BridgeOptions,
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
	NodeSystemError,
	StructuredError,
	getErrorStructure,
	setErrorFrames,
} from "@internal/errors";
import {Markup, joinMarkup, markup} from "@internal/markup";
import prettyFormat from "@internal/pretty-format";
import {RSERObject, RSERStream} from "@internal/binary-transport";
import {ExtendedMap} from "@internal/collections";
import {
	DIAGNOSTIC_CATEGORIES,
	createRuntimeDiagnosticsError,
} from "@internal/diagnostics";
import {Resource, createResourceFromCallback} from "@internal/resources";
import {
	isBridgeDisconnectedDiagnosticsError,
	isBridgeResponseMessage,
} from "./utils";
import {Duration, DurationMeasurer} from "@internal/numbers";
import util = require("util");

type ErrorSerial<Data extends RSERObject> = {
	serialize: (err: Error) => Data;
	hydrate: (err: StructuredError, obj: Data) => NodeSystemError;
};

// rome-ignore lint/ts/noExplicitAny: future cleanup
export type AnyBridgeEvent = BridgeEvent<string, any, any>;

export default class Bridge<
	ListenEvents extends BridgeEventsDeclaration,
	CallEvents extends BridgeEventsDeclaration,
	SharedEvents extends BridgeEventsDeclaration
> {
	constructor(
		type: BridgeType,
		opts: BridgeOptions,
		def: BridgeDefinition<{}, {}, SharedEvents>,
		listenEvents: ListenEvents,
		callEvents: CallEvents,
		SharedEvents: SharedEvents,
	) {
		this.customErrorTransports = new Map();

		this.open = true;
		this.connected = true;

		this.heartbeatTimeout = undefined;
		this.hasHandshook = false;
		this.endError = undefined;
		this.debugName = def.debugName;
		this.displayName = `${def.debugName}(${type})`;
		this.type = type;
		this.options = opts;

		this.requestIdCounter = 0;
		this.nameToEventMap = new ExtendedMap("nameToEventMap");
		this.idToEventMap = new ExtendedMap("eventIdToEventMap");
		this.requestIdToEvent = new ExtendedMap("requestIdToEvent");

		const debugPrefix = `Bridge.${type}<${this.debugName}>`;
		this[Symbol.toStringTag] = debugPrefix;

		this.heartbeatEvent = new Event(`${debugPrefix}.heartbeatEvent`);
		this.receivedMessageEvent = new Event(`${debugPrefix}.receivedMessageEvent`);
		this.sendMessageEvent = new Event(`${debugPrefix}.sendMessageEvent`);
		this.handshakeEvent = new Event(`${debugPrefix}.handshake`);
		this.endEvent = new Event(`${debugPrefix}.endEvent`);
		this.disconnectEvent = new Event(`${debugPrefix}.disconnectEvent`);

		this.resources = createResourceFromCallback(
			debugPrefix,
			async () => {
				await this.end("Resource released");
			},
			{
				optional: opts.optionalResource,
			},
		);

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

		for (const name in callEvents) {
			this.registerEvent(new BridgeEventCallOnly(name, this));
		}

		for (const name in listenEvents) {
			this.registerEvent(new BridgeEventListenOnly(name, this));
		}

		for (const name in SharedEvents) {
			this.registerEvent(new BridgeEventBidirectional(name, this));
		}

		if (def.init !== undefined) {
			def.init(this);
		}
	}

	private heartbeatTimeout: undefined | NodeJS.Timeout;

	private prioritizedResponses: Set<number>;
	private deprioritizedResponseQueue: BridgeResponseMessage[];

	private postHandshakeQueue: BridgeMessage[];
	private handshakeEvent: Event<void, void>;
	private hasHandshook: boolean;

	public [Symbol.toStringTag]: string;
	public events: BridgeEventsDeclarationToInstances<
		ListenEvents,
		CallEvents,
		SharedEvents
	>;
	public eventsIdCounter: number;

	public type: BridgeType;
	public displayName: string;
	public open: boolean;
	public heartbeatEvent: Event<void, void>;
	public receivedMessageEvent: Event<BridgeMessage, void>;
	public sendMessageEvent: Event<BridgeMessage, void>;
	public resources: Resource;
	public disconnectEvent: Event<void, void>;
	public endEvent: Event<Error, void>;

	private connected: boolean;
	private endError: undefined | Error;

	private options: BridgeOptions;
	private debugName: string;

	private requestIdCounter: number;
	private nameToEventMap: ExtendedMap<string, AnyBridgeEvent>;
	private idToEventMap: ExtendedMap<number, AnyBridgeEvent>;
	private requestIdToEvent: ExtendedMap<number, AnyBridgeEvent>;

	public listeners: Set<number>;
	private lastSentSubscriptionChange: Map<number, boolean>;
	private customErrorTransports: Map<string, ErrorSerial<RSERObject>>;

	private getPendingRequestsSummary(): Markup[] {
		const summaries: Markup[] = [];

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
					markup`<emphasis>${event.name}</emphasis> x ${requestCount}\n<ul>${joinMarkup(
						list,
					)}</ul>`,
				);
			}
		}

		return summaries;
	}

	// Ensure we are sending a message at least once per second
	private queueHeartbeat(): void {
		if (!(this.hasHandshook && this.open)) {
			return;
		}

		if (this.heartbeatTimeout !== undefined) {
			clearTimeout(this.heartbeatTimeout);
		}

		this.heartbeatTimeout = setTimeout(
			() => {
				this.sendMessage([BridgeMessageCodes.HEARTBEAT]);
			},
			1_000,
		);
	}

	public startHeartbeatMonitor(
		timeout: Duration,
		callback: (opts: BridgeHeartbeatExceededOptions) => void,
	) {
		if (this.options.ignoreHeartbeat) {
			return;
		}

		function checkFresh() {
			// We use setTimeout so that check is called without a call stack
			setTimeout(
				() => {
					check();
				},
				0,
			);
		}

		const check = async () => {
			let attempts = 0;
			let startTime = new DurationMeasurer();

			const timer = timeout.setInterval(() => {
				attempts++;

				callback({
					summary: this.getPendingRequestsSummary(),
					attempts,
					totalTime: startTime.since(),
				});
			});

			await this.heartbeatEvent.wait(undefined);
			timer.release();
			checkFresh();
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
			timeout?: Duration;
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
			const idMap: Map<number, string> = new Map();
			for (const [name, event] of this.nameToEventMap) {
				idMap.set(event.id, name);
			}

			this.sendMessage([
				BridgeMessageCodes.CLIENT_HANDSHAKE,
				this.getSubscriptionsForHandshake(),
				idMap,
			]);
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
			this.sendMessage([
				BridgeMessageCodes.SERVER_HANDSHAKE,
				this.getSubscriptionsForHandshake(),
			]);
		}

		this.hasHandshook = true;
		this.queueHeartbeat();

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

	public registerEvent(event: AnyBridgeEvent): void {
		const {name, id} = event;

		if (this.nameToEventMap.has(name)) {
			throw new Error(`Duplicate event name "${event.name}"`);
		}
		this.nameToEventMap.set(name, event);

		if (this.idToEventMap.has(id)) {
			throw new Error(`Duplicate event id "${id}"`);
		}
		this.idToEventMap.set(id, event);

		// @ts-ignore: Necessary evil XqjWidsMhvM
		this.events[name] = event;
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
		this.terminateEventRequests(err);

		// Send teardown message that will be handled async
		if (gracefulTeardown) {
			this.sendMessage([BridgeMessageCodes.TEARDOWN]);
		}

		// Do this after we dispatch the teardown event request
		this.open = false;
		this.endError = err;

		// Clear any pending heartbeat
		if (this.heartbeatTimeout !== undefined) {
			clearTimeout(this.heartbeatTimeout);
		}

		// Wait on a disconnect when requesting a graceful teardown
		if (gracefulTeardown) {
			try {
				// NB: Might be good to have a timer here
				await this.disconnectEvent.wait();
			} catch (err) {
				// We expect the bridge to disconnect as the indicator that it has finished
				if (!isBridgeDisconnectedDiagnosticsError(err)) {
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
		const err = createRuntimeDiagnosticsError({
			description: {
				message,
				category: DIAGNOSTIC_CATEGORIES["bridge/disconnected"],
			},
		});

		// Terminate pending requests. This happens in endWithError too but if this disconnect happens inside of a previous end call
		// then it's a noop
		this.terminateEventRequests(err);

		await this.endWithError(err, false);
		await this.disconnectEvent.callOptional();
	}

	private terminateEventRequests(err: Error) {
		for (const event of this.nameToEventMap.values()) {
			event.end(err);
		}
	}

	public async end(
		message: string = "Connection died",
		gracefulTeardown: boolean = true,
	) {
		await this.endWithError(
			createRuntimeDiagnosticsError({
				description: {
					message,
					category: DIAGNOSTIC_CATEGORIES["bridge/closed"],
				},
			}),
			gracefulTeardown,
		);
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
		const err = util.types.isNativeError(errRaw)
			? errRaw
			: new Error(String(errRaw));

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
			return [
				BridgeMessageCodes.RESPONSE_ERROR_CUSTOM,
				id,
				details.value,
				details.metadata,
			];
		} else {
			return [BridgeMessageCodes.RESPONSE_ERROR_NATIVE, id, details.value];
		}
	}

	public addCustomErrorTransport<T extends RSERObject>(
		name: string,
		transport: ErrorSerial<T>,
	) {
		// @ts-ignore: Dynamicism
		this.customErrorTransports.set(name, transport);
	}

	//# Message transmission

	// There's no try-catch gated around sendMessage because the call stack here will include some other error handler
	// We need to be specific for handleMessage because it could come from anywhere
	public sendMessage(msg: BridgeMessage) {
		this.assertOpen();

		if (
			msg[0] !== BridgeMessageCodes.CLIENT_HANDSHAKE &&
			msg[0] !== BridgeMessageCodes.SERVER_HANDSHAKE &&
			!this.hasHandshook
		) {
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

		this.queueHeartbeat();
		this.sendMessageEvent.call(msg);
	}

	public async handleMessage(msg: BridgeMessage) {
		try {
			this.assertOpen();
			this.heartbeatEvent.send();
			this.receivedMessageEvent.send(msg);

			switch (msg[0]) {
				case BridgeMessageCodes.SERVER_HANDSHAKE:
				case BridgeMessageCodes.CLIENT_HANDSHAKE: {
					await this.handleHandshakeMessage(msg);
					break;
				}

				case BridgeMessageCodes.TEARDOWN: {
					await this.end("Graceful teardown requested", false);
					break;
				}

				case BridgeMessageCodes.HEARTBEAT: {
					// Doesn't need to be handled as we just want it for the events
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

	private handleResponseMessage(data: BridgeResponseMessage) {
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

	private async handleHandshakeMessage(
		msg: BridgeHandshakeMessage,
	): Promise<void> {
		this.listeners = msg[1];

		// If we received a client handshake, then make sure our internal event IDs match what it gave us
		// This is so later we can support a backwards compatible bridge for a backend service that has multiple supported interfaces
		if (msg[0] === BridgeMessageCodes.CLIENT_HANDSHAKE) {
			const unusedEvents: Set<AnyBridgeEvent> = new Set(
				this.nameToEventMap.values(),
			);

			// Reset eventsIdCounter
			this.eventsIdCounter = 0;

			// Reassign the IDs of supported events
			for (const [id, name] of msg[2]) {
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
				if (this.open) {
					this.sendMessage([BridgeMessageCodes.RESPONSE_SUCCESS, id, value]);
				}
			},
			(err) => {
				if (this.open) {
					this.sendMessage(this.buildErrorResponse(id, err));
				}
			},
		);
	}
}
