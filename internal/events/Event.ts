/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ExtendedMap} from "@internal/collections";
import {humanizeDuration} from "@internal/string-utils";
import {EventCallback, EventOptions, EventSubscription} from "./types";

export default class Event<Param, Ret = void> {
	constructor(opts: EventOptions) {
		this.subscriptions = new ExtendedMap("subscriptions");
		this.callbacks = new Set();
		this.rootCallback = undefined;
		this.name = opts.name;
		this.displayName = opts.displayName ?? `event ${this.name}`;
		this.options = opts;
	}

	public name: string;
	public displayName: string;
	private options: EventOptions;

	private rootCallback: undefined | EventCallback<Param, Ret>;
	private callbacks: Set<EventCallback<Param, Ret>>;
	private subscriptions: ExtendedMap<
		EventCallback<Param, Ret>,
		EventSubscription
	>;

	private async callCallback(
		callback: EventCallback<Param, Ret>,
		param: Param,
	): Promise<Ret> {
		return callback(param, this.subscriptions.assert(callback));
	}

	private onSubscriptionChange() {
		const {onSubscriptionChange} = this.options;
		if (onSubscriptionChange !== undefined) {
			onSubscriptionChange();
		}
	}

	public clear() {
		this.callbacks.clear();
		this.rootCallback = undefined;
	}

	public hasSubscriptions(): boolean {
		return this.rootCallback !== undefined;
	}

	// Dispatch the event without caring about the return values
	public send(param: Param, required: boolean = false) {
		const {rootCallback: rootSubscription} = this;
		if (rootSubscription === undefined) {
			if (required) {
				throw new Error(`No subscription for ${this.displayName}`);
			}
			return;
		}

		this.callCallback(rootSubscription, param);

		for (const callback of this.callbacks) {
			this.callCallback(callback, param);
		}
	}

	public async call(param: Param): Promise<Ret> {
		const {rootCallback, callbacks: subscriptions} = this;
		if (rootCallback === undefined) {
			throw new Error(`No subscription for ${this.displayName}`);
		}

		if (this.options.serial === true) {
			const ret = await this.callCallback(rootCallback, param);
			for (const callback of subscriptions) {
				await this.callCallback(callback, param);
			}
			return ret;
		} else {
			const res = await Promise.all([
				this.callCallback(rootCallback, param),
				...Array.from(
					subscriptions,
					(callback) => this.callCallback(callback, param),
				),
			]);

			// Return the root subscription value
			return res[0];
		}
	}

	public wait(val: Ret, timeout?: number): Promise<Param> {
		return new Promise((resolve, reject) => {
			let timeoutId: undefined | NodeJS.Timeout;
			let timedOut = false;

			if (timeout !== undefined) {
				timeoutId = setTimeout(
					() => {
						timedOut = true;
						listener.unsubscribe().then(() => {
							reject(
								new Error(
									`Timed out after waiting ${humanizeDuration(timeout)} for ${this.displayName}`,
								),
							);
						}).catch((err) => {
							reject(err);
						});
					},
					timeout,
				);
			}

			const listener = this.subscribe(async (param, listener) => {
				if (timedOut) {
					return val;
				}

				if (timeoutId !== undefined) {
					clearTimeout(timeoutId);
				}

				await listener.unsubscribe();
				resolve(param);
				return val;
			});
		});
	}

	public async callOptional(param: Param): Promise<undefined | Ret> {
		if (this.rootCallback === undefined) {
			return undefined;
		} else {
			return this.call(param);
		}
	}

	public subscribe(
		callback: EventCallback<Param, Ret>,
		makeRoot?: boolean,
	): EventSubscription {
		if (this.options.unique === true && this.callbacks.size !== 0) {
			throw new Error(
				`Only allowed a single subscription for ${this.displayName}`,
			);
		}

		const subscription: EventSubscription = {
			unsubscribe: async () => {
				this.unsubscribe(callback);
			},
		};
		this.subscriptions.set(callback, subscription);

		if (this.rootCallback === callback || this.callbacks.has(callback)) {
			throw new Error(
				`Cannot double subscribe a callback for ${this.displayName}`,
			);
		}

		if (this.rootCallback === undefined) {
			this.rootCallback = callback;
		} else if (makeRoot === true) {
			this.callbacks.add(this.rootCallback);
			this.rootCallback = callback;
		} else {
			this.callbacks.add(callback);
		}

		this.onSubscriptionChange();

		return subscription;
	}

	private unsubscribe(callback: EventCallback<Param, Ret>) {
		this.subscriptions.delete(callback);

		if (this.callbacks.has(callback)) {
			this.callbacks.delete(callback);
			this.onSubscriptionChange();
			return;
		}

		// If this callback was the root subscription, then set it to the next one
		if (callback === this.rootCallback) {
			this.rootCallback = Array.from(this.callbacks)[0];
			this.callbacks.delete(this.rootCallback);
			this.onSubscriptionChange();
			return;
		}
	}
}
