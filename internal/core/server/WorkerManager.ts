/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ProjectDefinition} from "@internal/project";
import {SimpleStats} from "./fs/MemoryFileSystem";
import {forkThread} from "../common/utils/fork";
import {
	LAG_INTERVAL,
	MAX_MASTER_BYTES_BEFORE_WORKERS,
	MAX_WORKER_BYTES_BEFORE_ADD,
} from "../common/constants";
import {
	MAX_WORKER_COUNT,
	Server,
	ThreadWorkerContainer,
	Worker,
	WorkerBridge,
	WorkerContainer,
	WorkerType,
} from "@internal/core";
import {Locker} from "../../async/lockers";
import {Event} from "@internal/events";
import {AbsoluteFilePath} from "@internal/path";
import {markup} from "@internal/markup";
import prettyFormat from "@internal/pretty-format";
import {ReporterNamespace} from "@internal/cli-reporter";
import {ExtendedMap} from "@internal/collections";
import {Duration, DurationMeasurer} from "@internal/numbers";
import {PartialWorkerOptions, WorkerOptions} from "../worker/types";
import {createResourceFromCallback} from "@internal/resources";

type SpawnWorkerOptions = {
	type: WorkerType;
	ghost?: boolean;
	inspectorPort?: number;
	id?: number;
	env?: WorkerOptions["env"];
	displayID?: number;
	pipeIO?: boolean;
};

export default class WorkerManager {
	constructor(server: Server) {
		this.server = server;

		this.workerStartEvent = new Event("WorkerManager.workerStart");
		this.selfWorker = true;
		this.locker = new Locker();
		this.workers = new ExtendedMap("workers");
		this.idCounter = 0;

		this.logger = server.logger.namespace(markup`WorkerManager`);
	}

	public workerStartEvent: Event<WorkerContainer, void>;
	public locker: Locker<number>;

	private server: Server;
	private logger: ReporterNamespace;
	private selfWorker: boolean;
	private workers: ExtendedMap<number, WorkerContainer>;

	// We use an idCounter rather than using workers.size due to race conditions
	// If we use workers.size to generate the next id, then by the time we insert it
	// into the map between async operations, it could already be filled!
	private idCounter: number;

	private setWorker(container: WorkerContainer) {
		this.workers.set(container.id, container);

		const {bridge, logger} = container;

		bridge.sendMessageEvent.subscribe((raw) => {
			logger.info(() => {
				const msg = bridge.getDebugMessage(raw);
				return markup`Sending message ${prettyFormat(msg)}`;
			});
		});

		bridge.receivedMessageEvent.subscribe((raw) => {
			logger.info(() => {
				const msg = bridge.getDebugMessage(raw);
				return markup`Received message ${prettyFormat(msg)}`;
			});
		});
	}

	private getNextWorkerId(): number {
		return this.idCounter++;
	}

	public getWorkerAssert(id: number): WorkerContainer {
		return this.workers.assert(id);
	}

	public getWorkers(): WorkerContainer[] {
		return Array.from(this.workers.values());
	}

	private getProcessorWorkerCount(): number {
		let count = 0;
		for (const worker of this.workers.values()) {
			if (worker.type === "file-processor" && !worker.ghost) {
				count++;
			}
		}
		return count;
	}

	// Get all the workers that live in external processes
	public getExternalWorkers(): WorkerContainer[] {
		return this.getWorkers().filter((worker) => worker.thread !== undefined);
	}

	private getLowestByteCountWorker(): WorkerContainer {
		// Find the worker with the lowest byteCount value
		let smallestWorker;
		let byteCount;
		for (const worker of this.workers.values()) {
			if (
				!worker.ghost &&
				worker.type === "file-processor" &&
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

	public async init(): Promise<void> {
		// Create the worker
		const bridges = WorkerBridge.createFromLocal();
		this.server.resources.add(bridges.server);

		const worker = new Worker({
			userConfig: this.server.userConfig,
			bridge: bridges.client,
			dedicated: false,
			...this.buildPartialWorkerOptions({
				type: "file-processor",
				id: 0,
				inspectorPort: undefined,
				env: {},
			}),
		});
		this.server.resources.add(worker);

		// We make an assumption elsewhere in the code that this is always the first worker
		// Let's use an invariant here for completeness
		const id = this.getNextWorkerId();
		if (id !== 0) {
			throw new Error("Expected server worker id to be 0");
		}

		const container: WorkerContainer = {
			type: "file-processor",
			displayName: "local worker",
			id: 0,
			fileCount: 0,
			byteCount: 0n,
			thread: undefined,
			bridge: bridges.server,
			ghost: false,
			ready: false,
			logger: this.logger.namespace(markup`local worker`),
		};
		this.setWorker(container);
		await worker.init();

		await Promise.all([
			this.workerHandshake(container),
			bridges.client.handshake(),
		]);

		this.workerStartEvent.send(container);
	}

	private async replaceOwnWorker() {
		const lock = this.locker.getNewLock(0);

		try {
			const serverWorker = this.getWorkerAssert(0);
			this.selfWorker = false;

			this.logger.info(
				markup`Spawning first worker outside of server after exceeding ${String(
					MAX_MASTER_BYTES_BEFORE_WORKERS,
				)} bytes`,
			);

			// Spawn a new worker
			const newWorker = await this.spawnProcessorWorker({
				displayID: 0,
				ghost: true,
			});

			// Transfer buffers to the new worker
			if (this.server.memoryFs.hasAnyBuffers()) {
				const buffers = await serverWorker.bridge.events.getFileBuffers.call();

				for (const [path, buffer] of buffers) {
					await newWorker.bridge.events.updateBuffer.call({
						ref: this.server.projectManager.getFileReference(path),
						buffer,
					});
				}
			}

			// End the old worker, will automatically cleanup
			await serverWorker.bridge.end();

			// Swap the workers
			// We perform this as a single atomic operation rather than doing it in spawnWorker so we have predictable worker retrieval
			this.workers.set(
				0,
				{
					...newWorker,
					type: "file-processor",
					id: 0,
					fileCount: serverWorker.fileCount,
					byteCount: serverWorker.byteCount,
					ghost: false,
					ready: true,
				},
			);
			this.workers.delete(newWorker.id);

			this.logger.info(
				markup`Successfully replaced server worker with a dedicated one`,
			);
		} finally {
			lock.release();
		}
	}

	public async onNewProject(newProject: ProjectDefinition) {
		await this.server.projectManager.notifyWorkersOfProjects(
			this.getWorkers(),
			[newProject],
		);
	}

	private async workerHandshake(worker: WorkerContainer) {
		const {bridge} = worker;
		await bridge.handshake({
			timeout: Duration.fromSeconds(3),
		});
		await this.server.projectManager.notifyWorkersOfProjects([worker]);
		worker.ready = true;
	}

	private async spawnProcessorWorker(
		opts: Omit<SpawnWorkerOptions, "type" | "id"> = {},
	): Promise<WorkerContainer> {
		const id = this.getNextWorkerId();
		const lock = this.locker.getNewLock(id);
		try {
			const container = await this.spawnWorkerUnsafe({
				...opts,
				type: "file-processor",
				id,
			});

			container.bridge.startHeartbeatMonitor(
				LAG_INTERVAL,
				({summary, totalTime, attempts}) => {
					return;
					const reporter = this.server.getImportantReporter();
					reporter.warn(
						markup`Worker <emphasis>${id}</emphasis> has not responded for <emphasis>${totalTime}</emphasis>. It is unlikely to become responsive. Currently processing:`,
					);
					reporter.list(summary);
					reporter.info(
						markup`Please open an issue with the details provided above if necessary`,
					);

					if (attempts >= 5) {
						this.server.fatalErrorHandler.handle(
							new Error(`Unresponsive for ${totalTime.format()}`),
							container.displayName,
						);
					}

					reporter.resources.release();
				},
			);

			return container;
		} finally {
			lock.release();
		}
	}

	private buildPartialWorkerOptions(
		opts: Omit<PartialWorkerOptions, "cacheReadDisabled" | "cacheWriteDisabled">,
	): PartialWorkerOptions {
		return {
			cacheReadDisabled: this.server.cache.readDisabled,
			cacheWriteDisabled: this.server.cache.writeDisabled,
			...opts,
		};
	}

	// Considered unsafe as we have no locks
	public async spawnWorkerUnsafe(
		{
			type,
			id = this.getNextWorkerId(),
			displayID = id,
			ghost = false,
			pipeIO = true,
			env = {},
			inspectorPort,
		}: SpawnWorkerOptions,
	): Promise<ThreadWorkerContainer> {
		let displayName = `worker ${displayID}`;
		if (type === "test-runner") {
			displayName = `test ${displayName}`;
		} else if (type === "script-runner") {
			displayName = `script ${displayName}`;
		}

		const start = new DurationMeasurer();

		const thread = forkThread(
			"worker",
			{
				workerData: this.buildPartialWorkerOptions({
					type,
					id,
					inspectorPort,
					env,
				}),
				stderr: !pipeIO,
				stdout: !pipeIO,
			},
		);

		const logger = this.logger.namespace(markup`${displayName}`);

		thread.stdout.on(
			"data",
			(chunk) => {
				logger.info(`stdout: ${String(chunk)}`);
			},
		);

		thread.stderr.on(
			"data",
			(chunk) => {
				logger.error(`stderr: ${String(chunk)}`);
			},
		);

		const {bridge, resource: threadResource} = WorkerBridge.Server.createFromWorkerThread(
			thread,
		);
		this.server.resources.add(bridge);

		bridge.events.fatalError.subscribe(async (details) => {
			await this.server.fatalErrorHandler.handle(
				bridge.hydrateCustomError(details),
				displayName,
			);
		});

		bridge.events.log.subscribe(({chunk, isError}) => {
			this.server.emitLog(chunk, "worker", isError);
		});

		const container: ThreadWorkerContainer = {
			type,
			id,
			fileCount: 0,
			byteCount: 0n,
			thread: {
				worker: thread,
				resources: threadResource,
			},
			bridge,
			ghost,
			ready: false,
			displayName,
			logger,
		};
		this.setWorker(container);
		bridge.resources.add(
			createResourceFromCallback(
				"WorkerManagerRegistration",
				() => {
					this.workers.delete(id);
				},
			),
		);

		thread.once(
			"error",
			(err) => {
				// The process could not be spawned, or
				// The process could not be killed, or
				// Sending a message to the child process failed.
				this.server.fatalErrorHandler.handle(err, displayName);
				thread.terminate();
			},
		);

		await this.workerHandshake(container);

		// If a worker is spawned while we're profiling then make sure it's profiling too
		const profileData = this.server.getRunningProfilingData();
		if (profileData !== undefined) {
			await bridge.events.profilingStart.call(profileData);
		}

		if (this.server.hasWorkerLogsSubscriptions()) {
			await bridge.events.setLogs.call(true);
		}

		this.workerStartEvent.send(container);

		container.logger.info(markup`Started in ${start.since()}`);

		return container;
	}

	public own(workerId: number, stats: SimpleStats) {
		const worker = this.getWorkerAssert(workerId);
		worker.byteCount += stats.size;
		worker.fileCount++;
	}

	public disown(workerId: number, stats: SimpleStats) {
		const worker = this.getWorkerAssert(workerId);
		worker.byteCount -= stats.size;
		worker.fileCount--;
	}

	public async getNextWorker(path: AbsoluteFilePath): Promise<WorkerContainer> {
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
			this.getProcessorWorkerCount() < MAX_WORKER_COUNT
		) {
			logger.info(
				markup`[WorkerManager] Spawning a new worker as we've exceeded ${String(
					MAX_WORKER_BYTES_BEFORE_ADD,
				)} bytes across each worker`,
			);
			const container = await this.spawnProcessorWorker();
			workerId = container.id;
		}

		// Register size of file
		this.own(workerId, stats);

		// Just in case we've chosen a worker that's still spawning
		await this.locker.waitLock(workerId);

		return this.getWorkerAssert(workerId);
	}
}
