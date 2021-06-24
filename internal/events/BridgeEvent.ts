/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyBridge,
	BridgeMessageCodes,
	BridgeRequestCallMessage,
	BridgeResponseMessage,
	EventCallback,
} from "./types";
import {BridgeTimeoutError} from "./errors";
import Event from "./Event";
import {ErrorCallback, VoidCallback} from "@internal/typescript-helpers";
import {RSERValue} from "@internal/binary-transport";
import {
	DIAGNOSTIC_CATEGORIES,
	decorateErrorWithDiagnostics,
	getDiagnosticsFromError,
} from "@internal/diagnostics";
import {markup} from "@internal/markup";
import {Resource, createResourceContainer} from "@internal/resources";
import {Duration} from "@internal/numbers";

type CallOptions = {
	timeout?: number;
	priority?: boolean;
};

export class BridgeEvent<
	Name extends string,
	Param extends RSERValue,
	Ret extends RSERValue
> {
	constructor(name: Name, bridge: AnyBridge) {
		this.id = ++bridge.eventsIdCounter;
		this.bridge = bridge;
		this.name = name;
		this.requestCallbacks = new Map();
		this[Symbol.toStringTag] = `BridgeEvent<${name}>`;

		this.resources = createResourceContainer(this[Symbol.toStringTag]);
		bridge.resources.add(this);

		this.backingEvent = new Event(
			name,
			{
				displayName: `event ${name} in ${bridge.displayName}`,
				onSubscriptionChange: () => {
					this.bridge.onSubscriptionChange(
						this.id,
						this.backingEvent.hasSubscriptions(),
					);
				},
			},
		);
	}

	public name: Name;
	public id: number;
	public backingEvent: Event<Param, Ret>;
	public bridge: AnyBridge;
	public requestCallbacks: Map<
		number,
		{
			resource: Resource;
			param: Param;
			completed: undefined | VoidCallback;
			resolve: (data: Ret) => void;
			reject: ErrorCallback;
		}
	>;
	public [Symbol.toStringTag]: string;
	public resources: Resource;

	public end(err: Error) {
		for (const {reject} of this.requestCallbacks.values()) {
			if (getDiagnosticsFromError(err) === undefined) {
				reject(
					decorateErrorWithDiagnostics(
						err,
						{
							description: {
								message: markup`Terminated execution of ${this.backingEvent.displayName}`,
								category: DIAGNOSTIC_CATEGORIES["bridge/closed"],
							},
						},
					),
				);
			} else {
				reject(err);
			}
		}
	}

	public dispatchRequest(param: Param): Promise<Ret> {
		return this.backingEvent.call(param);
	}

	public dispatchResponse(id: number, msg: BridgeResponseMessage): void {
		const callbacks = this.requestCallbacks.get(id);
		if (!callbacks) {
			// ???
			return;
		}

		this.requestCallbacks.delete(id);

		switch (msg[0]) {
			case BridgeMessageCodes.RESPONSE_SUCCESS: {
				// @ts-expect-error
				callbacks.resolve(msg[2]);
				break;
			}

			case BridgeMessageCodes.RESPONSE_ERROR_CUSTOM: {
				try {
					callbacks.reject(
						this.bridge.hydrateCustomError({
							errorType: "custom",
							value: msg[2],
							metadata: msg[3],
						}),
					);
				} catch (err) {
					callbacks.reject(err);
				}
				break;
			}

			case BridgeMessageCodes.RESPONSE_ERROR_NATIVE: {
				callbacks.reject(msg[2]);
				break;
			}
		}

		callbacks.resource.release();

		callbacks.completed?.();
	}

	public hasSubscriptions(): boolean {
		return this.backingEvent.hasSubscriptions();
	}

	public hasSubscribers(): boolean {
		return this.bridge.listeners.has(this.id);
	}

	protected _send(param: Param): void {
		if (!this.hasSubscribers()) {
			// No point in sending over a subscription that doesn't have a listener
			return;
		}

		this.bridge.assertOpen();
		this.bridge.sendMessage([BridgeMessageCodes.SEND, this.id, param]);
	}

	protected async _call(param: Param, opts: CallOptions = {}): Promise<Ret> {
		if (!this.hasSubscribers()) {
			return Promise.reject(
				new Error(
					`Cannot call ${this.backingEvent.displayName} as it has no subscribers`,
				),
			);
		}

		const {priority = false, timeout} = opts;

		return new Promise((resolve, reject) => {
			this.bridge.assertOpen();

			const id = this.bridge.assignRequestId(this);

			let completed;
			if (timeout !== undefined) {
				const timeoutId = setTimeout(
					() => {
						// Remove the request callback
						this.requestCallbacks.delete(id);
						resource.release();

						// Reject the promise
						reject(
							new BridgeTimeoutError(
								`Timeout of ${String(timeout)}ms for ${this.name}(${String(
									JSON.stringify(param),
								)}) event exceeded`,
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

			const resource = createResourceContainer(
				`${this[Symbol.toStringTag]}.Request<${id}>`,
				{optional: true},
			);
			this.resources.add(resource);

			this.requestCallbacks.set(
				id,
				{
					resource,
					param,
					completed,
					reject,
					resolve,
				},
			);

			const code = priority
				? BridgeMessageCodes.PRIORITY_CALL
				: BridgeMessageCodes.CALL;
			let msg: BridgeRequestCallMessage;
			if (param === undefined) {
				// Make the message a little more compact by omitting the param element
				msg = [code, this.id, id];
			} else {
				msg = [code, this.id, id, param];
			}
			this.bridge.sendMessage(msg);
		});
	}

	protected async _callOptional(param: Param): Promise<undefined | Ret> {
		if (this.hasSubscribers()) {
			return this._call(param);
		} else {
			return undefined;
		}
	}

	protected _subscribe(callback: EventCallback<Param, Ret>): Resource {
		const sub = this.backingEvent.subscribe(callback);
		this.resources.add(sub);
		return sub;
	}

	protected _wait(val: Ret, timeout?: Duration): Promise<Param> {
		return this.backingEvent.wait(val, timeout);
	}
}

export class BridgeEventCallOnly<
	Name extends string,
	Param extends RSERValue,
	Ret extends RSERValue
> extends BridgeEvent<Name, Param, Ret> {
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
	Name extends string,
	Param extends RSERValue,
	Ret extends RSERValue
> extends BridgeEvent<Name, Param, Ret> {
	public subscribe(callback: EventCallback<Param, Ret>): Resource {
		return this._subscribe(callback);
	}

	public wait(val: Ret, timeout?: Duration): Promise<Param> {
		return this._wait(val, timeout);
	}
}

export class BridgeEventBidirectional<
	Name extends string,
	Param extends RSERValue,
	Ret extends RSERValue
> extends BridgeEvent<Name, Param, Ret> {
	public subscribe(callback: EventCallback<Param, Ret>): Resource {
		return this._subscribe(callback);
	}

	public wait(val: Ret, timeout?: Duration): Promise<Param> {
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
