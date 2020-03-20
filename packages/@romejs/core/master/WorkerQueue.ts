/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {WorkerContainer} from './WorkerManager';
import Master from './Master';
import {AbsoluteFilePath} from '@romejs/path';

type Queue<M> = Array<[AbsoluteFilePath, M]>;

type WorkerQueueItem<M> = {
  running: boolean;
  queue: Queue<M>;
};

type Callback<M> = (filename: AbsoluteFilePath, metadata: M) =>
    | undefined
    | Promise<void>;

export default class WorkerQueue<M> {
  constructor(master: Master) {
    this.master = master;
    this.queue = [];
    this.callbacks = [];
    this.promises = [];
    this.workers = new Map();
    this.open = true;
  }

  master: Master;
  queue: Queue<M>;
  promises: Array<Promise<void>>;
  callbacks: Array<Callback<M>>;
  workers: Map<WorkerContainer, WorkerQueueItem<M>>;
  open: boolean;

  pushQueue(filename: AbsoluteFilePath, metadata: M) {
    if (!this.open) {
      throw new Error('WorkerQueue has already closed');
    }

    this.queue.push([filename, metadata]);
  }

  addCallback(callback: Callback<M>) {
    this.callbacks.push(callback);
  }

  // Take all the root queue items, assign them to a worker, and start the worker queue if it's not running
  async updateWorkerQueues() {
    const {queue} = this;

    while (queue.length > 0) {
      const item = queue.shift();
      if (item === undefined) {
        throw new Error('Already validated queue.length above');
      }

      const path = item[0];
      const workerContainer = await this.master.fileAllocator.getOrAssignOwner(
        path,
      );

      // Populate the worker queue for this item
      let worker = this.workers.get(workerContainer);
      if (worker === undefined) {
        worker = {
          running: false,
          queue: [],
        };
        this.workers.set(workerContainer, worker);
      }
      worker.queue.push(item);

      // Start this worker if it isn't already
      if (worker.running === false) {
        const promise = this.processWorker(worker);
        // Add a `catch` so that we aren't considered an unhandled promise if it rejects before a handler is attached
        promise.catch(() => {});
        this.promises.push(promise);
      }
    }
  }

  async processWorker(item: WorkerQueueItem<M>) {
    item.running = true;

    const {queue} = item;

    while (queue.length > 0) {
      const item = queue.shift();
      if (item === undefined) {
        throw new Error('Already validated queue.length above');
      }

      const [filename, metadata] = item;
      for (const callback of this.callbacks) {
        await callback(filename, metadata);
      }
      await this.updateWorkerQueues();
    }

    item.running = false;
  }

  async spin() {
    const {queue} = this;

    // Create the initial queue
    await this.updateWorkerQueues();

    // Keep consuming all the promises until we're exhausted
    while (this.promises.length > 0) {
      const {promises} = this;
      this.promises = [];
      await Promise.all(promises);
    }

    // Ensure we never receive anymore queue items
    this.open = false;

    // Ensure main queue has been drained
    if (queue.length > 0) {
      throw new Error('Expected no queue items to remain');
    }

    // Ensure worker queues have been drained
    for (const [worker, {queue}] of this.workers) {
      if (queue.length > 0) {
        throw new Error(
          `Expected no queue items to remain for worker ${worker.id}`,
        );
      }
    }
  }
}
