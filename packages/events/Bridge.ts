/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BridgeErrorResponseMessage,
	BridgeMessage,
	BridgeOptions,
	BridgeRequestMessage,
	BridgeResponseMessage,
	BridgeSuccessResponseMessage,
	BridgeType,
	EventSubscription,
} from "./types";
import {JSONObject, JSONPropertyValue} from "@romefrontend/codec-json";
import BridgeError from "./BridgeError";
import BridgeEvent, {BridgeEventOptions} from "./BridgeEvent";
import Event from "./Event";
import {
	ErrorWithFrames,
	StructuredError,
	getErrorStructure,
	setErrorFrames,
} from "@romefrontend/v8";

type ErrorJSON = {
	serialize: (err: Error) => JSONObject;
	hydrate: (err: StructuredError, obj: JSONObject) => Error;
};

export default class Bridge {
	constructor(opts: BridgeOptions) {
		this.errorTransports = new Map();

		this.alive = true;
		this.endError = undefined;
		this.type = opts.type;
		this.opts = opts;

		this.messageIdCounter = 0;
		this.events = new Map();

		this.hasHandshook = false;
		this.handshakeEvent = new Event({name: "Bridge.handshake"});
		this.endEvent = new Event({name: "Bridge.end", serial: true});
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

	heartbeatEvent: BridgeEvent<void, void>;
	heartbeatTimeout: undefined | NodeJS.Timeout;

	prioritizedResponses: Set<number>;
	deprioritizedResponseQueue: Array<BridgeResponseMessage>;
	postHandshakeQueue: Array<BridgeMessage>;

	handshakeEvent: Event<
		{
			first: boolean;
			subscriptions: Array<string>;
		},
		void
	>;
	hasHandshook: boolean;
	endEvent: Event<Error, void>;

	alive: boolean;
	endError: undefined | Error;
	type: BridgeType;

	messageIdCounter: number;
	// rome-ignore lint/js/noExplicitAny
	events: Map<string, BridgeEvent<any, any>>;

	listeners: Set<string>;
	updatedListenersEvent: Event<Set<string>, void>;

	opts: BridgeOptions;
	errorTransports: Map<string, ErrorJSON>;

	attachEndSubscriptionRemoval(subscription: EventSubscription) {
		this.endEvent.subscribe(() => {
			subscription.unsubscribe();
		});
	}

	monitorHeartbeat(timeout: number, onExceeded: () => undefined | Promise<void>) {
		if (this.type === "server&client") {
			// No point in monitoring this since we're the same process
			return;
		}

		this.heartbeatTimeout = setTimeout(
			async () => {
				try {
					await this.heartbeatEvent.call(undefined, {timeout});
					this.monitorHeartbeat(timeout, onExceeded);
				} catch (err) {
					if (err instanceof BridgeError) {
						if (this.alive) {
							onExceeded();
						}
					} else {
						throw err;
					}
				}
			},
			1_000,
		);
	}

	clearPrioritization(id: number) {
		this.prioritizedResponses.delete(id);

		if (this.prioritizedResponses.size === 0) {
			for (const msg of this.deprioritizedResponseQueue) {
				this.sendMessage(msg);
			}
			this.deprioritizedResponseQueue = [];
		}
	}

	async handshake(
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

	getSubscriptions(): Array<string> {
		const names = [];
		for (const event of this.events.values()) {
			if (event.hasSubscriptions()) {
				names.push(event.name);
			}
		}
		return names;
	}

	sendSubscriptions(): void {
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

	receivedSubscriptions(names: Array<string>): void {
		this.listeners = new Set(names);
		this.updatedListenersEvent.send(this.listeners);
	}

	init(): void {
		// This method can be overridden by subclasses, it allows you to add logic such as error serializers
	}

	clear(): void {
		for (const [, event] of this.events) {
			event.clear();
		}
	}

	getNextMessageId(): number {
		return ++this.messageIdCounter;
	}

	createEvent<Param extends JSONPropertyValue, Ret extends JSONPropertyValue>(
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
	assertAlive(): void {
		if (this.endError !== undefined) {
			throw this.endError;
		}
	}

	endWithError(err: Error): void {
		if (this.alive === false) {
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
		this.endEvent.callSync(err);
	}

	end(message: string = "Connection died") {
		this.endWithError(new BridgeError(message, this));
	}

	//# Error serialization
	buildError(struct: StructuredError, data: JSONObject) {
		const transport = this.errorTransports.get(struct.name);
		if (transport === undefined) {
			const err: ErrorWithFrames = new Error(struct.message);
			err.name = struct.name || "Error";
			err.stack = struct.stack;
			setErrorFrames(err, struct.frames);
			return err;
		} else {
			return transport.hydrate(struct, data);
		}
	}

	buildErrorResponse(
		id: number,
		event: string,
		errRaw: unknown,
	): BridgeErrorResponseMessage {
		// Just in case something that wasn't an Error was thrown
		const err = errRaw instanceof Error ? errRaw : new Error(String(errRaw));

		// Fetch some metadata for hydration
		const tranport = this.errorTransports.get(err.name);
		const metadata: JSONObject =
			tranport === undefined ? {} : tranport.serialize(err);

		return {
			id,
			event,
			type: "response",
			responseStatus: "error",
			value: getErrorStructure(err),
			metadata,
		};
	}

	addErrorTransport(name: string, transport: ErrorJSON) {
		this.errorTransports.set(name, transport);
	}

	//# Message transmission
	sendMessage(msg: BridgeMessage) {
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

	handleJSONMessage(str: string) {
		try {
			const data = JSON.parse(str);
			this.handleMessage(data);
		} catch (err) {
			if (err instanceof SyntaxError) {
				this.endWithError(
					new BridgeError(`Error parsing message JSON: ${err.message}`, this),
				);
			} else {
				this.endWithError(err);
			}
		}
	}

	handleMessage(msg: BridgeMessage) {
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

	handleMessageResponse(
		data: BridgeSuccessResponseMessage | BridgeErrorResponseMessage,
	) {
		const {id, event} = data;
		if (id === undefined) {
			throw new Error("Expected id");
		}
		if (event === undefined) {
			throw new Error("Expected event");
		}

		const eventHandler = this.events.get(event);
		if (eventHandler === undefined) {
			throw new Error("Unknown event");
		}

		eventHandler.dispatchResponse(id, data);
	}

	handleMessageRequest(data: BridgeRequestMessage) {
		const {id, event, param, priority} = data;
		if (event === undefined) {
			throw new Error("Expected event in message request but received none");
		}

		const eventHandler = this.events.get(event);
		if (eventHandler === undefined) {
			throw new Error(`Unknown event ${event}`);
		}

		if (id === undefined) {
			// We don't need to do anything with the return value of this since
			// there's nothing on the other end to catch it
			eventHandler.dispatchRequest(param);
		} else {
			if (priority) {
				this.prioritizedResponses.add(id);
			}

			eventHandler.dispatchRequest(param).then((value) => {
				this.sendMessage({
					event,
					id,
					type: "response",
					responseStatus: "success",
					value,
				});
			}).catch((err) => {
				this.sendMessage(this.buildErrorResponse(id, event, err));
			}).catch((err) => this.endWithError(err));
		}
	}
}
