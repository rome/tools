import {EventSubscription} from "@internal/events/types";
import {AsyncVoidCallback, VoidCallback} from "@internal/typescript-helpers";
import createDeferredPromise from "@internal/async/createDeferredPromise";
import {GlobalLock} from "@internal/async";

type EventQueueCallack<Value> = AsyncVoidCallback<[Array<Value>]>;

export default class EventQueue<Value> {
	constructor(debounce: number = 100) {
		this.subscriptions = new Set();
		this.queue = [];
		this.queueKeys = new Set();
		this.timeout = undefined;
		this.debounce = debounce;
		this.lock = new GlobalLock();
	}

	private subscriptions: Set<EventQueueCallack<Value>>;
	private timeout: undefined | [VoidCallback, NodeJS.Timeout];
	private queueKeys: Set<string>;
	private queue: Array<{
		resolve: VoidCallback;
		value: Value;
	}>;
	private debounce: number;
	public lock: GlobalLock;

	public hasDebounce(): boolean {
		return this.timeout !== undefined;
	}

	public async flush() {
		const queue = this.queue;
		this.queue = [];
		this.queueKeys = new Set();

		const queueValues = queue.map((item) => item.value);

		await this.lock.wrap(async () => {
			if (this.timeout !== undefined) {
				const [callback, timeout] = this.timeout;
				clearTimeout(timeout);
				callback();
				this.timeout = undefined;
			}

			for (const callback of this.subscriptions) {
				await callback(queueValues);
			}

			for (const {resolve} of queue) {
				resolve();
			}
		});
	}

	public async push(value: Value, key?: string) {
		if (key !== undefined && this.queueKeys.has(key)) {
			return;
		}

		const {resolve, promise} = createDeferredPromise();

		this.queue.push({resolve, value});
		if (key !== undefined) {
			this.queueKeys.add(key);
		}

		if (this.timeout === undefined) {
			const timeout = setTimeout(() => this.flush(), this.debounce);
			this.lock.wrap(async () => {
				const {promise, resolve} = createDeferredPromise();
				this.timeout = [resolve, timeout];
				await promise;
			});
		}

		await promise;
	}

	public subscribe(callback: EventQueueCallack<Value>): EventSubscription {
		this.subscriptions.add(callback);

		return {
			unsubscribe: async () => {
				await this.flush();
				this.subscriptions.delete(callback);
			},
		};
	}
}
