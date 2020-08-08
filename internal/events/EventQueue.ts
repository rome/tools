import {EventSubscription} from "@internal/events/types";
import {AsyncVoidCallback, VoidCallback} from "@internal/typescript-helpers";
import createDeferredPromise from "@internal/core/common/utils/createDeferredPromise";

type EventQueueCallack<Value> = AsyncVoidCallback<[Array<Value>]>;

export default class EventQueue<Value> {
	constructor(debounce: number = 100) {
		this.subscriptions = new Set();
		this.queue = [];
		this.queueKeys = new Set();
		this.timeout = undefined;
		this.debounce = debounce;
	}

	subscriptions: Set<EventQueueCallack<Value>>;
	timeout: undefined | NodeJS.Timeout;
	queueKeys: Set<string>;
	queue: Array<{
		resolve: VoidCallback;
		value: Value;
	}>;
	debounce: number;

	async flush() {
		if (this.timeout !== undefined) {
			clearTimeout(this.timeout);
			this.timeout = undefined;
		}

		const queue = this.queue;
		this.queue = [];
		this.queueKeys = new Set();

		const queueValues = queue.map((item) => item.value);

		for (const callback of this.subscriptions) {
			await callback(queueValues);
		}

		for (const {resolve} of queue) {
			resolve();
		}
	}

	async push(value: Value, key?: string) {
		if (key !== undefined && this.queueKeys.has(key)) {
			return;
		}

		const {resolve, promise} = createDeferredPromise();

		this.queue.push({resolve, value});
		if (key !== undefined) {
			this.queueKeys.add(key);
		}

		if (this.timeout === undefined) {
			this.timeout = setTimeout(() => this.flush(), this.debounce);
		}

		await promise;
	}

	subscribe(callback: EventQueueCallack<Value>): EventSubscription {
		this.subscriptions.add(callback);

		return {
			unsubscribe: async () => {
				await this.flush();
				this.subscriptions.delete(callback);
			},
		};
	}
}
