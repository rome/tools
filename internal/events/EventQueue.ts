import {AsyncVoidCallback, VoidCallback} from "@internal/typescript-helpers";
import createDeferredPromise from "@internal/async/createDeferredPromise";
import {GlobalLock} from "@internal/async";
import {Resource, createResourceFromCallback} from "@internal/resources";

type EventQueueCallback<Value> = AsyncVoidCallback<[Value[]]>;

type EventQueueItem<Value> = {
	callback: EventQueueCallback<Value>;
	resource: Resource;
};

type ToDedupeKey<Value> = (value: Value) => unknown;

type EventQueueOptions<Value> = {
	debounce?: number;
	toDedupeKey?: ToDedupeKey<Value>;
};

export default class EventQueue<Value> {
	constructor(
		name: string,
		{debounce = 100, toDedupeKey}: EventQueueOptions<Value>,
	) {
		this.name = name;
		this.subscriptions = new Set();
		this.queue = new Map();
		this.timeout = undefined;
		this.debounce = debounce;
		this.lock = new GlobalLock();

		this.toDedupeKey = toDedupeKey;
	}

	public lock: GlobalLock;

	private name: string;
	private subscriptions: Set<EventQueueItem<Value>>;
	private timeout: undefined | [VoidCallback, NodeJS.Timeout];
	private queue: Map<
		unknown,
		{
			promise: Promise<void>;
			resolve: VoidCallback;
			value: Value;
		}
	>;
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

			for (const {callback} of this.subscriptions) {
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
			this.queue.set(
				key,
				{
					...existing,
					value,
				},
			);
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

	public subscribe(callback: EventQueueCallback<Value>): Resource {
		const resource = createResourceFromCallback(
			`EventQueueSubscription<${this.name}>`,
			async () => {
				await this.flush();
				this.subscriptions.delete(item);
			},
		);

		const item: EventQueueItem<Value> = {resource, callback};
		this.subscriptions.add(item);
		return resource;
	}
}
