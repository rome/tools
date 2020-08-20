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
	DiagnosticOrigin,
	Diagnostics,
	DiagnosticsProcessor,
	descriptions,
} from "@internal/diagnostics";
import {ServerCommand, serverCommands} from "./commands";
import {
	DiagnosticsFileReaders,
	printDiagnostics,
} from "@internal/cli-diagnostics";
import {ConsumePath, consume} from "@internal/consume";
import {Event, EventQueue, EventSubscription} from "@internal/events";
import ServerRequest, {EMPTY_SUCCESS_RESPONSE} from "./ServerRequest";
import ProjectManager from "./project/ProjectManager";
import WorkerManager from "./WorkerManager";
import Resolver from "./fs/Resolver";
import FileAllocator from "./fs/FileAllocator";
import Logger from "../common/utils/Logger";
import MemoryFileSystem from "./fs/MemoryFileSystem";
import Cache from "./Cache";
import {
	Reporter,
	ReporterProgress,
	ReporterProgressOptions,
	mergeProgresses,
} from "@internal/cli-reporter";
import {Profiler} from "@internal/v8";
import {
	PartialServerQueryRequest,
	ProfilingStartData,
} from "../common/bridges/ServerBridge";
import {
	ClientFlags,
	ClientRequestFlags,
	DEFAULT_CLIENT_REQUEST_FLAGS,
} from "../common/types/client";

import setupGlobalErrorHandlers from "../common/utils/setupGlobalErrorHandlers";

import {AbsoluteFilePath, createUnknownPath} from "@internal/path";
import {Dict, ErrorCallback, mergeObjects} from "@internal/typescript-helpers";
import LSPServer from "./lsp/LSPServer";
import ServerReporter from "./ServerReporter";
import VirtualModules from "../common/VirtualModules";
import {DiagnosticsProcessorOptions} from "@internal/diagnostics/DiagnosticsProcessor";
import {toKebabCase} from "@internal/string-utils";
import {FilePathLocker} from "../../async/lockers";
import {getEnvVar} from "@internal/cli-environment";
import {StaticMarkup, markup} from "@internal/markup";
import prettyFormat from "@internal/pretty-format";
import RecoveryStore from "./fs/RecoveryStore";
import handleFatalError from "../common/handleFatalError";

export type ServerClient = {
	id: number;
	reporter: Reporter;
	bridge: ServerBridge;
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

export default class Server {
	constructor(opts: ServerOptions) {
		this.onFatalErrorBound = this.onFatalError.bind(this);

		this.profiling = undefined;
		this.options = opts;

		this.userConfig = opts.userConfig;

		this.requestFileLocker = new FilePathLocker();

		this.connectedReporters = new ServerReporter(this);

		this.connectedClientsListeningForLogs = new Set();
		this.connectedLSPServers = new Set();
		this.connectedClients = new Set();

		this.clientIdCounter = 0;

		this.logInitBuffer = "";
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
						const path = createUnknownPath(filename);
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
					normalizePosition: (filename, line, column) => {
						const path = this.projectManager.getFilePathFromUid(filename);
						if (path === undefined) {
							return {filename, line, column};
						} else {
							return {filename: path.join(), line, column};
						}
					},
				},
			},
			{
				loggerType: "server",
				write: (chunk) => {
					this.emitServerLog(chunk);
				},
				check: () => {
					return (
						this.clientIdCounter === 0 ||
						this.connectedClientsListeningForLogs.size > 0
					);
				},
			},
		);
		this.logger.updateStream();

		this.virtualModules = new VirtualModules();
		this.recoveryStore = new RecoveryStore(this);
		this.memoryFs = new MemoryFileSystem(this);
		this.projectManager = new ProjectManager(this);
		this.workerManager = new WorkerManager(this);
		this.fileAllocator = new FileAllocator(this);
		this.resolver = new Resolver(this);
		this.cache = new Cache(this);

		this.logger.info(
			markup`[Server] Created Server with options: ${prettyFormat(opts)}`,
		);
	}

	public userConfig: UserConfig;
	public onFatalErrorBound: ErrorCallback;
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
	public cache: Cache;
	public connectedReporters: ServerReporter;
	public logger: Logger;
	public requestFileLocker: FilePathLocker;

	// Before we receive our first connected client we will buffer our server init logs
	// These should be relatively cheap to process since we don't do a lot
	private logInitBuffer: string;

	private requestRunningCounter: number;
	private terminateWhenIdle: boolean;
	private clientIdCounter: number;
	private profiling: undefined | ProfilingStartData;

	private connectedClients: Set<ServerClient>;
	private connectedLSPServers: Set<LSPServer>;
	private connectedClientsListeningForLogs: Set<ServerClient>;

	// Used when starting up child processes and indicates whether they should start profiling
	// on init
	public getRunningProfilingData(): undefined | ProfilingStartData {
		return this.profiling;
	}

	// Derive a concatenated reporter from the logger and all connected clients
	// This should only be used synchronously as the streams will not stay in sync
	// Used for very important log messages
	public getImportantReporter(): Reporter {
		return Reporter.concat([this.logger, this.connectedReporters]);
	}

	private emitServerLog(chunk: string) {
		if (this.clientIdCounter === 0) {
			this.logInitBuffer += chunk;
		}

		for (const {bridge} of this.connectedClientsListeningForLogs) {
			// Sometimes the bridge hasn't completely been teardown and we still consider it connected
			if (bridge.alive) {
				bridge.log.send({chunk, origin: "server"});
			}
		}
	}

	public onFatalError(error: Error, source: StaticMarkup = markup`server`) {
		// Ensure workers are properly ended as they could be hanging
		this.workerManager.end();

		// NB: This will call process.exit. If we want to expose this for other use-cases then we will probably want to
		// make this customizable
		handleFatalError({
			error,
			source,
			reporter: this.getImportantReporter(),
			exit: this.options.dedicated,
		});
	}

	// This is so all progress bars are renderer on each client. If we just use this.progressLocal then
	// while it would work, we would be doing all the rendering work on the server
	// The CLI also needs to know all the activeElements so it can properly draw and clear lines
	// We also create a progress bar for all connected LSP clients
	// Refer to ServerReporter
	public createConnectedProgress(opts?: ReporterProgressOptions) {
		const progresses: Array<ReporterProgress> = [];

		for (const client of this.connectedClients) {
			progresses.push(client.reporter.progress(opts));
		}

		for (const server of this.connectedLSPServers) {
			progresses.push(server.createProgress(opts));
		}

		return mergeProgresses(progresses);
	}

	public wrapFatalPromise(promise: Promise<unknown>) {
		promise.catch(this.onFatalErrorBound);
	}

	// rome-ignore lint/ts/noExplicitAny
	public wrapFatal<T extends (...args: Array<any>) => any>(callback: T): T {
		return (((...args: Array<any>): any => {
			try {
				const res = callback(...args);
				if (res instanceof Promise) {
					res.catch(this.onFatalErrorBound);
				}
				return res;
			} catch (err) {
				throw this.onFatalError(err);
			}
		}) as T);
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
				fileReaders: this.createDiagnosticsPrinterFileReaders(),
			},
		});
	}

	public createDiagnosticsPrinterFileReaders(): DiagnosticsFileReaders {
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

			getMtime: async (path) => {
				const virtualContents = this.virtualModules.getPossibleVirtualFileContents(
					path,
				);
				if (virtualContents === undefined) {
					return undefined;
				} else {
					return this.memoryFs.getMtime(path);
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
		origins: Array<DiagnosticOrigin>,
	): DiagnosticsProcessor {
		return this.createDiagnosticsProcessor({
			onDiagnostics: (diagnostics: Diagnostics) => {
				this.wrapFatalPromise(this.handleDisconnectedDiagnostics(diagnostics));
			},
			origins: [
				...origins,
				{
					category: "server",
					message: "Created disconnected diagnostics collector",
				},
			],
		});
	}

	private maybeSetupGlobalErrorHandlers() {
		if (!this.options.globalErrorHandlers) {
			return;
		}

		const teardown = setupGlobalErrorHandlers((err) => {
			this.onFatalError(err);
		});

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
			client.bridge.end();
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

	public async attachToBridge(bridge: ServerBridge): Promise<ServerClient> {
		let profiler: undefined | Profiler;

		// If we aren't a dedicated process then we should only expect a single connection
		// and when that ends. End the Server.
		if (!this.options.dedicated) {
			bridge.endEvent.subscribe(() => {
				this.end();
			});
		}

		bridge.profilingStart.subscribe(async (data) => {
			if (profiler !== undefined) {
				throw new Error("Expected no profiler to be running");
			}
			profiler = new Profiler();
			await profiler.startProfiling(data.samplingInterval);
			this.profiling = data;
			for (const {bridge} of this.workerManager.getExternalWorkers()) {
				await bridge.profilingStart.call(data);
			}
		});

		bridge.profilingStop.subscribe(async () => {
			if (profiler === undefined) {
				throw new Error("Expected profiler to be running");
			}
			const serverProfile = await profiler.stopProfiling();
			profiler = undefined;
			this.profiling = undefined;
			return serverProfile;
		});

		bridge.profilingGetWorkers.subscribe(async () => {
			const ids: Array<number> = [];
			for (const {id} of this.workerManager.getExternalWorkers()) {
				ids.push(id);
			}
			return ids;
		});

		bridge.profilingStopWorker.subscribe(async (id) => {
			const worker = this.workerManager.getWorkerAssert(id);
			return await worker.bridge.profilingStop.call();
		});

		// When enableWorkerLogs is called we setup subscriptions to the worker logs
		// Logs are never transported from workers to the server unless there is a subscription
		let subscribedWorkers = false;
		bridge.enableWorkerLogs.subscribe(() => {
			// enableWorkerLogs could be called twice in the case of `--logs --rage`. We'll only want to setup the subscriptions once
			if (subscribedWorkers) {
				return;
			} else {
				subscribedWorkers = true;
			}

			function onLog(chunk: string) {
				bridge.log.call({origin: "worker", chunk});
			}

			// Add on existing workers if there are any
			for (const worker of this.workerManager.getWorkers()) {
				bridge.attachEndSubscriptionRemoval(worker.bridge.log.subscribe(onLog));
			}

			// Listen for logs for any workers that start later
			this.workerManager.workerStartEvent.subscribe((worker) => {
				bridge.attachEndSubscriptionRemoval(worker.log.subscribe(onLog));
			});
		});

		await bridge.handshake();

		const client = await this.createClient(bridge);

		if (client.version !== VERSION) {
			// TODO this wont ever actually be printed?
			client.reporter.error(
				markup`Client version ${client.version} does not match server version ${VERSION}. Goodbye lol.`,
			);
			client.bridge.end();
			return client;
		}

		bridge.query.subscribe(async (request) => {
			return await this.handleRequest(client, request);
		});

		bridge.cancelQuery.subscribe(async (token) => {
			for (const req of client.requestsInFlight) {
				if (req.query.cancelToken === token) {
					await req.cancel("user requested");
				}
			}
		});

		bridge.endServer.subscribe(async () => this.end());

		await this.clientStartEvent.callOptional(client);
		await bridge.serverReady.call();

		return client;
	}

	private async createClient(bridge: ServerBridge): Promise<ServerClient> {
		const {
			flags,
			streamState,
			outputFormat,
			outputSupport,
			version,
		} = await bridge.getClientInfo.call();

		// Initialize the reporter
		const reporter = new Reporter({
			wrapperFactory: this.wrapFatal.bind(this),
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

					bridge.write.send([chunk, error]);
				},
			},
			streamState,
		);

		bridge.updateFeatures.subscribe((features) => {
			streamHandle.stream.updateFeatures(features);
		});

		// Streams to teardown on client disconnect
		const streamHandles = [streamHandle];

		// Add reporter to connected set, important logs may be output to these
		streamHandles.push(
			this.connectedReporters.addAttachedStream(streamHandle.stream),
		);

		// Warn about disabled disk caching. Don't bother if it's only been set due to ROME_DEV. We don't care to see it in development.
		if (this.cache.disabled && getEnvVar("ROME_DEV").type !== "ENABLED") {
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

		bridge.updatedListenersEvent.subscribe((listeners) => {
			if (listeners.has("log")) {
				if (!this.connectedClientsListeningForLogs.has(client)) {
					this.connectedClientsListeningForLogs.add(client);
					let buffer = this.logInitBuffer;
					buffer += ".".repeat(20);
					buffer += "\n";
					bridge.log.send({
						chunk: buffer,
						origin: "server",
					});
				}
			} else {
				this.connectedClientsListeningForLogs.delete(client);
			}
			this.logger.updateStream();
		});

		bridge.endEvent.subscribe(() => {
			this.connectedClients.delete(client);
			this.connectedClientsListeningForLogs.delete(client);
			for (const handle of streamHandles) {
				handle.remove();
			}
			this.logger.updateStream();

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
			await this.onFatalErrorBound(err);
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
		origins: Array<string>,
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
				filePath: createUnknownPath("argv"),
				parent: undefined,
				value: query.commandFlags,
				onDefinition(def) {
					// objectPath should only have a depth of 1
					defaultCommandFlags[def.objectPath[0]] = def.default;
				},
				objectPath: [],
				context: {
					category: "flags/invalid",
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
