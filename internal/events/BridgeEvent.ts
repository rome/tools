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

	private validateDirection(
		invalidDirections: [BridgeEventDirection, BridgeType][],
		verb: string,
	) {
		for (const [eventDirection, bridgeType] of invalidDirections) {
			if (this.direction === eventDirection && this.bridge.type === bridgeType) {
				throw new Error(
					`The ${this.getDisplayName()} cannot be ${verb} by this sort of bridge`,
				);
			}
		}
	}

	protected getDisplayName(): string {
		const {bridge} = this;
		return `${super.getDisplayName()}(${this.direction}) in ${bridge.getDisplayName()}`;
	}

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
		this.validateDirection(
			[["server<-client", "client"], ["server->client", "server"]],
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
		this.validateDirection(
			[["server<-client", "server"], ["server->client", "client"]],
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

	public async callOptional(param: Param): Promise<undefined | Ret> {
		if (this.hasSubscribers()) {
			return this.call(param);
		} else {
			return undefined;
		}
	}
}
