/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Server} from "@romefrontend/core";
import {Stats} from "./MemoryFileSystem";
import {WorkerContainer} from "../WorkerManager";
import {FilePathLocker} from "../../common/utils/lockers";
import {AbsoluteFilePath, AbsoluteFilePathMap} from "@romefrontend/path";

export default class FileAllocator {
	constructor(server: Server) {
		this.server = server;
		this.fileToWorker = new AbsoluteFilePathMap();
		this.locker = new FilePathLocker();
	}

	server: Server;
	locker: FilePathLocker;
	fileToWorker: AbsoluteFilePathMap<number>;

	init() {
		this.server.memoryFs.deletedFileEvent.subscribe((path) => {
			return this.handleDeleted(path);
		});

		this.server.memoryFs.changedFileEvent.subscribe(({path, oldStats, newStats}) => {
			return this.handleChange(path, oldStats, newStats);
		});
	}

	getAllOwnedFilenames(): Array<AbsoluteFilePath> {
		return Array.from(this.fileToWorker.keys());
	}

	hasOwner(path: AbsoluteFilePath): boolean {
		return this.getOwnerId(path) !== undefined;
	}

	getOwnerId(path: AbsoluteFilePath): undefined | number {
		return this.fileToWorker.get(path);
	}

	verifySize(path: AbsoluteFilePath, stats: Stats) {
		const project = this.server.projectManager.assertProjectExisting(path);
		const maxSize = project.config.files.maxSize;

		if (stats.size > maxSize) {
			throw new Error(
				`The file ${path.join()} exceeds the project config max size of ${maxSize} bytes`,
			);
		}
	}

	getOwnerAssert(path: AbsoluteFilePath): WorkerContainer {
		const {workerManager} = this.server;
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
		const {workerManager} = this.server;

		const workerId = this.getOwnerId(path);
		if (workerId === undefined) {
			return this.assignOwner(path);
		} else {
			await workerManager.locker.waitLock(workerId);
			return workerManager.getWorkerAssert(workerId);
		}
	}

	async evict(path: AbsoluteFilePath) {
		// Find owner
		const workerId = this.getOwnerId(path);
		if (workerId === undefined) {
			return;
		}

		// Notify the worker to remove it from 'it's cache
		const filename = path.join();
		const worker = this.server.workerManager.getWorkerAssert(workerId);
		await worker.bridge.evict.call({
			filename,
		});

		this.server.logger.info("[FileAllocator] Evicted %s", path.toMarkup());
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
		this.fileToWorker.delete(path);

		// Remove the total size from 'this worker so it'll be assigned next
		const stats = this.server.memoryFs.getFileStatsAssert(path);
		this.server.workerManager.disown(workerId, stats);
	}

	async handleChange(
		path: AbsoluteFilePath,
		oldStats: undefined | Stats,
		newStats: Stats,
	) {
		const {logger, workerManager} = this.server;

		// Send update to worker owner
		if (this.hasOwner(path)) {
			// Get the worker
			const workerId = this.getOwnerId(path);
			if (workerId === undefined) {
				throw new Error(`Expected worker id for ${path.join()}`);
			}

			// Evict the file from 'cache
			await this.evict(path);

			// Verify that this file doesn't exceed any size limit
			this.verifySize(path, newStats);

			// Add on the new size, and remove the old
			if (oldStats === undefined) {
				throw new Error(
					"File already has an owner so expected to have old stats but had none",
				);
			}
			workerManager.disown(workerId, oldStats);
			workerManager.own(workerId, newStats);
		} else if (await this.server.projectManager.maybeEvictPossibleConfig(path)) {
			logger.info(
				"[FileAllocator] Evicted the project belonging to config %s",
				path.toMarkup(),
			);
		} else {
			logger.info("[FileAllocator] No owner for eviction %s", path.toMarkup());
		}
	}

	async assignOwner(path: AbsoluteFilePath): Promise<WorkerContainer> {
		const {workerManager, logger} = this.server;

		const lock = await this.locker.getLock(path);

		// We may have waited on the lock and could already have an owner
		if (this.hasOwner(path)) {
			lock.release();
			return this.getOwnerAssert(path);
		}
		try {
			const worker = await workerManager.getNextWorker(path);

			// Add ourselves to the file map
			logger.info(
				"[FileAllocator] File %s assigned to worker %s",
				path.toMarkup(),
				worker.id,
			);
			this.fileToWorker.set(path, worker.id);

			return worker;
		} finally {
			// Release and continue
			lock.release();
		}
	}
}
