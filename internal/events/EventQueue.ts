import {EventSubscription} from "@internal/events/types";
import {AsyncVoidCallback, VoidCallback} from "@internal/typescript-helpers";
import createDeferredPromise from "@internal/async/createDeferredPromise";
import {GlobalLock} from "@internal/async";
import { createEventSubscription } from "./utils";

type EventQueueCallack<Value> = AsyncVoidCallback<[Value[]]>;

type ToDedupeKey<Value> = (value: Value) => unknown;

type EventQueueOptions<Value> = {
	debounce?: number;
	toDedupeKey?: ToDedupeKey<Value>;
}

export default class EventQueue<Value> {
	constructor({debounce = 100, toDedupeKey}: EventQueueOptions<Value>) {
		this.subscriptions = new Set();
		this.queue = new Map();
		this.timeout = undefined;
		this.debounce = debounce;
		this.lock = new GlobalLock();

		this.toDedupeKey = toDedupeKey;
	}

	public lock: GlobalLock;

	private subscriptions: Set<EventQueueCallack<Value>>;
	private timeout: undefined | [VoidCallback, NodeJS.Timeout];
	private queue: Map<unknown, {
		promise: Promise<void>;
		resolve: VoidCallback;
		value: Value;
	}>;
	private debounce: number;
	private toDedupeKey: undefined | ToDedupeKey<Value>;

	public hasDebounce(): boolean {
		return this.timeout !== undefined;
	}

	public async flush() {
		const queue = this.queue;
		this.queue = new Map();

		const queueValues = Array.from(queue.values(), (item) => item.value);

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

			for (const {resolve} of queue.values()) {
				resolve();
			}
		});
	}

	public async push(value: Value): Promise<void> {
		const key = this.toDedupeKey === undefined ? value : this.toDedupeKey(value);
		const existing = this.queue.get(key);

		// Take over the queued value
		if (existing !== undefined) {
			this.queue.set(key, {
				...existing,
				value,
			});
			return existing.promise;
		}

		const {resolve, promise} = createDeferredPromise();

		this.queue.set(key, {promise, resolve, value});

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

		return createEventSubscription({
			unsubscribe: async () => {
				if (!this.subscriptions.has(callback)) {
					return false;
				}

				await this.flush();
				this.subscriptions.delete(callback);
				return true;
			},
		});
	}
}
