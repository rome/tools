/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyBridge,
	BridgeErrorResponseMessage,
	BridgeSuccessResponseMessage,
	EventCallback,
	EventSubscription,
} from "./types";
import BridgeError from "./BridgeError";
import Event from "./Event";
import {ErrorCallback, VoidCallback} from "@internal/typescript-helpers";
import {RSERValue} from "@internal/codec-binary-serial";

type CallOptions = {
	timeout?: number;
	priority?: boolean;
};

export class BridgeEvent<Param extends RSERValue, Ret extends RSERValue> {
	constructor(name: string, bridge: AnyBridge) {
		this.bridge = bridge;
		this.name = name;
		this.requestCallbacks = new Map();

		this.backingEvent = new Event({
			name,
			displayName: `event ${this.name} in ${bridge.getDisplayName()}`,
			onSubscriptionChange: () => {
				this.bridge.sendSubscriptions();
			},
		});
	}

	public name: string;
	public backingEvent: Event<Param, Ret>;
	public bridge: AnyBridge;
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
		this.backingEvent.clear();
		this.requestCallbacks.clear();
	}

	public end(err: Error) {
		for (const {reject} of this.requestCallbacks.values()) {
			reject(err);
		}
	}

	public dispatchRequest(param: Param): Promise<Ret> {
		return this.backingEvent.call(param);
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

	public hasSubscriptions(): boolean {
		return this.backingEvent.hasSubscriptions();
	}

	public hasSubscribers(): boolean {
		return this.bridge.listeners.has(this.name);
	}

	protected _send(param: Param): void {
		if (!this.hasSubscribers()) {
			// No point in sending over a subscription that doesn't have a listener
			return;
		}

		this.bridge.assertAlive();
		this.bridge.sendMessage({
			type: "request",
			event: this.name,
			param,
			priority: false,
		});
	}

	protected async _call(param: Param, opts: CallOptions = {}): Promise<Ret> {
		const {priority = false, timeout} = opts;

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

	protected async _callOptional(param: Param): Promise<undefined | Ret> {
		if (this.hasSubscribers()) {
			return this._call(param);
		} else {
			return undefined;
		}
	}

	protected _subscribe(
		callback: EventCallback<Param, Ret>,
		makeRoot?: boolean,
	): EventSubscription {
		return this.backingEvent.subscribe(callback, makeRoot);
	}

	protected _wait(val: Ret, timeout?: number): Promise<Param> {
		return this.backingEvent.wait(val, timeout);
	}
}

export class BridgeEventCallOnly<Param extends RSERValue, Ret extends RSERValue>
	extends BridgeEvent<Param, Ret> {
	public send(param: Param): void {
		return this._send(param);
	}

	public async call(param: Param, opts?: CallOptions): Promise<Ret> {
		return this._call(param, opts);
	}

	public async callOptional(param: Param): Promise<undefined | Ret> {
		return this._callOptional(param);
	}
}

export class BridgeEventListenOnly<
	Param extends RSERValue,
	Ret extends RSERValue
> extends BridgeEvent<Param, Ret> {
	public subscribe(
		callback: EventCallback<Param, Ret>,
		makeRoot?: boolean,
	): EventSubscription {
		return this._subscribe(callback, makeRoot);
	}

	public wait(val: Ret, timeout?: number): Promise<Param> {
		return this._wait(val, timeout);
	}
}

export class BridgeEventBidirectional<
	Param extends RSERValue,
	Ret extends RSERValue
> extends BridgeEvent<Param, Ret> {
	public subscribe(
		callback: EventCallback<Param, Ret>,
		makeRoot?: boolean,
	): EventSubscription {
		return this._subscribe(callback, makeRoot);
	}

	public wait(val: Ret, timeout?: number): Promise<Param> {
		return this._wait(val, timeout);
	}

	public send(param: Param): void {
		return this._send(param);
	}

	public async call(param: Param, opts?: CallOptions): Promise<Ret> {
		return this._call(param, opts);
	}

	public async callOptional(param: Param): Promise<undefined | Ret> {
		return this._callOptional(param);
	}
}
