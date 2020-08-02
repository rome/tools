/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ProjectDefinition} from "@internal/project";
import {Stats} from "./fs/MemoryFileSystem";
import fork from "../common/utils/fork";
import {
	LAG_INTERVAL,
	MAX_MASTER_BYTES_BEFORE_WORKERS,
	MAX_WORKER_BYTES_BEFORE_ADD,
} from "../common/constants";
import {MAX_WORKER_COUNT, Server, Worker, WorkerBridge} from "@internal/core";
import {Locker} from "../common/utils/lockers";
import {
	Event,
	createBridgeFromChildProcess,
	createBridgeFromLocal,
} from "@internal/events";
import child = require("child_process");
import {AbsoluteFilePath, createAbsoluteFilePath} from "@internal/path";
import {markup} from "@internal/markup";
import prettyFormat from "@internal/pretty-format";
import {ReporterNamespace} from "@internal/cli-reporter";

export type WorkerContainer = {
	id: number;
	fileCount: number;
	byteCount: number;
	bridge: WorkerBridge;
	process: undefined | child.ChildProcess;
	// Whether we've completed a handshake with the worker and it's ready to receive requests
	ready: boolean;
	// Whether we should assign files to this worker
	ghost: boolean;
};

export default class WorkerManager {
	constructor(server: Server) {
		this.server = server;

		this.workerStartEvent = new Event({
			name: "WorkerManager.workerStart",
			onError: server.onFatalErrorBound,
		});
		this.selfWorker = true;
		this.locker = new Locker();
		this.workers = new Map();
		this.idCounter = 0;

		this.logger = server.logger.namespace(markup`[WorkerManager]`);
	}

	server: Server;
	locker: Locker<number>;
	logger: ReporterNamespace;

	selfWorker: boolean;
	workerStartEvent: Event<WorkerBridge, void>;

	workers: Map<number, WorkerContainer>;

	// We use an idCounter rather than using workers.size due to race conditions
	// If we use workers.size to generate the next id, then by the time we insert it
	// into the map between async operations, it could already be filled!
	idCounter: number;

	getNextWorkerId(): number {
		return this.idCounter++;
	}

	getWorkerAssert(id: number): WorkerContainer {
		const worker = this.workers.get(id);
		if (worker === undefined) {
			throw new Error("Expected worker");
		}
		return worker;
	}

	getWorkers(): Array<WorkerContainer> {
		return Array.from(this.workers.values());
	}

	// Get worker count, excluding ghost workers
	getWorkerCount(): number {
		let count = 0;
		for (const worker of this.workers.values()) {
			if (worker.ghost === false) {
				count++;
			}
		}
		return count;
	}

	// Get all the workers that live in external processes
	getExternalWorkers(): Array<WorkerContainer> {
		return this.getWorkers().filter((worker) => worker.process !== undefined);
	}

	// Safe way to ensure all workers get properly killed if we need to kill the main process
	killWorkers() {}

	end() {
		// Shutdown all workers, no need to clean up any internal data structures since they will never be used
		for (const {process, bridge} of this.workers.values()) {
			bridge.end();

			if (process !== undefined) {
				process.kill();
			}
		}
	}

	getLowestByteCountWorker(): WorkerContainer {
		// Find the worker with the lowest byteCount value
		let smallestWorker;
		let byteCount;
		for (const worker of this.workers.values()) {
			if (
				!worker.ghost &&
				(byteCount === undefined || byteCount > worker.byteCount)
			) {
				smallestWorker = worker;
				byteCount = worker.byteCount;
			}
		}

		if (smallestWorker === undefined) {
			// This shouldn't be possible
			throw new Error("No worker found");
		} else {
			return smallestWorker;
		}
	}

	async init(): Promise<void> {
		// Create the worker
		const bridge = createBridgeFromLocal(WorkerBridge, {});
		const worker = new Worker({
			userConfig: this.server.userConfig,
			bridge,
			globalErrorHandlers: false,
		});

		// We make an assumption elsewhere in the code that this is always the first worker
		// Let's use an invariant here for completeness
		const id = this.getNextWorkerId();
		if (id !== 0) {
			throw new Error("Expected server worker id to be 0");
		}

		const container: WorkerContainer = {
			id: 0,
			fileCount: 0,
			byteCount: 0,
			process: undefined,
			bridge,
			ghost: false,
			ready: false,
		};
		this.workers.set(0, container);
		await worker.init();

		await Promise.all([this.workerHandshake(container), bridge.handshake()]);

		this.workerStartEvent.send(bridge);
	}

	async replaceOwnWorker() {
		const lock = this.locker.getNewLock(0);

		try {
			const serverWorker = this.getWorkerAssert(0);
			this.logger.info(
				markup`Spawning first worker outside of server after exceeding ${String(
					MAX_MASTER_BYTES_BEFORE_WORKERS,
				)} bytes`,
			);
			this.selfWorker = false;

			// Spawn a new worker
			const newWorker = await this.spawnWorker(this.getNextWorkerId(), true);

			// Transfer buffers to the new worker
			const buffers = await serverWorker.bridge.getFileBuffers.call();

			for (const {filename, content} of buffers) {
				await newWorker.bridge.updateBuffer.call({
					ref: this.server.projectManager.getTransportFileReference(
						createAbsoluteFilePath(filename),
					),
					content,
				});
			}

			// End the old worker, will automatically cleanup
			serverWorker.bridge.end();

			// Swap the workers

			// We perform this as a single atomic operation rather than doing it in spawnWorker so we have predictable worker retrieval
			this.workers.set(
				0,
				{
					id: 0,
					fileCount: serverWorker.fileCount,
					byteCount: serverWorker.byteCount,
					bridge: newWorker.bridge,
					process: newWorker.process,
					ghost: false,
					ready: true,
				},
			);
			this.workers.delete(newWorker.id);
		} finally {
			lock.release();
		}
	}

	onNewProject(newProject: ProjectDefinition) {
		this.server.projectManager.notifyWorkersOfProjects(
			this.getWorkers(),
			[newProject],
		);
	}

	async workerHandshake(worker: WorkerContainer) {
		const {bridge} = worker;
		await bridge.handshake({timeout: 3_000});
		await this.server.projectManager.notifyWorkersOfProjects([worker]);
		worker.ready = true;
	}

	async spawnWorker(
		workerId: number,
		isGhost: boolean = false,
	): Promise<WorkerContainer> {
		const lock = this.locker.getNewLock(workerId);
		try {
			return await this._spawnWorker(workerId, isGhost);
		} finally {
			lock.release();
		}
	}

	async _spawnWorker(
		workerId: number,
		isGhost: boolean,
	): Promise<WorkerContainer> {
		const fatalErrorSource = markup`worker ${workerId}`;
		const start = Date.now();

		const process = fork("worker");

		const bridge = createBridgeFromChildProcess(
			WorkerBridge,
			process,
			{
				type: "client",
				onSendMessage: (data) => {
					this.logger.info(
						markup`Sending worker request to ${String(workerId)}: ${prettyFormat(
							data,
						)}`,
					);
				},
			},
		);

		bridge.fatalError.subscribe((details) => {
			this.server.onFatalError(bridge.hydrateError(details), fatalErrorSource);
		});

		bridge.monitorHeartbeat(
			LAG_INTERVAL,
			({summary, totalTime, iterations}) => {
				const reporter = this.server.getImportantReporter();
				reporter.warn(
					markup`Worker <emphasis>${workerId}</emphasis> has not responded for <emphasis><duration>${String(
						totalTime,
					)}</duration> seconds</emphasis>. It is unlikely to become responsive. Currently processing:`,
				);
				reporter.list(summary);
				reporter.info(
					markup`Please open an issue with the details provided above if necessary`,
				);

				if (iterations > 5) {
					this.server.onFatalError(
						new Error(
							`Did not respond for ${totalTime}ms and was checked ${iterations} times`,
						),
						fatalErrorSource,
					);
				}
			},
		);

		const worker: WorkerContainer = {
			id: workerId,
			fileCount: 0,
			byteCount: 0,
			process,
			bridge,
			ghost: isGhost,
			ready: false,
		};
		this.workers.set(workerId, worker);

		process.once(
			"error",
			(err) => {
				// The process could not be spawned, or
				// The process could not be killed, or
				// Sending a message to the child process failed.
				this.server.onFatalError(err, fatalErrorSource);
				process.kill();
			},
		);

		process.once(
			"exit",
			() => {
				this.server.onFatalError(
					new Error("Process unexpectedly exit"),
					fatalErrorSource,
				);
			},
		);

		await this.workerHandshake(worker);

		// If a worker is spawned while we're profiling then make sure it's profiling too
		if (this.server.profiling !== undefined) {
			await bridge.profilingStart.call(this.server.profiling);
		}

		this.workerStartEvent.send(bridge);

		this.logger.info(
			markup`Worker ${String(workerId)} started after <duration>${String(
				Date.now() - start,
			)}</duration>`,
		);

		return worker;
	}

	own(workerId: number, stats: Stats) {
		const worker = this.getWorkerAssert(workerId);
		worker.byteCount += stats.size;
		worker.fileCount++;
	}

	disown(workerId: number, stats: Stats) {
		const worker = this.getWorkerAssert(workerId);
		worker.byteCount -= stats.size;
		worker.fileCount--;
	}

	async getNextWorker(path: AbsoluteFilePath): Promise<WorkerContainer> {
		const {logger, memoryFs, fileAllocator} = this.server;

		// Get stats first
		let stats = memoryFs.getFileStats(path);
		if (stats === undefined) {
			// Give memoryFs a chance to finish initializing if it's in a pending project
			await this.server.memoryFs.waitIfInitializingWatch(path);

			stats = memoryFs.getFileStatsAssert(path);
		}

		// Verify that this file doesn't exceed any size limit
		fileAllocator.verifySize(path, stats);

		// Lock in case we're in the process of swapping the server worker with a dedicated worker
		await this.locker.waitLock(0);

		// If we are inband only then we should never fork workers
		if (this.server.options.inbandOnly) {
			this.own(0, stats);
			return this.getWorkerAssert(0);
		}

		// If the worker is running in the server process and we've exceed our byte limit
		// then start up a dedicated worker process
		if (this.selfWorker) {
			const worker = this.getWorkerAssert(0);
			if (worker.byteCount > MAX_MASTER_BYTES_BEFORE_WORKERS) {
				await this.replaceOwnWorker();
			}
		}

		// Find the worker with the lowest owned byte size
		const smallestWorker = this.getLowestByteCountWorker();
		let workerId = smallestWorker.id;

		// When the smallest worker exceeds the max worker byte limit and we're still under
		// our max worker limit, then let's start a new one
		if (
			smallestWorker.byteCount > MAX_WORKER_BYTES_BEFORE_ADD &&
			this.getWorkerCount() < MAX_WORKER_COUNT
		) {
			logger.info(
				markup`[WorkerManager] Spawning a new worker as we've exceeded ${String(
					MAX_WORKER_BYTES_BEFORE_ADD,
				)} bytes across each worker`,
			);
			workerId = this.getNextWorkerId();
			await this.spawnWorker(workerId);
		}

		// Register size of file
		this.own(workerId, stats);

		// Just in case we've chosen a worker that's still spawning
		await this.locker.waitLock(workerId);

		return this.getWorkerAssert(workerId);
	}
}
