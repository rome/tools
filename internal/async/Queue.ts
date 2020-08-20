/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import createDeferredPromise from "./createDeferredPromise";
import {AsyncVoidCallback, VoidCallback} from "@internal/typescript-helpers";
import {ExtendedMap} from "@internal/collections";

type QueueThread<Metadata> = {
	running: boolean;
	items: Array<[Metadata, VoidCallback]>;
};

export type QueueOptions<Metadata, Thread> = {
	maxThreads?: number;
	maxPerThread?: number;
	callback: AsyncVoidCallback<[Metadata, Thread]>;
};

export default class Queue<Metadata, Thread> {
	constructor(
		{maxThreads = Infinity, maxPerThread = 2, callback}: QueueOptions<
			Metadata,
			Thread
		>,
	) {
		this.runningThreads = [];
		this.waitingThreads = [];

		this.threads = new ExtendedMap(
			"threads",
			() => ({
				running: false,
				items: [],
			}),
		);

		this.locked = false;
		this.paused = false;

		this.callback = callback;
		this.maxThreads = maxThreads;
		this.maxPerThread = maxPerThread;
	}

	private maxPerThread: number;
	private maxThreads: number;

	private runningThreads: Array<Promise<void>>;
	private waitingThreads: Array<Thread>;
	private callback: AsyncVoidCallback<[Metadata, Thread]>;
	private threads: ExtendedMap<Thread, QueueThread<Metadata>>;
	private locked: boolean;
	private paused: boolean;

	public async pushQueue(
		thread: Thread,
		metadata: Metadata,
		wait: boolean = false,
	) {
		if (this.locked) {
			throw new Error("Queue is locked and no longer accepts items");
		}

		const {resolve, promise} = createDeferredPromise<void>();

		// Populate the worker queue for this item
		const queue = this.threads.assert(thread);
		queue.items.push([metadata, resolve]);

		if (!queue.running && !this.paused) {
			this.startThread(thread, queue);
		}

		// If requested, wait on this queue item to finish
		if (wait) {
			await promise;
		}
	}

	private startThread(thread: Thread, queue: QueueThread<Metadata>) {
		// Start this thread if it isn't already
		if (this.runningThreads.length < this.maxThreads) {
			const promise = this.processThread(thread, queue);
			this.runningThreads.push(promise);

			// Add a `catch` so that we aren't considered an unhandled promise if it rejects before a handler is attached
			promise.catch(() => {});
		} else {
			// Otherwise when another thread has finished, we'll start
			this.waitingThreads.push(thread);
		}
	}

	private async processThread(thread: Thread, queue: QueueThread<Metadata>) {
		queue.running = true;

		const {items} = queue;

		const next = async () => {
			if (this.paused) {
				return;
			}

			const item = items.shift();
			if (item === undefined) {
				// Exhausted queue
				return;
			}

			const [metadata, resolve] = item;
			await this.callback(metadata, thread);
			resolve();
			await next();
		};

		const threads = [];
		for (let i = 0; i < this.maxPerThread; i++) {
			threads.push(next());
		}
		await Promise.all(threads);

		queue.running = false;

		if (this.waitingThreads.length > 0 && !this.paused) {
			const nextThread = this.waitingThreads.shift()!;
			const nextQueue = this.threads.get(nextThread)!;
			await this.processThread(nextThread, nextQueue);
		}
	}

	public async pause() {
		// Build promises of work that went complete, will indicate a completely paused queue
		const promises = [];
		for (const queue of this.threads.values()) {
			for (const item of queue.items) {
				promises.push(item[1]);
			}
		}
		this.paused = true;
		this.waitingThreads = [];
		await Promise.all(promises);
	}

	public resume() {
		this.paused = false;

		// Restart all threads
		for (const [thread, queue] of this.threads) {
			if (queue.items.length > 0) {
				this.startThread(thread, queue);
			}
		}
	}

	public lock() {
		this.locked = true;
	}

	public async spin() {
		while (
			// Keep consuming all the promises until we're exhausted
			this.runningThreads.length >
			0
		) {
			const {runningThreads} = this;
			this.runningThreads = [];
			await Promise.all(runningThreads);
		}
	}
}
