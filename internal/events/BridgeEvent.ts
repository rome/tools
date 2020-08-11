/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BridgeErrorResponseMessage,
	BridgeSuccessResponseMessage,
	BridgeType,
	EventOptions,
} from "./types";
import Bridge from "./Bridge";
import BridgeError from "./BridgeError";
import Event from "./Event";
import {ErrorCallback, VoidCallback} from "@internal/typescript-helpers";
import {RSERValue} from "@internal/codec-binary-serial";

type CallOptions = {
	timeout?: number;
	priority?: boolean;
};

export type BridgeEventDirection =
	| "server->client"
	| "server<-client"
	| "server<->client";

export type BridgeEventOptions = EventOptions & {
	direction: BridgeEventDirection;
};

function validateDirection(
	// rome-ignore lint/ts/noExplicitAny
	event: BridgeEvent<any, any>,
	invalidDirections: Array<[BridgeEventDirection, BridgeType]>,
	verb: string,
) {
	invalidDirections.push(["server<->client", "server&client"]);

	for (const [eventDirection, bridgeType] of invalidDirections) {
		if (event.direction === eventDirection && event.bridge.type === bridgeType) {
			throw new Error(
				`The ${eventDirection} event "${event.name}" cannot be ${verb} by a ${bridgeType} bridge`,
			);
		}
	}
}

export default class BridgeEvent<Param extends RSERValue, Ret extends RSERValue>
	extends Event<Param, Ret> {
	constructor(opts: BridgeEventOptions, bridge: Bridge) {
		super(opts);

		this.bridge = bridge;
		this.requestCallbacks = new Map();
		this.direction = opts.direction;
	}

	public bridge: Bridge;
	public direction: BridgeEventDirection;
	public requestCallbacks: Map<
		number,
		{
			param: Param;
			completed: undefined | VoidCallback;
			resolve: (data: Ret) => void;
			reject: ErrorCallback;
		}
	>;

	public clear() {
		super.clear();
		this.requestCallbacks.clear();
	}

	public end(err: Error) {
		for (const {reject} of this.requestCallbacks.values()) {
			reject(err);
		}
	}

	public onSubscriptionChange() {
		validateDirection(
			this,
			[["server->client", "client"], ["server<-client", "server"]],
			"subscribed",
		);
		this.bridge.sendSubscriptions();
	}

	public dispatchRequest(param: Param): Promise<Ret> {
		return super.call(param);
	}

	public dispatchResponse(
		id: number,
		data: BridgeSuccessResponseMessage | BridgeErrorResponseMessage,
	): void {
		const callbacks = this.requestCallbacks.get(id);
		if (!callbacks) {
			// ???
			return;
		}

		this.requestCallbacks.delete(id);

		if (data.responseStatus === "success") {
			// @ts-ignore
			callbacks.resolve(data.value);
		} else if (data.responseStatus === "error") {
			try {
				callbacks.reject(this.bridge.hydrateError(data));
			} catch (err) {
				callbacks.reject(err);
			}
		} else {
			// ???
		}

		if (callbacks.completed !== undefined) {
			callbacks.completed();
		}
	}

	public hasSubscribers(): boolean {
		return this.bridge.listeners.has(this.name);
	}

	private validateCanSend(): void {
		validateDirection(
			this,
			[["server<-client", "client"], ["server->client", "server"]],
			"called",
		);
	}

	public send(param: Param): void {
		if (!this.hasSubscribers()) {
			// No point in sending over a subscription that doesn't have a listener
			return;
		}

		this.validateCanSend();
		this.bridge.assertAlive();
		this.bridge.sendMessage({
			type: "request",
			event: this.name,
			param,
			priority: false,
		});
	}

	public async call(param: Param, opts: CallOptions = {}): Promise<Ret> {
		const {priority = false, timeout} = opts;
		this.validateCanSend();

		return new Promise((resolve, reject) => {
			this.bridge.assertAlive();

			const id = this.bridge.getNextMessageId();

			let completed;
			if (timeout !== undefined) {
				const timeoutId = setTimeout(
					() => {
						// Remove the request callback
						this.requestCallbacks.delete(id);

						// Reject the promise
						reject(
							new BridgeError(
								`Timeout of ${String(timeout)}ms for ${this.name}(${String(
									JSON.stringify(param),
								)}) event exceeded`,
								this.bridge,
							),
						);
					},
					timeout,
				);

				// Cancel the timeout if the response returns before the timer
				completed = () => {
					clearTimeout(timeoutId);
				};
			}

			this.requestCallbacks.set(
				id,
				{
					param,
					completed,
					reject,
					resolve,
				},
			);

			this.bridge.sendMessage({
				id,
				event: this.name,
				param,
				type: "request",
				priority,
			});
		});
	}

	public callOptional(): never {
		throw new Error(`callOptional not allowed on BridgeEvent ${this.name}`);
	}
}
