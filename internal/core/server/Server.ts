/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ServerBridge,
	ServerQueryRequest,
	ServerQueryResponse,
	UserConfig,
	VERSION,
} from "@internal/core";
import {
	DIAGNOSTIC_CATEGORIES,
	DiagnosticOrigin,
	Diagnostics,
	DiagnosticsProcessor,
	descriptions,
} from "@internal/diagnostics";
import {ServerCommand, serverCommands} from "./commands";
import {
	DiagnosticsFileHandler,
	printDiagnostics,
} from "@internal/cli-diagnostics";
import {ConsumePath, consume} from "@internal/consume";
import {BridgeServer, Event, EventQueue} from "@internal/events";
import ServerRequest, {EMPTY_SUCCESS_RESPONSE} from "./ServerRequest";
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
import {Profiler} from "@internal/v8";
import {
	PartialServerQueryRequest,
	ProfilingStartData,
	ServerBridgeLog,
	ServerProfileWorker,
} from "../common/bridges/ServerBridge";
import {
	ClientFlags,
	ClientLogsLevel,
	ClientRequestFlags,
	DEFAULT_CLIENT_REQUEST_FLAGS,
} from "../common/types/client";
import {AbsoluteFilePath, createUIDPath} from "@internal/path";
import {Dict, mergeObjects} from "@internal/typescript-helpers";
import LSPServer from "./lsp/LSPServer";
import ServerReporter from "./ServerReporter";
import VirtualModules from "../common/VirtualModules";
import {DiagnosticsProcessorOptions} from "@internal/diagnostics/DiagnosticsProcessor";
import {toKebabCase} from "@internal/string-utils";
import {PathLocker} from "../../async/lockers";
import {DEFAULT_TERMINAL_FEATURES, getEnvVar} from "@internal/cli-environment";
import {markup} from "@internal/markup";
import prettyFormat from "@internal/pretty-format";
import RecoveryStore from "./fs/RecoveryStore";
import WorkerQueue, {WorkerQueueOptions} from "./WorkerQueue";
import FatalErrorHandler from "../common/FatalErrorHandler";
import {
	Resource,
	createResource,
	createResourceRoot,
} from "@internal/resources";
import {DurationMeasurer} from "@internal/numbers";

export type ServerClient = {
	id: number;
	reporter: Reporter;
	bridge: BridgeServer<typeof ServerBridge>;
	flags: ClientFlags;
	version: string;
	resources: Resource;
	requestsInFlight: Set<ServerRequest>;
};

export type ServerOptions = {
	inbandOnly?: boolean;
	forceCacheEnabled?: boolean;
	userConfig: UserConfig;
	dedicated: boolean;
	daemon: boolean;
	globalErrorHandlers: boolean;
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

const disallowedFlagsWhenReviewing: Array<keyof ClientRequestFlags> = ["watch"];

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

async function validateRequestFlags(
	req: ServerRequest,
	serverCommand: ServerCommand<Dict<unknown>>,
) {
	const {requestFlags} = req.query;

	// Commands need to explicitly allow these flags
	validateAllowedRequestFlag(req, "watch", serverCommand);
	validateAllowedRequestFlag(req, "review", serverCommand);

	// Don't allow review in combination with other flags
	if (requestFlags.review) {
		for (const key of disallowedFlagsWhenReviewing) {
			if (requestFlags[key]) {
				throw req.throwDiagnosticFlagError({
					description: descriptions.FLAGS.DISALLOWED_REVIEW_FLAG(key),
					target: {type: "flag", key},
				});
			}
		}
	}
}

function validateAllowedRequestFlag(
	req: ServerRequest,
	flagKey: NonNullable<ServerCommand<Dict<unknown>>["allowRequestFlags"]>[number],
	serverCommand: ServerCommand<Dict<unknown>>,
) {
	const allowRequestFlags = serverCommand.allowRequestFlags || [];
	if (req.query.requestFlags[flagKey] && !allowRequestFlags.includes(flagKey)) {
		throw req.throwDiagnosticFlagError({
			description: descriptions.FLAGS.DISALLOWED_REQUEST_FLAG(flagKey),
			target: {type: "flag", key: flagKey},
		});
	}
}

function sendLog(
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
			getOptions: () => {
				//console.dir(this.resources.buildTree(), {depth: null, customInspect: true});
				return {
					source: markup`server`,
					reporter: this.getImportantReporter(),
					exit: this.options.dedicated,
				};
			},
		});
		this.requestFileLocker = new PathLocker();
		this.connectedReporters = new ServerReporter(this);
		this.resources.add(this.connectedReporters);

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

			sendLog(client.bridge, level, log);
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

	private async handleDisconnectedDiagnostics(diagnostics: Diagnostics) {
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

	private maybeSetupGlobalErrorHandlers() {
		if (this.options.globalErrorHandlers) {
			this.resources.add(this.fatalErrorHandler.setupGlobalHandlers());
		}
	}

	public async init() {
		this.maybeSetupGlobalErrorHandlers();
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

	public async attachToBridge(
		bridge: BridgeServer<typeof ServerBridge>,
	): Promise<ServerClient> {
		const id = this.clientIdCounter++;
		const resources = createResource(`ServerClient<${id}>`);
		resources.add(bridge);
		this.resources.add(resources);

		if (!this.hadConnectedClient) {
			this.hadConnectedClient = true;
			this.loggerStream.update();
		}

		let profiler: undefined | Profiler;

		// If we aren't a dedicated process then we should only expect a single connection
		// and when that ends, end the Server.
		// NB: I don't think this is necessary as we already handle it in the Client
		if (!this.options.dedicated) {
			bridge.resources.addCallback("DedicatedEndHandler", () => this.end());
		}

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

		bridge.events.profilingGetWorkers.subscribe(async () => {
			const workers: ServerProfileWorker[] = [];
			for (const {id, displayName} of this.workerManager.getExternalWorkers()) {
				workers.push({id, displayName});
			}
			return workers;
		});

		bridge.events.profilingStopWorker.subscribe(async (id) => {
			const worker = this.workerManager.getWorkerAssert(id);
			return await worker.bridge.events.profilingStop.call();
		});

		bridge.resources.addCallback(
			"BridgeEndServerRequestCancellationHandler",
			async () => {
				for (const req of client.requestsInFlight) {
					await req.cancel("client disconnected");
				}
			},
		);

		await bridge.handshake();

		const client = await this.createClient({id, bridge, resources});

		if (client.version !== VERSION) {
			// TODO this wont ever actually be printed?
			client.reporter.error(
				markup`Client version ${client.version} does not match server version ${VERSION}. Goodbye lol.`,
			);
			await client.bridge.end();
			return client;
		}

		bridge.events.query.subscribe(async (request) => {
			return await this.handleRequest(client, request);
		});

		bridge.events.cancelQuery.subscribe(async (token) => {
			for (const req of client.requestsInFlight) {
				if (req.query.cancelToken === token) {
					await req.cancel("user requested");
				}
			}
		});

		bridge.events.endServer.subscribe(() => this.end());

		await this.clientStartEvent.callOptional(client);
		await bridge.events.serverReady.call();

		return client;
	}

	private async createClient(
		{id, bridge, resources}: {
			id: number;
			bridge: BridgeServer<typeof ServerBridge>;
			resources: Resource;
		},
	): Promise<ServerClient> {
		const {
			flags,
			streamState,
			outputFormat,
			outputSupport,
			version,
		} = await bridge.events.getClientInfo.call();

		// Initialize the reporter
		const reporter = new Reporter(
			"ServerClient",
			{
				wrapperFactory: this.fatalErrorHandler.wrapBound,
				markupOptions: {
					...this.logger.markupOptions,
					cwd: flags.cwd,
				},
			},
		);
		resources.add(reporter);

		const stream = reporter.addStream(
			{
				format: outputFormat,
				features: outputSupport,
				write(chunk: string, error: boolean) {
					if (flags.silent && !error) {
						return;
					}

					bridge.events.write.send([chunk, error]);
				},
			},
			streamState,
		);

		bridge.events.updateFeatures.subscribe((features) => {
			return stream.updateFeatures(features);
		});

		// Add reporter to connected set, important logs may be output to these
		this.connectedReporters.addAttachedStream(stream);

		// Warn about disabled disk caching. Don't bother if it's only been set due to ROME_DEV. We don't care to see it in development.
		if (this.cache.writeDisabled && getEnvVar("ROME_DEV").type !== "ENABLED") {
			reporter.warn(
				markup`Disk caching has been disabled due to the <emphasis>ROME_CACHE=0</emphasis> environment variable`,
			);
		}

		const client: ServerClient = {
			id,
			bridge,
			reporter,
			flags,
			version,
			resources,
			requestsInFlight: new Set(),
		};

		this.connectedClients.add(client);

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
						sendLog(bridge, level, {chunk, origin: "server", isError});
					}

					// Send separator
					sendLog(
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

		bridge.endEvent.subscribe(() => {
			this.connectedClients.delete(client);
			this.connectedClientsListeningForLogs.delete(client);
			this.connectedClientsListeningForWorkerLogs.delete(client);
			this.loggerStream.update();

			// Cancel any requests still in flight
			for (const req of client.requestsInFlight) {
				req.cancel("bridge died");
			}
		});

		return client;
	}

	public async handleRequestStart(req: ServerRequest) {
		req.logger.info(markup`Start ${prettyFormat(req.query)}`);

		// Hook used by the web server to track and collect server requests
		await this.requestStartEvent.callOptional(req);
	}

	public async handleRequestEnd(req: ServerRequest) {
		req.logger.info(markup`Request end`);
	}

	public async handleRequest(
		client: ServerClient,
		partialQuery: PartialServerQueryRequest,
	): Promise<ServerQueryResponse> {
		const query: ServerQueryRequest = partialServerQueryRequestToFull(
			partialQuery,
		);

		const req = new ServerRequest({
			client,
			query,
			server: this,
		});
		await req.init();

		try {
			let res: ServerQueryResponse = await this.dispatchRequest(req, []);
			res = await req.teardown(res);
			return res;
		} catch (err) {
			await this.fatalErrorHandler.handleAsync(err);
			throw new Error("Process should have quit already");
		}
	}

	private async dispatchBenchmarkRequest(
		req: ServerRequest,
	): Promise<ServerQueryResponse> {
		const {client} = req;
		const {reporter} = client;
		const {benchmarkIterations} = req.query.requestFlags;

		// Warmup
		const warmupStart = new DurationMeasurer();
		const result = await this.dispatchRequest(req, ["benchmark"]);
		const warmupTook = warmupStart.since();

		// Benchmark
		const progress = req.reporter.progress({title: markup`Running benchmark`});
		progress.setTotal(benchmarkIterations);
		const benchmarkStart = new DurationMeasurer();
		for (let i = 0; i < benchmarkIterations; i++) {
			await this.dispatchRequest(req, ["benchmark"]);
			progress.tick();
		}
		progress.end();
		const benchmarkTook = benchmarkStart.since();

		await reporter.section(
			markup`Benchmark results`,
			() => {
				reporter.info(
					markup`Request artifacts may have been cached after the first run, artificially decreasing subsequent run time`,
				);
				reporter.heading(markup`Query`);
				reporter.inspect(req.query);
				reporter.heading(markup`Stats`);
				reporter.list([
					markup`Warmup took <emphasis>${warmupTook}</emphasis>`,
					markup`<number emphasis>${String(benchmarkIterations)}</number> runs`,
					markup`<emphasis>${benchmarkTook}</emphasis> total`,
					markup`<emphasis>${benchmarkTook.divide(benchmarkIterations, true)}</emphasis> per run`,
				]);
			},
		);

		return result;
	}

	private async dispatchRequest(
		req: ServerRequest,
		origins: string[],
	): Promise<ServerQueryResponse> {
		const {query} = req;
		const {requestFlags} = query;

		if (requestFlags.benchmark && !origins.includes("benchmark")) {
			return this.dispatchBenchmarkRequest(req);
		}

		try {
			const defaultCommandFlags: Dict<unknown> = {};

			// A type-safe wrapper for retrieving command flags
			// TODO perhaps present this as JSON or something if this isn't a request from the CLI?
			const flagsConsumer = consume({
				path: createUIDPath("argv"),
				parent: undefined,
				value: query.commandFlags,
				onDefinition(def) {
					// objectPath should only have a depth of 1
					defaultCommandFlags[def.objectPath[0]] = def.default;
				},
				objectPath: [],
				context: {
					category: DIAGNOSTIC_CATEGORIES["flags/invalid"],
					getOriginalValue: () => {
						return undefined;
					},
					normalizeKey: (key) => {
						return toKebabCase(key);
					},
					getDiagnosticLocation: (keys: ConsumePath) => {
						return req.getDiagnosticLocationFromFlags({
							type: "flag",
							key: String(keys[0]),
							target: "value",
						});
					},
				},
			});

			// Get command
			const serverCommand: undefined | ServerCommand<Dict<unknown>> = serverCommands.get(
				query.commandName,
			);
			if (serverCommand) {
				await validateRequestFlags(req, serverCommand);

				let commandFlags;
				if (serverCommand.defineFlags !== undefined) {
					commandFlags = serverCommand.defineFlags(flagsConsumer);
				}

				req.setNormalizedCommandFlags({
					flags: commandFlags,
					defaultFlags: defaultCommandFlags,
				});

				// @ts-ignore
				const data = await serverCommand.callback(req, commandFlags);
				return {
					...EMPTY_SUCCESS_RESPONSE,
					hasData: data !== undefined,
					data,
				};
			} else {
				req.throwDiagnosticFlagError({
					target: {
						type: "arg",
						key: 0,
					},
					description: descriptions.FLAGS.UNKNOWN_COMMAND({
						programName: "rome",
						commandName: query.commandName,
					}),
				});
			}
		} catch (err) {
			return await req.buildResponseFromError(err);
		}
	}
}
