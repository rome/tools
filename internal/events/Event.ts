/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Duration} from "@internal/numbers";
import {Resource, createResourceFromCallback} from "@internal/resources";
import {EventCallback, EventOptions, EventSubscriptionOptions} from "./types";

type EventSubscription<Param, Ret> = {
	callback: EventCallback<Param, Ret>;
	resource: Resource;
};

export default class Event<Param, Ret = void> {
	constructor(name: string, opts: EventOptions = {}) {
		this.subscriptions = new Set();
		this.rootSubscription = undefined;
		this.name = name;
		this.displayName = opts.displayName ?? `event ${this.name}`;
		this.options = opts;
	}

	public name: string;
	public displayName: string;

	private options: EventOptions;
	private rootSubscription: undefined | EventSubscription<Param, Ret>;
	private subscriptions: Set<EventSubscription<Param, Ret>>;

	private async callSubscription(
		sub: EventSubscription<Param, Ret>,
		param: Param,
	): Promise<Ret> {
		return sub.callback(param, sub.resource);
	}

	private onSubscriptionChange() {
		const {onSubscriptionChange} = this.options;

		onSubscriptionChange?.();
	}

	public async clear(): Promise<void> {
		// Release all subscriptions
		const promises = Array.from(
			this.subscriptions,
			(sub) => sub.resource.release(),
		);
		if (this.rootSubscription !== undefined) {
			promises.push(this.rootSubscription.resource.release());
		}
		await Promise.all(promises);
		this.subscriptions.clear();
		this.rootSubscription = undefined;
	}

	public hasSubscriptions(): boolean {
		return this.rootSubscription !== undefined;
	}

	// Dispatch the event without caring about the return values
	public send(param: Param, required: boolean = false) {
		const {rootSubscription} = this;
		if (rootSubscription === undefined) {
			if (required) {
				throw new Error(`Event.send: No subscriptions for ${this.displayName}`);
			}
			return;
		}

		this.callSubscription(rootSubscription, param);

		for (const sub of this.subscriptions) {
			this.callSubscription(sub, param);
		}
	}

	public async call(param: Param): Promise<Ret> {
		const {rootSubscription, subscriptions} = this;
		if (rootSubscription === undefined) {
			throw new Error(`Event.call: No subscriptions for ${this.displayName}`);
		}

		if (this.options.serial === true) {
			const ret = await this.callSubscription(rootSubscription, param);
			for (const sub of subscriptions) {
				await this.callSubscription(sub, param);
			}
			return ret;
		} else {
			const res = await Promise.all([
				this.callSubscription(rootSubscription, param),
				...Array.from(subscriptions, (sub) => this.callSubscription(sub, param)),
			]);

			// Return the root subscription value
			return res[0];
		}
	}

	public wait(val: Ret, timeout?: Duration): Promise<Param> {
		return new Promise((resolve, reject) => {
			const resource = this.subscribe(async (param, resource) => {
				resolve(param);
				await resource.release();
				return val;
			});

			if (timeout !== undefined) {
				resource.add(
					timeout.setTimeout(() => {
						resource.release().then(() => {
							reject(
								new Error(
									`Timed out after waiting ${timeout.format()} for ${this.displayName}`,
								),
							);
						}).catch((err) => {
							reject(err);
						});
					}),
				);
			}
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
		callback: EventCallback<Param, Ret>,
		opts: EventSubscriptionOptions = {},
	): Resource {
		opts;

		if (this.options.unique === true && this.subscriptions.size !== 0) {
			throw new Error(
				`Only allowed a single subscription for ${this.displayName}`,
			);
		}

		const resource = createResourceFromCallback(
			`EventSubscription<${this.name}>`,
			() => {
				this.unsubscribe(sub);
			},
			{optional: this.options.requiredSubscriptionResource !== true},
		);

		const sub: EventSubscription<Param, Ret> = {
			callback,
			resource,
		};

		if (this.rootSubscription === undefined) {
			this.rootSubscription = sub;
		} else {
			this.subscriptions.add(sub);
		}

		this.onSubscriptionChange();

		return resource;
	}

	private unsubscribe(sub: EventSubscription<Param, Ret>): void {
		if (this.subscriptions.has(sub)) {
			this.subscriptions.delete(sub);
			this.onSubscriptionChange();
			return;
		}

		// If this subscription is root, set it to the next one
		if (this.rootSubscription === sub) {
			this.rootSubscription = Array.from(this.subscriptions)[0];
			this.subscriptions.delete(this.rootSubscription);
			this.onSubscriptionChange();
			return;
		}

		throw new Error("Unhandled subscription");
	}
}
