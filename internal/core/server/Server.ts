/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerBridge, ServerQueryRequest, UserConfig} from "@internal/core";
import {
	Diagnostic,
	DiagnosticOrigin,
	DiagnosticsProcessor,
} from "@internal/diagnostics";
import {
	DiagnosticsFileHandler,
	printDiagnostics,
} from "@internal/cli-diagnostics";
import {BridgeServer, Event, EventQueue} from "@internal/events";
import ServerRequest from "./ServerRequest";
import ProjectManager from "./project/ProjectManager";
import WorkerManager from "./WorkerManager";
import Resolver from "./fs/Resolver";
import FileAllocator from "./fs/FileAllocator";
import Logger from "../common/utils/Logger";
import MemoryFileSystem, {SimpleStats} from "./fs/MemoryFileSystem";
import ServerCache from "./ServerCache";
import {
	Reporter,
	ReporterConditionalStream,
	ReporterProgress,
	ReporterProgressOptions,
	mergeProgresses,
} from "@internal/cli-reporter";
import {
	PartialServerQueryRequest,
	ProfilingStartData,
	ServerBridgeLog,
} from "../common/bridges/ServerBridge";
import {
	ClientLogsLevel,
	ClientRequestFlags,
	DEFAULT_CLIENT_REQUEST_FLAGS,
} from "../common/types/client";
import {AbsoluteFilePath} from "@internal/path";
import {mergeObjects} from "@internal/typescript-helpers";
import LSPServer from "./lsp/LSPServer";
import ServerReporter from "./ServerReporter";
import VirtualModules from "../common/VirtualModules";
import {DiagnosticsProcessorOptions} from "@internal/diagnostics/DiagnosticsProcessor";
import {PathLocker} from "../../async/lockers";
import {DEFAULT_TERMINAL_FEATURES} from "@internal/cli-environment";
import {markup} from "@internal/markup";
import prettyFormat from "@internal/pretty-format";
import RecoveryStore from "./fs/RecoveryStore";
import WorkerQueue, {WorkerQueueOptions} from "./WorkerQueue";
import FatalErrorHandler from "../common/FatalErrorHandler";
import {
	Resource,
	createResourceFromCallback,
	createResourceRoot,
} from "@internal/resources";
import ServerClient from "./ServerClient";
import {Profiler} from "@internal/v8";

export type ServerOptions = {
	inbandOnly?: boolean;
	forceCacheEnabled?: boolean;
	userConfig: UserConfig;
	dedicated: boolean;
	daemon: boolean;
};

export type ServerUnfinishedMarker = {
	// Text label to be associated with the timeline entry
	label: string;

	// Start time in milliseconds
	start: number;

	// Process / Thread that the events are associated with
	rowId: string;

	//
	facet: string;
};

export type ServerMarker = ServerUnfinishedMarker & {
	// End time in milliseconds
	end: number;
};

export function partialServerQueryRequestToFull(
	partialQuery: PartialServerQueryRequest,
): ServerQueryRequest {
	const requestFlags: ClientRequestFlags = mergeObjects(
		DEFAULT_CLIENT_REQUEST_FLAGS,
		partialQuery.requestFlags || {},
	);

	return {
		commandName: partialQuery.commandName,
		args: partialQuery.args ?? [],
		noData: partialQuery.noData === true,
		noFileWrites: partialQuery.noFileWrites === true,
		requestFlags,
		silent: partialQuery.silent === true || requestFlags.benchmark,
		commandFlags: partialQuery.commandFlags === undefined
			? {}
			: partialQuery.commandFlags,
		cancelToken: partialQuery.cancelToken,
	};
}

export type ServerRefreshFile =
	| {
			type: "DELETED";
			path: AbsoluteFilePath;
		}
	| {
			type: "CREATED";
			path: AbsoluteFilePath;
		}
	| {
			type: "DISK_UPDATE";
			path: AbsoluteFilePath;
			oldStats: undefined | SimpleStats;
			newStats: SimpleStats;
		}
	| {
			type: "BUFFER_UPDATE";
			path: AbsoluteFilePath;
		};

export default class Server {
	constructor(opts: ServerOptions) {
		this.resources = createResourceRoot("Server");

		this.profiling = undefined;
		this.options = opts;
		this.userConfig = opts.userConfig;

		this.fatalErrorHandler = new FatalErrorHandler({
			source: markup`server`,
			exit: this.options.dedicated,
			getReporter: () => {
				return this.getImportantReporter();
			},
		});

		this.requestFileLocker = new PathLocker();
		this.connectedReporters = new ServerReporter(this);
		this.connectedClientsListeningForWorkerLogs = new Set();
		this.connectedClientsListeningForLogs = new Map();
		this.connectedLSPServers = new Set();
		this.connectedClients = new Set();

		this.hadConnectedClient = false;
		this.clientIdCounter = 0;

		this.logInitBuffer = [];

		this.clientStartEvent = new Event("Server.clientStart");
		this.requestStartEvent = new Event("Server.requestStart");
		this.refreshFileEvent = new EventQueue(
			"Server.refreshFileEvent",
			{toDedupeKey: ({path}) => path.join()},
		);

		this.logger = new Logger(
			this.resources,
			{
				markupOptions: {
					userConfig: this.userConfig,
					humanizeFilename: (path) => {
						if (path.isAbsolute()) {
							const remote = this.projectManager.getRemoteFromLocalPath(
								path.assertAbsolute(),
							);
							if (remote !== undefined) {
								return remote.join();
							}
						}
						return undefined;
					},
					normalizePosition: (path, line, column) => {
						const normalPath = this.projectManager.maybeGetFilePathFromUID(path);
						if (normalPath === undefined) {
							return {path, line, column};
						} else {
							return {path: normalPath, line, column};
						}
					},
				},
			},
			"Server",
		);

		this.loggerStream = this.logger.attachConditionalStream(
			{
				format: "markup",
				features: {
					...DEFAULT_TERMINAL_FEATURES,
					columns: undefined,
				},
				write: (chunk, isError) => {
					this.emitLog(chunk, "server", isError);
				},
			},
			() => {
				return (
					!this.hadConnectedClient ||
					this.connectedClientsListeningForLogs.size > 0
				);
			},
		);

		this.virtualModules = new VirtualModules();
		this.recoveryStore = new RecoveryStore(this);
		this.memoryFs = new MemoryFileSystem(this);
		this.projectManager = new ProjectManager(this);
		this.workerManager = new WorkerManager(this);
		this.fileAllocator = new FileAllocator(this);
		this.resolver = new Resolver(this);
		this.cache = new ServerCache(this);

		this.logger.info(
			markup`[Server] Created Server with options: ${prettyFormat(opts)}`,
		);

		this.resources.add(this.connectedReporters);

		if (this.options.daemon) {
			this.resources.add(this.fatalErrorHandler.setupGlobalHandlers());
		}
	}

	public userConfig: UserConfig;
	public fatalErrorHandler: FatalErrorHandler;
	public options: ServerOptions;
	public resources: Resource;

	// Public events
	public requestStartEvent: Event<ServerRequest, void>;
	public clientStartEvent: Event<ServerClient, void>;

	// Event for when a file needs to be "refreshed". This could include:
	// - Deleted
	// - Created
	// - Modified
	// - Buffer updated
	public refreshFileEvent: EventQueue<ServerRefreshFile>;

	// Public modules
	public recoveryStore: RecoveryStore;
	public memoryFs: MemoryFileSystem;
	public virtualModules: VirtualModules;
	public resolver: Resolver;
	public projectManager: ProjectManager;
	public workerManager: WorkerManager;
	public fileAllocator: FileAllocator;
	public cache: ServerCache;
	public connectedReporters: ServerReporter;
	public logger: Logger;
	public requestFileLocker: PathLocker;

	private loggerStream: ReporterConditionalStream;

	// Before we receive our first connected client we will buffer our server init logs
	// These __should__ be relatively cheap to retain since we don't do a lot
	private logInitBuffer: [string, boolean][];

	private clientIdCounter: number;
	private hadConnectedClient: boolean;
	private profiling: undefined | ProfilingStartData;

	private connectedClients: Set<ServerClient>;
	private connectedLSPServers: Set<LSPServer>;

	private connectedClientsListeningForLogs: Map<ServerClient, ClientLogsLevel>;
	private connectedClientsListeningForWorkerLogs: Set<ServerClient>;

	// Used when starting up child processes and indicates whether they should start profiling
	// on init
	public getRunningProfilingData(): undefined | ProfilingStartData {
		return this.profiling;
	}

	// Used when starting up workers and indicates whether they should start sending logs
	public hasWorkerLogsSubscriptions(): boolean {
		return this.connectedClientsListeningForWorkerLogs.size > 0;
	}

	public async updateWorkerLogsSubscriptions() {
		const enabled = this.hasWorkerLogsSubscriptions();

		await Promise.all(
			this.workerManager.getWorkers().map((worker) => {
				return worker.bridge.events.setLogs.call(enabled);
			}),
		);
	}

	// Derive a concatenated reporter from the logger and all connected clients
	// This should only be used synchronously as new clients will not be added after creation
	// Used for very important log messages
	public getImportantReporter(): Reporter {
		const reporters: Reporter[] = [this.logger];
		for (const client of this.connectedClients) {
			if (!this.connectedClientsListeningForLogs.has(client)) {
				reporters.push(client.reporter);
			}
		}
		if (this.connectedClients.size === 0) {
			reporters.push(Reporter.fromProcess());
		}
		return Reporter.concat(reporters);
	}

	public createWorkerQueue<M = void>(
		opts: WorkerQueueOptions<M>,
	): WorkerQueue<M> {
		return new WorkerQueue<M>(this, opts);
	}

	public emitLog(
		chunk: string,
		origin: ServerBridgeLog["origin"],
		isError: boolean,
	) {
		if (this.clientIdCounter === 0) {
			this.logInitBuffer.push([chunk, isError]);
		}

		const log: ServerBridgeLog = {chunk, origin, isError};

		for (const [client, level] of this.connectedClientsListeningForLogs) {
			if (
				origin === "worker" &&
				!this.connectedClientsListeningForWorkerLogs.has(client)
			) {
				continue;
			}

			this.sendLog(client.bridge, level, log);
		}
	}

	public sendLog(
		bridge: BridgeServer<typeof ServerBridge>,
		level: ClientLogsLevel,
		log: ServerBridgeLog,
	) {
		// Sometimes the bridge hasn't completely been teardown and we still consider it connected
		if (!bridge.open) {
			return;
		}

		if (log.isError || level === "all") {
			bridge.events.log.send(log);
		}
	}

	// This is so all progress bars are renderer on each client. If we just use this.progressLocal then
	// while it would work, we would be doing all the rendering work on the server
	// The CLI also needs to know all the activeElements so it can properly draw and clear lines
	// We also create a progress bar for all connected LSP clients
	// Refer to ServerReporter
	public createConnectedProgress(opts?: ReporterProgressOptions) {
		const progresses: ReporterProgress[] = [];

		for (const client of this.connectedClients) {
			progresses.push(client.reporter.progress(opts));
		}

		for (const server of this.connectedLSPServers) {
			progresses.push(server.createProgress(opts));
		}

		return mergeProgresses(progresses);
	}

	private async handleDisconnectedDiagnostics(diagnostics: Diagnostic[]) {
		const reporter = this.getImportantReporter();

		reporter.error(markup`Generated diagnostics without a current request`);

		await printDiagnostics({
			diagnostics,
			suppressions: [],
			printerOptions: {
				processor: this.createDiagnosticsProcessor(),
				reporter,
				fileHandlers: [this.createDiagnosticsPrinterFileHandler()],
			},
		});

		reporter.resources.release();
	}

	public createDiagnosticsPrinterFileHandler(): DiagnosticsFileHandler {
		return {
			read: async (path) => {
				const virtualContents = this.virtualModules.getPossibleVirtualFileContents(
					path,
				);
				if (virtualContents === undefined) {
					return undefined;
				} else {
					return virtualContents;
				}
			},
			exists: async (path) => {
				if (path.isAbsolute() && this.virtualModules.isVirtualPath(path)) {
					return true;
				} else {
					return undefined;
				}
			},
		};
	}

	public createDiagnosticsProcessor(
		opts: DiagnosticsProcessorOptions = {},
	): DiagnosticsProcessor {
		return new DiagnosticsProcessor({
			markupOptions: this.logger.markupOptions,
			...opts,
		});
	}

	public createDisconnectedDiagnosticsProcessor(
		origins: DiagnosticOrigin[],
	): DiagnosticsProcessor {
		const processor = this.createDiagnosticsProcessor({
			origins: [
				...origins,
				{
					category: "server",
					message: "Created disconnected diagnostics collector",
				},
			],
		});

		processor.insertDiagnosticsEvent.subscribe(() => {
			if (processor.hasDiagnostics()) {
				this.fatalErrorHandler.wrapPromise(
					this.handleDisconnectedDiagnostics(processor.getDiagnostics()),
				);
			}
		});

		return processor;
	}

	public async init() {
		await this.recoveryStore.init();
		await this.virtualModules.init();
		await this.projectManager.init();
		await this.memoryFs.init();
		await this.fileAllocator.init();
		await this.resolver.init();
		await this.cache.init();
		await this.workerManager.init();
	}

	public async end() {
		this.logger.info(markup`[Server] Teardown triggered`);

		// Unwatch all project directories
		// We do this before anything else as we don't want events firing while we're in a teardown state
		this.memoryFs.unwatchAll();

		// Cancel all queries in flight
		for (const client of this.connectedClients) {
			for (const req of client.requestsInFlight) {
				await req.cancel("server ended");
			}

			// Kill socket
			await client.bridge.end();
		}

		await this.resources.release();
	}

	public onLSPServer(req: ServerRequest, lsp: LSPServer) {
		this.connectedLSPServers.add(lsp);

		req.endEvent.subscribe(() => {
			this.connectedLSPServers.delete(lsp);
		});
	}

	public async createClient(
		bridge: BridgeServer<typeof ServerBridge>,
	): Promise<ServerClient> {
		const id = this.clientIdCounter++;

		if (!this.hadConnectedClient) {
			this.hadConnectedClient = true;
			this.loggerStream.update();
		}

		const client = new ServerClient(this, id, bridge);
		await client.init();

		let profiler: undefined | Profiler;

		bridge.events.profilingStart.subscribe(async (data) => {
			if (profiler !== undefined) {
				throw new Error("Expected no profiler to be running");
			}
			profiler = new Profiler();
			await profiler.startProfiling(data.samplingInterval);
			this.profiling = data;
			for (const {bridge} of this.workerManager.getExternalWorkers()) {
				await bridge.events.profilingStart.call(data);
			}
		});

		bridge.events.profilingStop.subscribe(async () => {
			if (profiler === undefined) {
				throw new Error("Expected profiler to be running");
			}
			const serverProfile = await profiler.stopProfiling();
			profiler = undefined;
			this.profiling = undefined;
			return serverProfile;
		});

		bridge.events.setLogLevel.subscribe(async ({level, includeWorker}) => {
			let startWorkerLogsEnabled = this.hasWorkerLogsSubscriptions();

			if (includeWorker) {
				this.connectedClientsListeningForWorkerLogs.add(client);
			}

			if (level === undefined) {
				this.connectedClientsListeningForLogs.delete(client);
				this.connectedClientsListeningForWorkerLogs.delete(client);
			} else {
				if (!this.connectedClientsListeningForLogs.has(client)) {
					// Send init logs
					for (const [chunk, isError] of this.logInitBuffer) {
						this.sendLog(bridge, level, {chunk, origin: "server", isError});
					}

					// Send separator
					this.sendLog(
						bridge,
						level,
						{chunk: ".".repeat(20) + "\n", origin: "server", isError: false},
					);
				}

				this.connectedClientsListeningForLogs.set(client, level);
			}

			this.loggerStream.update();

			const currWorkerLogsEnabled = this.hasWorkerLogsSubscriptions();
			if (currWorkerLogsEnabled !== startWorkerLogsEnabled) {
				await this.updateWorkerLogsSubscriptions();
			}
		});

		this.connectedClients.add(client);

		client.resources.add(
			createResourceFromCallback(
				"ServerRegistration",
				() => {
					this.connectedClients.delete(client);
					this.connectedClientsListeningForLogs.delete(client);
					this.connectedClientsListeningForWorkerLogs.delete(client);
					this.loggerStream.update();
				},
			),
		);

		await this.clientStartEvent.callOptional(client);
		await bridge.events.serverReady.call();

		return client;
	}
}
