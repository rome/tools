/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Server, WorkerContainer} from "@internal/core";
import {SimpleStats} from "./MemoryFileSystem";
import {PathLocker} from "../../../async/lockers";
import {AbsoluteFilePath, AbsoluteFilePathMap} from "@internal/path";
import {Markup, joinMarkup, markup} from "@internal/markup";
import {ReporterNamespace} from "@internal/cli-reporter";
import {
	createSingleDiagnosticsError,
	descriptions,
} from "@internal/diagnostics";
import {matchPathPatterns} from "@internal/path-match";
import {ServerRefreshFile} from "../Server";

export default class FileAllocator {
	constructor(server: Server) {
		this.server = server;
		this.fileToWorker = new AbsoluteFilePathMap();
		this.locker = new PathLocker();
		this.logger = server.logger.namespace(markup`FileAllocator`);
	}

	private server: Server;
	private locker: PathLocker;
	private fileToWorker: AbsoluteFilePathMap<number>;
	private logger: ReporterNamespace;

	public init() {
		this.server.resources.add(
			this.server.refreshFileEvent.subscribe(async (events) => {
				return this.handleRefresh(events);
			}),
		);
	}

	public getAllOwnedFilenames(): AbsoluteFilePath[] {
		return Array.from(this.fileToWorker.keys());
	}

	private hasOwner(path: AbsoluteFilePath): boolean {
		return this.getOwnerId(path) !== undefined;
	}

	private getOwnerId(path: AbsoluteFilePath): undefined | number {
		return this.fileToWorker.get(path);
	}

	public verifySize(path: AbsoluteFilePath, stats: SimpleStats) {
		const project = this.server.projectManager.findLoadedProject(path);
		if (project === undefined) {
			return;
		}

		const maxSize = project.config.files.maxSize;
		if (
			stats.size > maxSize &&
			matchPathPatterns(
				path,
				project.config.files.maxSizeIgnore,
				project.directory,
			).type === "NO_MATCH"
		) {
			throw createSingleDiagnosticsError({
				description: descriptions.FILES.TOO_BIG(
					path,
					project.directory,
					stats.size,
					maxSize,
				),
				location: {
					path,
				},
			});
		}
	}

	public getOwnerAssert(path: AbsoluteFilePath): WorkerContainer {
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

	public async evictAll() {
		const queue = this.server.createWorkerQueue({
			callback: async ({path}) => {
				await this.evict(path, markup`evict all requested`);
			},
		});

		await queue.prepare(this.fileToWorker.keys());
		await queue.spin();
	}

	public async evict(path: AbsoluteFilePath, reason: Markup) {
		// Find owner
		const workerId = this.getOwnerId(path);
		if (workerId === undefined) {
			return;
		}

		// Notify the worker to remove it from it's cache
		// We do not use a FileReference here as the file might not exist
		const uid = this.server.projectManager.getUID(path, true);
		const worker = this.server.workerManager.getWorkerAssert(workerId);
		await worker.bridge.events.evict.call({
			real: path,
			uid,
		});

		this.logger.info(
			markup`Evicted <emphasis>${path}</emphasis> due to <emphasis>${reason}</emphasis>`,
		);
	}

	private async handleDeleted(path: AbsoluteFilePath) {
		// Find owner
		const workerId = this.getOwnerId(path);
		if (workerId === undefined) {
			return;
		}

		// Evict file from 'worker cache
		await this.evict(path, markup`file deleted`);

		// Disown it from 'our internal map
		this.fileToWorker.delete(path);

		// Remove the total size from 'this worker so it'll be assigned next
		const stats = this.server.memoryFs.getFileStatsAssert(path);
		this.server.workerManager.disown(workerId, stats);
	}

	private async handleRefresh(events: ServerRefreshFile[]) {
		const {workerManager} = this.server;

		for (const event of events) {
			const {path} = event;

			// Workers handle cache eviction internally when updating buffers
			if (event.type === "BUFFER_UPDATE" || event.type === "BUFFER_CLEARED") {
				continue;
			}

			if (event.type === "DELETED") {
				await this.handleDeleted(path);
				continue;
			}

			if (event.type === "DISK_UPDATE") {
				if (this.hasOwner(path)) {
					// Get the worker
					const workerId = this.getOwnerId(path);
					if (workerId === undefined) {
						throw new Error(`Expected worker id for ${path.join()}`);
					}

					// Evict the file from cache
					await this.evict(path, markup`file change`);

					const {newStats, oldStats} = event;

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
					this.logger.info(
						markup`No owner for eviction <emphasis>${path}</emphasis>`,
					);
				}
			}
		}

		const paths = events.filter((event) =>
			event.type === "DISK_UPDATE" || event.type === "DELETED"
		).map((event) => event.path);

		if (
			paths.length > 0 &&
			(await this.server.projectManager.maybeEvictProjects(paths))
		) {
			const displayPaths = joinMarkup(
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
