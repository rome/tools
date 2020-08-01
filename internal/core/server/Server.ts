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
} from "@internal/core";
import {
	DiagnosticOrigin,
	Diagnostics,
	DiagnosticsProcessor,
	createInternalDiagnostic,
	deriveDiagnosticFromError,
	descriptions,
	getDiagnosticsFromError,
} from "@internal/diagnostics";
import {ServerCommand, serverCommands} from "./commands";
import {
	DiagnosticsFileReaders,
	DiagnosticsPrinter,
	printDiagnostics,
} from "@internal/cli-diagnostics";
import {ConsumePath, consume} from "@internal/consume";
import {Event, EventSubscription} from "@internal/events";
import ServerRequest, {
	EMPTY_SUCCESS_RESPONSE,
	ServerRequestCancelled,
	ServerRequestInvalid,
} from "./ServerRequest";
import ProjectManager from "./project/ProjectManager";
import WorkerManager from "./WorkerManager";
import Resolver from "./fs/Resolver";
import FileAllocator from "./fs/FileAllocator";
import Logger from "../common/utils/Logger";
import MemoryFileSystem from "./fs/MemoryFileSystem";
import Cache from "./Cache";
import {Reporter} from "@internal/cli-reporter";
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
import {VERSION} from "../common/constants";
import setupGlobalErrorHandlers from "../common/utils/setupGlobalErrorHandlers";
import {UserConfig, loadUserConfig} from "../common/userConfig";
import {
	AbsoluteFilePath,
	createAbsoluteFilePath,
	createUnknownFilePath,
} from "@internal/path";
import {Dict, mergeObjects} from "@internal/typescript-helpers";
import LSPServer from "./lsp/LSPServer";
import ServerReporter from "./ServerReporter";
import VirtualModules from "../common/VirtualModules";
import {DiagnosticsProcessorOptions} from "@internal/diagnostics/DiagnosticsProcessor";
import {toKebabCase} from "@internal/string-utils";
import {FilePathLocker} from "../common/utils/lockers";
import {getEnvVar} from "@internal/cli-environment";
import {markup} from "@internal/markup";
import prettyFormat from "@internal/pretty-format";
import {convertPossibleNodeErrorToDiagnostic} from "@internal/node";
import RecoveryStore from "./fs/RecoveryStore";

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
	userConfig?: UserConfig;
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

		this.userConfig =
			opts.userConfig === undefined ? loadUserConfig() : opts.userConfig;

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
			onError: this.onFatalErrorBound,
		});

		this.requestStartEvent = new Event({
			name: "Server.requestStart",
			onError: this.onFatalErrorBound,
		});

		this.refreshFileEvent = new Event({
			name: "Server.refreshFile",
			onError: this.onFatalErrorBound,
		});

		this.endEvent = new Event({
			name: "Server.end",
			onError: this.onFatalErrorBound,
			serial: true,
		});

		this.logger = new Logger(
			"server",
			{
				markupOptions: {
					userConfig: this.userConfig,
					humanizeFilename: (filename) => {
						const path = createUnknownFilePath(filename);
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

	userConfig: UserConfig;

	requestStartEvent: Event<ServerRequest, void>;
	clientStartEvent: Event<ServerClient, void>;
	endEvent: Event<void, void>;

	// Event for when a file needs to be "refreshed". This could include:
	// - Deleted
	// - Created
	// - Modified
	// - Buffer updated
	refreshFileEvent: Event<AbsoluteFilePath, void>;

	onFatalErrorBound: (err: Error) => void;

	requestRunningCounter: number;
	terminateWhenIdle: boolean;

	clientIdCounter: number;

	profiling: undefined | ProfilingStartData;
	options: ServerOptions;

	recoveryStore: RecoveryStore;
	memoryFs: MemoryFileSystem;
	virtualModules: VirtualModules;
	resolver: Resolver;
	projectManager: ProjectManager;
	workerManager: WorkerManager;
	fileAllocator: FileAllocator;
	cache: Cache;
	connectedReporters: ServerReporter;
	logger: Logger;
	requestFileLocker: FilePathLocker;

	// Before we receive our first connected client we will buffer our server init logs
	// These should be relatively cheap to process since we don't do a lot
	logInitBuffer: string;

	connectedClients: Set<ServerClient>;
	connectedLSPServers: Set<LSPServer>;
	connectedClientsListeningForLogs: Set<ServerClient>;

	emitServerLog(chunk: string) {
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

	onFatalError(err: Error) {
		err = convertPossibleNodeErrorToDiagnostic(err);
		const message = markup`<emphasis>Fatal error occurred</emphasis>: ${err.stack ||
		err.message}`;
		this.logger.error(message);
		this.connectedReporters.error(message);
		process.exit();
	}

	// rome-ignore lint/ts/noExplicitAny
	wrapFatal<T extends (...args: Array<any>) => any>(callback: T): T {
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

	async handleDisconnectedDiagnostics(diagnostics: Diagnostics) {
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

	createDiagnosticsPrinterFileReaders(): DiagnosticsFileReaders {
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

	createDiagnosticsProcessor(
		opts: DiagnosticsProcessorOptions = {},
	): DiagnosticsProcessor {
		return new DiagnosticsProcessor({
			markupOptions: this.logger.markupOptions,
			...opts,
		});
	}

	createDisconnectedDiagnosticsProcessor(
		origins: Array<DiagnosticOrigin>,
	): DiagnosticsProcessor {
		return this.createDiagnosticsProcessor({
			onDiagnostics: (diagnostics: Diagnostics) => {
				this.handleDisconnectedDiagnostics(diagnostics);
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

	maybeSetupGlobalErrorHandlers() {
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

	async init() {
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

	async end() {
		this.logger.info(markup`[Server] Teardown triggered`);

		// Unwatch all project directories
		// We do this before anything else as we don't want events firing while we're in a teardown state
		this.memoryFs.unwatchAll();

		// Cancel all queries in flight
		for (const client of this.connectedClients) {
			for (const req of client.requestsInFlight) {
				await req.cancel();
			}

			// Kill socket
			client.bridge.end();
		}

		// We should remove everything that has an external dependency like a socket or process
		await this.endEvent.callOptional();
		this.workerManager.end();

		if (this.options.dedicated) {
			process.exit();
		}
	}

	async attachToBridge(bridge: ServerBridge): Promise<ServerClient> {
		let profiler: undefined | Profiler;

		// If we aren't a dedicated process then we should only expect a single connection
		// and when that ends. End the Server.
		if (this.options.dedicated === false) {
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
					await req.cancel();
				}
			}
		});

		bridge.endServer.subscribe(async () => this.end());

		await this.clientStartEvent.callOptional(client);

		return client;
	}

	async createClient(bridge: ServerBridge): Promise<ServerClient> {
		const {
			flags: rawFlags,
			streamState,
			outputFormat,
			outputSupport,
			version,
		} = await bridge.getClientInfo.call();

		// Turn the cwd back into a AbsoluteFilePath
		const flags: ClientFlags = {
			...rawFlags,
			realCwd: createAbsoluteFilePath(rawFlags.realCwd),
			cwd: createAbsoluteFilePath(rawFlags.cwd),
		};

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
				req.cancel();
			}
		});

		return client;
	}

	async handleRequestStart(req: ServerRequest) {
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

	handleRequestEnd(req: ServerRequest) {
		this.requestRunningCounter--;
		req.logger.info(markup`Request end`);

		// If we're waiting to terminate ourselves when idle, then do so when there's no running requests
		if (this.terminateWhenIdle && this.requestRunningCounter === 0) {
			this.end();
		}
	}

	async handleRequest(
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
			// Unhandled error
			await req.teardown({
				type: "ERROR",
				fatal: false,
				handled: false,
				name: err.name,
				message: err.message,
				stack: err.stack,
				markers: [],
			});
			throw err;
		} finally {
			// We no longer care if the client dies
			bridgeEndEvent.unsubscribe();
		}
	}

	async dispatchBenchmarkRequest(
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

	async dispatchRequest(
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
				filePath: createUnknownFilePath("argv"),
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
			let diagnostics: undefined | Diagnostics = await this.handleRequestError(
				req,
				err,
			);

			if (diagnostics === undefined) {
				return {
					type: "ERROR",
					fatal: false,
					handled: true,
					name: err.name,
					message: err.message,
					stack: err.stack,
					markers: [],
				};
			} else if (err instanceof ServerRequestCancelled) {
				return {
					type: "CANCELLED",
					markers: [],
				};
			} else if (err instanceof ServerRequestInvalid) {
				return {
					type: "INVALID_REQUEST",
					diagnostics,
					showHelp: err.showHelp,
					markers: [],
				};
			} else {
				return {
					type: "DIAGNOSTICS",
					files: {},
					hasDiagnostics: diagnostics.length > 0,
					diagnostics,
					markers: [],
				};
			}
		}
	}

	async handleRequestError(
		req: ServerRequest,
		rawErr: Error,
	): Promise<undefined | Diagnostics> {
		let err = rawErr;

		// If we can derive diagnostics from the error then create a diagnostics printer
		const diagnostics = getDiagnosticsFromError(err);
		if (diagnostics !== undefined) {
			const printer = req.createDiagnosticsPrinter(
				req.createDiagnosticsProcessor({
					origins: [
						{
							category: "internal",
							message: "Derived diagnostics from thrown error",
						},
					],
				}),
			);
			printer.processor.addDiagnostics(diagnostics);
			err = printer;
		}

		// Print it!
		if (err instanceof DiagnosticsPrinter) {
			const printer = err;

			// Only print when the bridge is alive and we aren't in review mode
			// When we're in review mode we don't expect to show any diagnostics because they'll be intercepted in the client command
			// We will always print invalid request errors
			let shouldPrint = true;
			if (req.query.requestFlags.review) {
				shouldPrint = false;
			}
			if (rawErr instanceof ServerRequestInvalid) {
				shouldPrint = true;
			}
			if (!req.bridge.alive) {
				shouldPrint = false;
			}

			if (shouldPrint) {
				await printer.print();

				// Don't output the footer if this is a notifier for an invalid request as it will be followed by a help screen
				if (!(rawErr instanceof ServerRequestInvalid)) {
					await printer.footer();
				}
			}

			return printer.processor.getDiagnostics();
		}

		if (!req.bridge.alive) {
			return undefined;
		}

		const printer = req.createDiagnosticsPrinter(
			req.createDiagnosticsProcessor({
				origins: [
					{
						category: "internal",
						message: "Error captured and converted into a diagnostic",
					},
				],
			}),
		);
		const errorDiag = deriveDiagnosticFromError(
			err,
			{
				description: {
					category: "internalError/request",
				},
			},
		);
		printer.processor.addDiagnostic(createInternalDiagnostic(errorDiag));
		await printer.print();

		// We could probably return printer.getDiagnostics() but we just want to print to the console
		// We will still want to send the `error` property
		return undefined;
	}
}
