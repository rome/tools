/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {EventOptions, EventSubscription} from "./types";

type Callback<Param, Ret> = (param: Param) => Ret | Promise<Ret>;

export default class Event<Param, Ret = void> {
	constructor(opts: EventOptions) {
		this.subscriptions = new Set();
		this.rootSubscription = undefined;
		this.name = opts.name;
		this.options = opts;
	}

	public name: string;
	private options: EventOptions;

	private rootSubscription: undefined | Callback<Param, Ret>;
	private subscriptions: Set<Callback<Param, Ret>>;

	public onSubscriptionChange() {
		// Hook for BridgeEvent
	}

	public clear() {
		this.subscriptions.clear();
		this.rootSubscription = undefined;
	}

	public hasSubscribers(): boolean {
		return this.hasSubscriptions();
	}

	public hasSubscriptions(): boolean {
		return this.rootSubscription !== undefined;
	}

	// Dispatch the event without caring about the return values
	public send(param: Param, required: boolean = false) {
		const {rootSubscription} = this;
		if (rootSubscription === undefined) {
			if (required) {
				throw new Error(`No subscription for event ${this.name}`);
			}
			return;
		}

		rootSubscription(param);

		for (const callback of this.subscriptions) {
			callback(param);
		}
	}

	public async call(param: Param): Promise<Ret> {
		const {rootSubscription, subscriptions} = this;
		if (rootSubscription === undefined) {
			throw new Error(`No subscription for event ${this.name}`);
		}

		if (this.options.serial === true) {
			const ret = await rootSubscription(param);
			for (const callback of subscriptions) {
				await callback(param);
			}
			return ret;
		} else {
			const res = await Promise.all([
				rootSubscription(param),
				...Array.from(subscriptions, (callback) => callback(param)),
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
									`Timed out after waiting ${timeout}ms for ${this.name}`,
								),
							);
						}).catch((err) => {
							reject(err);
						});
					},
					timeout,
				);
			}

			const listener = this.subscribe(async (param) => {
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
		if (this.rootSubscription === undefined) {
			return undefined;
		} else {
			return this.call(param);
		}
	}

	public subscribe(
		callback: Callback<Param, Ret>,
		makeRoot?: boolean,
	): EventSubscription {
		if (this.options.unique === true && this.subscriptions.size !== 0) {
			throw new Error(`Event ${this.name} only allows a single subscription`);
		}

		if (this.rootSubscription === callback || this.subscriptions.has(callback)) {
			throw new Error("Cannot double subscribe a callback");
		}

		if (this.rootSubscription === undefined) {
			this.rootSubscription = callback;
		} else if (makeRoot === true) {
			this.subscriptions.add(this.rootSubscription);
			this.rootSubscription = callback;
		} else {
			this.subscriptions.add(callback);
		}

		this.onSubscriptionChange();

		return {
			unsubscribe: async () => {
				this.unsubscribe(callback);
			},
		};
	}

	private unsubscribe(callback: Callback<Param, Ret>) {
		if (this.subscriptions.has(callback)) {
			this.subscriptions.delete(callback);
			this.onSubscriptionChange();
			return;
		}

		// If this callback was the root subscription, then set it to the next one
		if (callback === this.rootSubscription) {
			this.rootSubscription = Array.from(this.subscriptions)[0];
			this.subscriptions.delete(this.rootSubscription);
			this.onSubscriptionChange();
			return;
		}
	}
}
