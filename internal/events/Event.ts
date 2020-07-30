/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {EventOptions, EventSubscription} from "./types";

type Callback<Param, Ret> = (param: Param) => Ret | Promise<Ret>;

function noPromise<Ret>(ret: Ret | Promise<Ret>): Ret {
	if (ret instanceof Promise) {
		throw new Error("Subscription returned promise for a callSync");
	} else {
		return ret;
	}
}

export default class Event<Param, Ret = void> {
	constructor(opts: EventOptions) {
		this.subscriptions = new Set();
		this.rootSubscription = undefined;
		this.name = opts.name;
		this.options = opts;
	}

	name: string;
	options: EventOptions;

	rootSubscription: undefined | Callback<Param, Ret>;
	subscriptions: Set<Callback<Param, Ret>>;

	onSubscriptionChange() {
		// Hook for BridgeEvent
	}

	onError(err: Error) {
		const {onError} = this.options;
		if (onError !== undefined) {
			onError(err);
		}
	}

	clear() {
		this.subscriptions.clear();
		this.rootSubscription = undefined;
	}

	hasSubscribers(): boolean {
		return this.hasSubscriptions();
	}

	hasSubscriptions(): boolean {
		return this.rootSubscription !== undefined;
	}

	// Dispatch the event without caring about the return values
	send(param: Param) {
		const {rootSubscription} = this;
		if (rootSubscription === undefined) {
			return;
		}

		rootSubscription(param);

		for (const callback of this.subscriptions) {
			callback(param);
		}
	}

	callSync(param: Param): Ret {
		try {
			const {rootSubscription, subscriptions} = this;
			if (rootSubscription === undefined) {
				throw new Error(`No subscription for event ${this.name}`);
			}

			const ret = noPromise(rootSubscription(param));
			for (const callback of subscriptions) {
				noPromise(callback(param));
			}
			return ret;
		} catch (err) {
			this.onError(err);
			throw err;
		}
	}

	async call(param: Param): Promise<Ret> {
		const {rootSubscription, subscriptions} = this;
		if (rootSubscription === undefined) {
			throw new Error(`No subscription for event ${this.name}`);
		}

		try {
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
		} catch (err) {
			this.onError(err);
			throw err;
		}
	}

	wait(val: Ret, timeout?: number): Promise<Param> {
		return new Promise((resolve, reject) => {
			let timeoutId: undefined | NodeJS.Timeout;
			let timedOut = false;

			if (timeout !== undefined) {
				timeoutId = setTimeout(
					() => {
						timedOut = true;
						listener.unsubscribe();
						reject(
							new Error(`Timed out after waiting ${timeout}ms for ${this.name}`),
						);
					},
					timeout,
				);
			}

			const listener = this.subscribe((param) => {
				if (timedOut) {
					return val;
				}

				if (timeoutId !== undefined) {
					clearTimeout(timeoutId);
				}

				listener.unsubscribe();
				resolve(param);
				return val;
			});
		});
	}

	async callOptional(param: Param): Promise<undefined | Ret> {
		if (this.rootSubscription === undefined) {
			return undefined;
		} else {
			return this.call(param);
		}
	}

	subscribe(
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

	unsubscribe(callback: Callback<Param, Ret>) {
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
