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
import {
	BridgeServer,
	Event,
	EventQueue,
	EventSubscription,
} from "@internal/events";
import ServerRequest, {EMPTY_SUCCESS_RESPONSE} from "./ServerRequest";
import ProjectManager from "./project/ProjectManager";
import WorkerManager from "./WorkerManager";
import Resolver from "./fs/Resolver";
import FileAllocator from "./fs/FileAllocator";
import Logger from "../common/utils/Logger";
import MemoryFileSystem from "./fs/MemoryFileSystem";
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
} from "../common/bridges/ServerBridge";
import {
	ClientFlags,
	ClientLogsLevel,
	ClientRequestFlags,
	DEFAULT_CLIENT_REQUEST_FLAGS,
} from "../common/types/client";
import {AbsoluteFilePath, createAnyPath, createUIDPath} from "@internal/path";
import {Dict, mergeObjects} from "@internal/typescript-helpers";
import LSPServer from "./lsp/LSPServer";
import ServerReporter from "./ServerReporter";
import VirtualModules from "../common/VirtualModules";
import {DiagnosticsProcessorOptions} from "@internal/diagnostics/DiagnosticsProcessor";
import {toKebabCase} from "@internal/string-utils";
import {FilePathLocker} from "../../async/lockers";
import {DEFAULT_TERMINAL_FEATURES, getEnvVar} from "@internal/cli-environment";
import {markup} from "@internal/markup";
import prettyFormat from "@internal/pretty-format";
import RecoveryStore from "./fs/RecoveryStore";
import WorkerQueue, {WorkerQueueOptions} from "./WorkerQueue";
import FatalErrorHandler from "../common/FatalErrorHandler";

export type ServerClient = {
	id: number;
	reporter: Reporter;
	bridge: BridgeServer<typeof ServerBridge>;
	flags: ClientFlags;
	version: string;
	requestsInFlight: Set<ServerRequest>;
};

export type ServerOptions = {
	inbandOnly?: boolean;
	forceCacheEnabled?: boolean;
	userConfig: UserConfig;
	dedicated: boolean;
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
		terminateWhenIdle: partialQuery.terminateWhenIdle === true,
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
	if (!bridge.alive) {
		return;
	}

	if (log.isError || level === "all") {
		bridge.events.log.send(log);
	}
}

export default class Server {
	constructor(opts: ServerOptions) {
		this.profiling = undefined;
		this.options = opts;

		this.userConfig = opts.userConfig;

		this.fatalErrorHandler = new FatalErrorHandler({
			getOptions: () => {
				// Ensure workers are properly ended as they could be hanging
				this.workerManager.end();

				return {
					source: markup`server`,
					reporter: this.getImportantReporter(),
					exit: this.options.dedicated,
				};
			},
		});

		this.requestFileLocker = new FilePathLocker();

		this.connectedReporters = new ServerReporter(this);

		this.connectedClientsListeningForWorkerLogs = new Set();
		this.connectedClientsListeningForLogs = new Map();
		this.connectedLSPServers = new Set();
		this.connectedClients = new Set();

		this.hadConnectedClient = false;
		this.clientIdCounter = 0;

		this.logInitBuffer = [];
		this.requestRunningCounter = 0;
		this.terminateWhenIdle = false;

		this.clientStartEvent = new Event({
			name: "Server.clientStart",
		});

		this.requestStartEvent = new Event({
			name: "Server.requestStart",
		});

		this.refreshFileEvent = new EventQueue();

		this.endEvent = new Event({
			name: "Server.end",
			serial: true,
		});

		this.logger = new Logger(
			{
				markupOptions: {
					userConfig: this.userConfig,
					humanizeFilename: (filename) => {
						const path = createAnyPath(filename);
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
						const normalPath = this.projectManager.maybeGetFilePathFromUid(path);
						if (normalPath === undefined) {
							return {path, line, column};
						} else {
							return {path: normalPath, line, column};
						}
					},
				},
			},
			"server",
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

	// Public events
	public requestStartEvent: Event<ServerRequest, void>;
	public clientStartEvent: Event<ServerClient, void>;
	public endEvent: Event<void, void>;

	// Event for when a file needs to be "refreshed". This could include:
	// - Deleted
	// - Created
	// - Modified
	// - Buffer updated
	public refreshFileEvent: EventQueue<AbsoluteFilePath>;

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
	public requestFileLocker: FilePathLocker;

	private loggerStream: ReporterConditionalStream;

	// Before we receive our first connected client we will buffer our server init logs
	// These __should__ be relatively cheap to retain since we don't do a lot
	private logInitBuffer: [string, boolean][];

	private requestRunningCounter: number;
	private terminateWhenIdle: boolean;
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
	// This should only be used synchronously as the streams will not stay in sync
	// Used for very important log messages
	public getImportantReporter(): Reporter {
		return Reporter.concat([this.logger, this.connectedReporters]);
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
		this.connectedReporters.error(
			markup`Generated diagnostics without a current request`,
		);

		await printDiagnostics({
			diagnostics,
			suppressions: [],
			printerOptions: {
				processor: this.createDiagnosticsProcessor(),
				reporter: this.connectedReporters,
				fileHandlers: [this.createDiagnosticsPrinterFileHandler()],
			},
		});
	}

	public createDiagnosticsPrinterFileHandler(): DiagnosticsFileHandler {
		return {
			readAbsolute: async (path) => {
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
				if (this.virtualModules.isVirtualPath(path)) {
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
		if (!this.options.globalErrorHandlers) {
			return;
		}

		const teardown = this.fatalErrorHandler.setupGlobalHandlers();

		this.endEvent.subscribe(() => {
			teardown();
		});
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

		// We should remove everything that has an external dependency like a socket or process
		await this.endEvent.callOptional();
		await this.workerManager.end();

		if (this.options.dedicated) {
			process.exit();
		}
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
		if (!this.hadConnectedClient) {
			this.hadConnectedClient = true;
			this.loggerStream.update();
		}

		let profiler: undefined | Profiler;

		// If we aren't a dedicated process then we should only expect a single connection
		// and when that ends. End the Server.
		if (!this.options.dedicated) {
			bridge.endEvent.subscribe(async () => {
				await this.end();
			});
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
			const ids: number[] = [];
			for (const {id} of this.workerManager.getExternalWorkers()) {
				ids.push(id);
			}
			return ids;
		});

		bridge.events.profilingStopWorker.subscribe(async (id) => {
			const worker = this.workerManager.getWorkerAssert(id);
			return await worker.bridge.events.profilingStop.call();
		});

		await bridge.handshake();

		const client = await this.createClient(bridge);

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
		bridge: BridgeServer<typeof ServerBridge>,
	): Promise<ServerClient> {
		const {
			flags,
			streamState,
			outputFormat,
			outputSupport,
			version,
		} = await bridge.events.getClientInfo.call();

		// Initialize the reporter
		const reporter = new Reporter({
			wrapperFactory: this.fatalErrorHandler.wrapBound,
			markupOptions: {
				...this.logger.markupOptions,
				cwd: flags.cwd,
			},
		});

		const streamHandle = reporter.addStream(
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
			streamHandle.stream.updateFeatures(features);
		});

		// Streams to teardown on client disconnect
		const streamHandles = [streamHandle];

		// Add reporter to connected set, important logs may be output to these
		streamHandles.push(
			this.connectedReporters.addAttachedStream(streamHandle.stream),
		);

		// Warn about disabled disk caching. Don't bother if it's only been set due to ROME_DEV. We don't care to see it in development.
		if (this.cache.writeDisabled && getEnvVar("ROME_DEV").type !== "ENABLED") {
			reporter.warn(
				markup`Disk caching has been disabled due to the <emphasis>ROME_CACHE=0</emphasis> environment variable`,
			);
		}

		const client: ServerClient = {
			id: this.clientIdCounter++,
			bridge,
			reporter,
			flags,
			version,
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
			for (const handle of streamHandles) {
				handle.remove();
			}
			this.loggerStream.update();

			// Cancel any requests still in flight
			for (const req of client.requestsInFlight) {
				req.cancel("bridge died");
			}

			// Teardown reporter
			client.reporter.teardown();
		});

		return client;
	}

	public async handleRequestStart(req: ServerRequest) {
		req.logger.info(markup`Start ${prettyFormat(req.query)}`);

		// Hook used by the web server to track and collect server requests
		await this.requestStartEvent.callOptional(req);

		// Track the amount of running queries for terminateWhenIdle
		this.requestRunningCounter++;

		// Sometimes we'll want to terminate the process once all queries have finished
		if (req.query.terminateWhenIdle) {
			this.terminateWhenIdle = true;
		}
	}

	public async handleRequestEnd(req: ServerRequest) {
		this.requestRunningCounter--;
		req.logger.info(markup`Request end`);

		// If we're waiting to terminate ourselves when idle, then do so when there's no running requests
		if (this.terminateWhenIdle && this.requestRunningCounter === 0) {
			await this.end();
		}
	}

	public async handleRequest(
		client: ServerClient,
		partialQuery: PartialServerQueryRequest,
	): Promise<ServerQueryResponse> {
		const query: ServerQueryRequest = partialServerQueryRequestToFull(
			partialQuery,
		);

		const {bridge} = client;

		// Create a promise for the client dying so we can race it later
		let bridgeEndEvent: undefined | EventSubscription;
		const bridgeEndPromise: Promise<void> = new Promise((resolve, reject) => {
			bridgeEndEvent = bridge.endEvent.subscribe((err) => {
				reject(err);
			});
		});
		if (bridgeEndEvent === undefined) {
			throw new Error("Expected bridgeEndEvent to have been initialized");
		}

		const req = new ServerRequest({
			client,
			query,
			server: this,
		});

		await req.init();

		try {
			let res: undefined | ServerQueryResponse = await this.dispatchRequest(
				req,
				bridgeEndPromise,
				[],
			);

			res = await req.teardown(res);

			if (res === undefined) {
				throw new Error(
					"teardown should have returned a normalized ServerQueryResponse",
				);
			}

			return res;
		} catch (err) {
			await this.fatalErrorHandler.handleAsync(err);
			throw new Error("Should never meet this condition");
		} finally {
			// We no longer care if the client dies
			await bridgeEndEvent.unsubscribe();
		}
	}

	private async dispatchBenchmarkRequest(
		req: ServerRequest,
		bridgeEndPromise: Promise<void>,
	): Promise<ServerQueryResponse> {
		const {client} = req;
		const {reporter} = client;
		const {benchmarkIterations} = req.query.requestFlags;

		// Warmup
		const warmupStart = Date.now();
		const result = await this.dispatchRequest(
			req,
			bridgeEndPromise,
			["benchmark"],
		);
		const warmupTook = Date.now() - warmupStart;

		// Benchmark
		const progress = req.reporter.progress({title: markup`Running benchmark`});
		progress.setTotal(benchmarkIterations);
		const benchmarkStart = Date.now();
		for (let i = 0; i < benchmarkIterations; i++) {
			await this.dispatchRequest(req, bridgeEndPromise, ["benchmark"]);
			progress.tick();
		}
		progress.end();
		const benchmarkTook = Date.now() - benchmarkStart;

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
					markup`Warmup took <duration emphasis>${String(warmupTook)}</duration>`,
					markup`<number emphasis>${String(benchmarkIterations)}</number> runs`,
					markup`<duration emphasis>${String(benchmarkTook)}</duration> total`,
					markup`<duration emphasis approx>${String(
						benchmarkTook / benchmarkIterations,
					)}</duration> per run`,
				]);
			},
		);

		return result;
	}

	private async dispatchRequest(
		req: ServerRequest,
		bridgeEndPromise: Promise<void>,
		origins: string[],
	): Promise<ServerQueryResponse> {
		const {query} = req;
		const {requestFlags} = query;

		if (requestFlags.benchmark && !origins.includes("benchmark")) {
			return this.dispatchBenchmarkRequest(req, bridgeEndPromise);
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

			// An array of promises that we'll race, the only promise that will ever resolve will be the command one
			let promises: Array<Promise<unknown> | undefined> = [bridgeEndPromise];

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
				const commandPromise = serverCommand.callback(req, commandFlags);
				promises.push(commandPromise);

				await Promise.race(promises);

				// Only the command promise should have won the race with a resolve
				const data = await commandPromise;
				return {
					...EMPTY_SUCCESS_RESPONSE,
					hasData: data !== undefined,
					data,
				};
			} else {
				throw new Error(`Unknown command ${String(query.commandName)}`);
			}
		} catch (err) {
			return await req.buildResponseFromError(err);
		}
	}
}
