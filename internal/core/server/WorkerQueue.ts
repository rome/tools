/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {WorkerContainer} from "./WorkerManager";
import Server from "./Server";
import {AbsoluteFilePath} from "@internal/path";
import createDeferredPromise from "@internal/core/common/utils/createDeferredPromise";
import {VoidCallback} from "@internal/typescript-helpers";
import {FileNotFound} from "@internal/fs/FileNotFound";

type Queue<M> = Array<[AbsoluteFilePath, M, VoidCallback]>;

type WorkerQueueItem<M> = {
	running: boolean;
	queue: Queue<M>;
};

type Callback<M> = (
	path: AbsoluteFilePath,
	metadata: M,
) => undefined | Promise<void>;

export default class WorkerQueue<M> {
	constructor(server: Server, maxPer: number = 2) {
		this.server = server;
		this.callbacks = [];
		this.runningWorkers = [];
		this.workers = new Map();
		this.open = true;
		this.maxPer = maxPer;
	}

	private server: Server;
	private maxPer: number;
	private runningWorkers: Array<Promise<void>>;
	private callbacks: Array<Callback<M>>;
	private workers: Map<WorkerContainer, WorkerQueueItem<M>>;
	private open: boolean;

	// Prematurely fetch the owners so we don't waterfall worker creation
	public async prepare(paths: Iterable<AbsoluteFilePath>) {
		await Promise.all(
			Array.from(
				paths,
				async (path) => {
					return FileNotFound.allowMissing(
						path,
						() => this.server.fileAllocator.getOrAssignOwner(path),
					);
				},
			),
		);
	}

	public async pushQueue(
		path: AbsoluteFilePath,
		metadata: M,
		wait: boolean = false,
	) {
		if (!this.open) {
			throw new Error("WorkerQueue has already closed");
		}

		if (this.callbacks.length === 0) {
			throw new Error("No callbacks attached to queue");
		}

		const workerContainer = await this.server.fileAllocator.getOrAssignOwner(
			path,
		);

		const {resolve, promise} = createDeferredPromise<void>();

		// Populate the worker queue for this item
		let worker = this.workers.get(workerContainer);
		if (worker === undefined) {
			worker = {
				running: false,
				queue: [],
			};
			this.workers.set(workerContainer, worker);
		}
		worker.queue.push([path, metadata, resolve]);

		// Start this worker if it isn't already
		if (!worker.running) {
			const promise = this.processWorker(worker);
			// Add a `catch` so that we aren't considered an unhandled promise if it rejects before a handler is attached
			promise.catch(() => {});
			this.runningWorkers.push(promise);
		}

		// If requested, wait on this queue item to finish
		if (wait) {
			await promise;
		}
	}

	public addCallback(callback: Callback<M>) {
		this.callbacks.push(callback);
	}

	private async processWorker(worker: WorkerQueueItem<M>) {
		worker.running = true;

		const {queue} = worker;

		const next = async () => {
			const item = queue.shift();
			if (item === undefined) {
				// Exhausted queue
				return;
			}

			const [filename, metadata, resolve] = item;
			for (const callback of this.callbacks) {
				await callback(filename, metadata);
			}
			resolve();
			await next();
		};

		while (queue.length > 0) {
			// "threads"
			const threads = [];
			for (let i = 0; i < this.maxPer; i++) {
				threads.push(next());
			}
			await Promise.all(threads);
		}

		worker.running = false;
	}

	public async spin() {
		while (
			// Keep consuming all the promises until we're exhausted
			this.runningWorkers.length >
			0
		) {
			const {runningWorkers} = this;
			this.runningWorkers = [];
			await Promise.all(runningWorkers);
		}

		// Ensure we never receive anymore queue items

		this.open = false;
	}
}
