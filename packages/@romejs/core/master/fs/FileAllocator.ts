/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Master} from '@romejs/core';
import {Stats} from './MemoryFileSystem';
import {WorkerContainer} from '../WorkerManager';
import Locker from '../../common/utils/Locker';
import {AbsoluteFilePath, AbsoluteFilePathSet} from '@romejs/path';

export default class FileAllocator {
  constructor(master: Master) {
    this.master = master;
    this.fileToWorker = new Map();
    this.locker = new Locker();
  }

  master: Master;
  locker: Locker<string>;
  fileToWorker: Map<string, number>;

  init() {
    this.master.memoryFs.deletedFileEvent.subscribe(path => {
      return this.handleDeleted(path);
    });

    this.master.memoryFs.changedFileEvent.subscribe(
      ({path, oldStats, newStats}) => {
        return this.handleChange(path, oldStats, newStats);
      },
    );
  }

  getAllOwnedFilenames(): Array<string> {
    return Array.from(this.fileToWorker.keys());
  }

  hasOwner(path: AbsoluteFilePath): boolean {
    return this.getOwnerId(path) !== undefined;
  }

  getOwnerId(path: AbsoluteFilePath): undefined | number {
    return this.fileToWorker.get(path.join());
  }

  verifySize(path: AbsoluteFilePath, stats: Stats) {
    const project = this.master.projectManager.assertProjectExisting(path);
    const maxSize = project.config.files.maxSize;

    if (stats.size > maxSize) {
      throw new Error(
        `The file ${path.join()} exceeds the project config max size of ${maxSize} bytes`,
      );
    }
  }

  getOwnerAssert(path: AbsoluteFilePath): WorkerContainer {
    const {workerManager} = this.master;
    const workerId = this.getOwnerId(path);
    if (workerId === undefined) {
      throw new Error(`No worker found for ${path}`);
    }

    const worker = workerManager.getWorkerAssert(workerId);
    if (!worker.ready) {
      throw new Error(`Worker ${workerId} isn't ready`);
    }
    return worker;
  }

  async getOrAssignOwner(path: AbsoluteFilePath): Promise<WorkerContainer> {
    const {workerManager} = this.master;

    const workerId = this.getOwnerId(path);
    if (workerId === undefined) {
      return this.assignOwner(path);
    } else {
      await workerManager.locker.waitLock(workerId);
      return workerManager.getWorkerAssert(workerId);
    }
  }

  async groupPathsByWorker(
    paths: AbsoluteFilePathSet | Array<AbsoluteFilePath>,
  ): Promise<Array<Array<AbsoluteFilePath>>> {
    const pathsByWorker: Map<number, Array<AbsoluteFilePath>> = new Map();

    // Populate our queues
    await Promise.all(
      Array.from(paths, async path => {
        const worker = await this.getOrAssignOwner(path);

        let queue = pathsByWorker.get(worker.id);
        if (queue === undefined) {
          queue = [];
          pathsByWorker.set(worker.id, queue);
        }
        queue.push(path);
      }),
    );

    return Array.from(pathsByWorker.values());
  }

  async evict(path: AbsoluteFilePath) {
    // Find owner
    const workerId = this.getOwnerId(path);
    if (workerId === undefined) {
      return;
    }

    // Notify the worker to remove it from 'it's cache
    const filename = path.join();
    const worker = this.master.workerManager.getWorkerAssert(workerId);
    await worker.bridge.evict.call({
      filename,
    });

    this.master.logger.info(`[FileAllocator] Evicted %s`, filename);
  }

  async handleDeleted(path: AbsoluteFilePath) {
    // Find owner
    const workerId = this.getOwnerId(path);
    if (workerId === undefined) {
      return;
    }

    // Evict file from 'worker cache
    await this.evict(path);

    // Disown it from 'our internal map
    this.fileToWorker.delete(path.join());

    // Remove the total size from 'this worker so it'll be assigned next
    const stats = this.master.memoryFs.getFileStatsAssert(path);
    this.master.workerManager.disown(workerId, stats);
  }

  async handleChange(
    path: AbsoluteFilePath,
    oldStats: undefined | Stats,
    newStats: Stats,
  ) {
    const filename = path.join();
    const {logger, workerManager} = this.master;

    // Send update to worker owner
    if (this.hasOwner(path)) {
      // Get the worker
      const workerId = this.getOwnerId(path);
      if (workerId === undefined) {
        throw new Error(`Expected worker id for ${filename}`);
      }

      // Evict the file from 'cache
      await this.evict(path);

      // Verify that this file doesn't exceed any size limit
      this.verifySize(path, newStats);

      // Add on the new size, and remove the old
      if (oldStats === undefined) {
        throw new Error(
          'File already has an owner so expected to have old stats but had none',
        );
      }
      workerManager.disown(workerId, oldStats);
      workerManager.own(workerId, newStats);
    } else if (
      await this.master.projectManager.maybeEvictPossibleConfig(path)
    ) {
      logger.info(
        `[FileAllocator] Evicted the project belonging to config %s`,
        filename,
      );
    } else {
      logger.info(`[FileAllocator] No owner for eviction %s`, filename);
    }
  }

  async assignOwner(path: AbsoluteFilePath): Promise<WorkerContainer> {
    const {workerManager, logger} = this.master;

    const filename = path.join();
    const lock = await this.locker.getLock(filename);

    // We may have waited on the lock and could already have an owner
    if (this.hasOwner(path)) {
      lock.release();
      return this.getOwnerAssert(path);
    }

    const worker = await workerManager.getNextWorker(path);

    // Add ourselves to the file map
    logger.info(
      `[FileAllocator] File %s assigned to worker %s`,
      filename,
      worker.id,
    );
    this.fileToWorker.set(filename, worker.id);

    // Release and continue
    lock.release();

    return worker;
  }
}
