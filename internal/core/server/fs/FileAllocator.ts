/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Server} from "@internal/core";
import {ChangedFileEventItem, Stats} from "./MemoryFileSystem";
import {WorkerContainer} from "../WorkerManager";
import {FilePathLocker} from "../../../async/lockers";
import {AbsoluteFilePath, AbsoluteFilePathMap} from "@internal/path";
import {AnyMarkup, concatMarkup, markup} from "@internal/markup";
import {ReporterNamespace} from "@internal/cli-reporter";

export default class FileAllocator {
	constructor(server: Server) {
		this.server = server;
		this.fileToWorker = new AbsoluteFilePathMap();
		this.locker = new FilePathLocker();
		this.logger = server.logger.namespace(markup`[FileAllocator]`);
	}

	private server: Server;
	private locker: FilePathLocker;
	private fileToWorker: AbsoluteFilePathMap<number>;
	private logger: ReporterNamespace;

	public init() {
		this.server.memoryFs.deletedFileEvent.subscribe((paths) => {
			return this.handleDeleted(paths);
		});

		this.server.memoryFs.changedFileEvent.subscribe((events) => {
			return this.handleChange(events);
		});
	}

	public getAllOwnedFilenames(): Array<AbsoluteFilePath> {
		return Array.from(this.fileToWorker.keys());
	}

	private hasOwner(path: AbsoluteFilePath): boolean {
		return this.getOwnerId(path) !== undefined;
	}

	private getOwnerId(path: AbsoluteFilePath): undefined | number {
		return this.fileToWorker.get(path);
	}

	public verifySize(path: AbsoluteFilePath, stats: Stats) {
		const project = this.server.projectManager.findLoadedProject(path);
		if (project === undefined) {
			return;
		}

		const maxSize = project.config.files.maxSize;
		if (stats.size > maxSize) {
			throw new Error(
				`The file ${path.join()} exceeds the project config max size of ${maxSize} bytes`,
			);
		}
	}

	private getOwnerAssert(path: AbsoluteFilePath): WorkerContainer {
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

	public async getOrAssignOwner(
		path: AbsoluteFilePath,
	): Promise<WorkerContainer> {
		const {workerManager} = this.server;

		const workerId = this.getOwnerId(path);
		if (workerId === undefined) {
			return this.assignOwner(path);
		} else {
			await workerManager.locker.waitLock(workerId);
			return workerManager.getWorkerAssert(workerId);
		}
	}

	public async evict(path: AbsoluteFilePath, reason: AnyMarkup) {
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

		this.logger.info(
			markup`Evicted <emphasis>${path}</emphasis> due to <emphasis>${reason}</emphasis>`,
		);
	}

	private async handleDeleted(paths: Array<AbsoluteFilePath>) {
		for (const path of paths) {
			// Find owner
			const workerId = this.getOwnerId(path);
			if (workerId === undefined) {
				continue;
			}

			// Evict file from 'worker cache
			await this.evict(path, markup`file deleted`);

			// Disown it from 'our internal map
			this.fileToWorker.delete(path);

			// Remove the total size from 'this worker so it'll be assigned next
			const stats = this.server.memoryFs.getFileStatsAssert(path);
			this.server.workerManager.disown(workerId, stats);
		}
	}

	private async handleChange(events: Array<ChangedFileEventItem>) {
		const {workerManager} = this.server;

		for (const {path, oldStats, newStats} of events) {
			// Send update to worker owner
			if (this.hasOwner(path)) {
				// Get the worker
				const workerId = this.getOwnerId(path);
				if (workerId === undefined) {
					throw new Error(`Expected worker id for ${path.join()}`);
				}

				// Evict the file from cache
				await this.evict(path, markup`file change`);

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
			} else {
				this.logger.info(markup`No owner for eviction ${path}`);
			}
		}

		const paths = events.map((event) => event.path);
		if (await this.server.projectManager.maybeEvictProjects(paths)) {
			const displayPaths = concatMarkup(
				paths.map((path) => markup`<emphasis>${path}</emphasis>`),
				markup`, `,
			);
			this.logger.info(
				markup`Evicted projects belonging to dependencies ${displayPaths}`,
			);
		}
	}

	private async assignOwner(path: AbsoluteFilePath): Promise<WorkerContainer> {
		const {workerManager, memoryFs} = this.server;

		const lock = await this.locker.getLock(path);

		// We may have waited on the lock and could already have an owner
		if (this.hasOwner(path)) {
			lock.release();
			return this.getOwnerAssert(path);
		}

		try {
			const worker = await workerManager.getNextWorker(path);

			// Add ourselves to the file map
			this.logger.info(
				markup`File <emphasis>${path}</emphasis> (file size <filesize>${String(
					memoryFs.getFileStats(path)?.size,
				)}</filesize>) assigned to worker ${worker.id}`,
			);
			this.fileToWorker.set(path, worker.id);

			return worker;
		} finally {
			// Release and continue
			lock.release();
		}
	}
}
