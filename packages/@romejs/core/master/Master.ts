/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	MasterBridge,
	MasterQueryRequest,
	MasterQueryResponse,
} from '@romejs/core';
import {
	DiagnosticOrigin,
	Diagnostics,
	DiagnosticsProcessor,
	INTERNAL_ERROR_LOG_ADVICE,
	deriveDiagnosticFromError,
	descriptions,
	getDiagnosticsFromError,
} from '@romejs/diagnostics';
import {MasterCommand, masterCommands} from './commands';
import {
	DiagnosticsFileReader,
	DiagnosticsPrinter,
	printDiagnostics,
	readDiagnosticsFileLocal,
} from '@romejs/cli-diagnostics';
import {ConsumePath, consume} from '@romejs/consume';
import {Event, EventSubscription} from '@romejs/events';
import MasterRequest, {
	EMPTY_SUCCESS_RESPONSE,
	MasterRequestCancelled,
	MasterRequestInvalid,
} from './MasterRequest';
import ProjectManager from './project/ProjectManager';
import WorkerManager from './WorkerManager';
import Resolver from './fs/Resolver';
import FileAllocator from './fs/FileAllocator';
import Logger from '../common/utils/Logger';
import MemoryFileSystem from './fs/MemoryFileSystem';
import Cache from './Cache';
import {Reporter, ReporterStream} from '@romejs/cli-reporter';
import {Profiler} from '@romejs/v8';
import {
	PartialMasterQueryRequest,
	ProfilingStartData,
} from '../common/bridges/MasterBridge';
import {
	ClientFlags,
	ClientRequestFlags,
	DEFAULT_CLIENT_REQUEST_FLAGS,
} from '../common/types/client';
import {VERSION} from '../common/constants';
import {escapeMarkup} from '@romejs/string-markup';
import setupGlobalErrorHandlers from '../common/utils/setupGlobalErrorHandlers';
import {UserConfig, loadUserConfig} from '../common/userConfig';
import {
	AbsoluteFilePath,
	createAbsoluteFilePath,
	createUnknownFilePath,
} from '@romejs/path';
import {Dict} from '@romejs/typescript-helpers';
import LSPServer from './lsp/LSPServer';
import MasterReporter from './MasterReporter';
import VirtualModules from './fs/VirtualModules';
import {DiagnosticsProcessorOptions} from '@romejs/diagnostics/DiagnosticsProcessor';
import {toKebabCase} from '@romejs/string-utils';

const STDOUT_MAX_CHUNK_LENGTH = 100_000;

export type MasterClient = {
	id: number;
	reporter: Reporter;
	bridge: MasterBridge;
	flags: ClientFlags;
	version: string;
	requestsInFlight: Set<MasterRequest>;
};

export type MasterOptions = {
	dedicated: boolean;
	globalErrorHandlers: boolean;
};

export type MasterUnfinishedMarker = {
	// Text label to be associated with the timeline entry
	label: string;

	// Start time in milliseconds
	start: number;

	// Process / Thread that the events are associated with
	rowId: string;

	//
	facet: string;
};

export type MasterMarker = MasterUnfinishedMarker & {
	// End time in milliseconds
	end: number;
};

const disallowedFlagsWhenReviewing: Array<keyof ClientRequestFlags> = ['watch'];

async function validateRequestFlags(
	req: MasterRequest,
	masterCommand: MasterCommand<Dict<unknown>>,
) {
	const {requestFlags} = req.query;

	// Commands need to explicitly allow these flags
	validateAllowedRequestFlag(req, 'watch', masterCommand);
	validateAllowedRequestFlag(req, 'review', masterCommand);

	// Don't allow review in combination with other flags
	if (requestFlags.review) {
		for (const key of disallowedFlagsWhenReviewing) {
			if (requestFlags[key]) {
				throw req.throwDiagnosticFlagError({
					description: descriptions.FLAGS.DISALLOWED_REVIEW_FLAG(key),
					target: {type: 'flag', key},
				});
			}
		}
	}
}

function validateAllowedRequestFlag(
	req: MasterRequest,
	flagKey: NonNullable<MasterCommand<Dict<unknown>>['allowRequestFlags']>[number],
	masterCommand: MasterCommand<Dict<unknown>>,
) {
	const allowRequestFlags = masterCommand.allowRequestFlags || [];
	if (req.query.requestFlags[flagKey] && !allowRequestFlags.includes(flagKey)) {
		throw req.throwDiagnosticFlagError({
			description: descriptions.FLAGS.DISALLOWED_REQUEST_FLAG(flagKey),
			target: {type: 'flag', key: flagKey},
		});
	}
}

export default class Master {
	constructor(opts: MasterOptions) {
		this.onFatalErrorBound = this.onFatalError.bind(this);

		this.profiling = undefined;
		this.options = opts;

		this.userConfig = loadUserConfig();

		this.fileChangeEvent = new Event({
			name: 'Master.fileChange',
			onError: this.onFatalErrorBound,
		});

		this.clientStartEvent = new Event({
			name: 'Master.clientStart',
			onError: this.onFatalErrorBound,
		});

		this.requestStartEvent = new Event({
			name: 'Master.requestStart',
			onError: this.onFatalErrorBound,
		});

		this.logEvent = new Event({
			name: 'Master.log',
			onError: this.onFatalErrorBound,
		});

		this.endEvent = new Event({
			name: 'Master.end',
			onError: this.onFatalErrorBound,
			serial: true,
		});

		this.logger = new Logger(
			'master',
			() => {
				return (
					this.logEvent.hasSubscribers() ||
					this.connectedClientsListeningForLogs.size > 0
				);
			},
			{
				streams: [
					{
						type: 'all',
						format: 'none',
						columns: 0,
						unicode: true,
						write: (chunk) => {
							this.emitMasterLog(chunk);
						},
					},
				],
				markupOptions: {
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
					normalizeFilename: (filename: string): string => {
						const path = this.projectManager.getFilePathFromUid(filename);
						if (path === undefined) {
							return filename;
						} else {
							return path.join();
						}
					},
				},
			},
		);

		this.connectedReporters = new MasterReporter(this);

		this.connectedClientsListeningForLogs = new Set();
		this.connectedLSPServers = new Set();
		this.connectedClients = new Set();

		this.virtualModules = new VirtualModules(this);
		this.memoryFs = new MemoryFileSystem(this);
		this.projectManager = new ProjectManager(this);
		this.workerManager = new WorkerManager(this);
		this.fileAllocator = new FileAllocator(this);
		this.resolver = new Resolver(this);
		this.cache = new Cache(this);

		this.memoryFs.deletedFileEvent.subscribe((path) => {
			return this.handleFileDelete(path);
		});

		this.memoryFs.changedFileEvent.subscribe(({path}) => {
			return this.handleFileChange(path);
		});

		this.warnedCacheClients = new WeakSet();

		this.clientIdCounter = 0;

		this.requestRunningCounter = 0;
		this.terminateWhenIdle = false;
	}

	userConfig: UserConfig;

	requestStartEvent: Event<MasterRequest, void>;
	clientStartEvent: Event<MasterClient, void>;
	fileChangeEvent: Event<AbsoluteFilePath, void>;
	logEvent: Event<string, void>;
	endEvent: Event<void, void>;

	onFatalErrorBound: (err: Error) => void;

	requestRunningCounter: number;
	terminateWhenIdle: boolean;

	clientIdCounter: number;

	profiling: undefined | ProfilingStartData;
	options: MasterOptions;

	warnedCacheClients: WeakSet<MasterBridge>;
	memoryFs: MemoryFileSystem;
	virtualModules: VirtualModules;
	resolver: Resolver;
	projectManager: ProjectManager;
	workerManager: WorkerManager;
	fileAllocator: FileAllocator;
	cache: Cache;
	connectedReporters: MasterReporter;
	logger: Logger;

	connectedClients: Set<MasterClient>;
	connectedLSPServers: Set<LSPServer>;
	connectedClientsListeningForLogs: Set<MasterClient>;

	emitMasterLog(chunk: string) {
		this.logEvent.send(chunk);

		for (const {bridge} of this.connectedClientsListeningForLogs) {
			bridge.log.send({chunk, origin: 'master'});
		}
	}

	onFatalError(err: Error) {
		const message = `<emphasis>Fatal error occurred</emphasis>: ${escapeMarkup(
			err.stack || err.message,
		)}`;
		this.logger.error(message);
		this.connectedReporters.error(message);
		process.exit();
	}

	// rome-ignore lint/noExplicitAny
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
			'Generated diagnostics without a current request',
		);

		printDiagnostics({
			diagnostics,
			suppressions: [],
			printerOptions: {
				processor: this.createDiagnosticsProcessor(),
				reporter: this.connectedReporters,
				readFile: this.readDiagnosticsPrinterFile.bind(this),
			},
		});
	}

	readDiagnosticsPrinterFile(
		path: AbsoluteFilePath,
	): ReturnType<DiagnosticsFileReader> {
		const remoteToLocal = this.projectManager.remoteToLocalPath.get(path);

		if (remoteToLocal === undefined) {
			return readDiagnosticsFileLocal(path);
		} else {
			return readDiagnosticsFileLocal(remoteToLocal);
		}
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
					category: 'master',
					message: 'Created disconnected diagnostics collector',
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
		this.memoryFs.init();
		await this.projectManager.init();
		this.fileAllocator.init();
		this.resolver.init();
		await this.cache.init();
		await this.virtualModules.init();
		await this.workerManager.init();
	}

	async end() {
		// Cancel all queries in flight
		for (const client of this.connectedClients) {
			for (const req of client.requestsInFlight) {
				req.cancel();
			}
		}

		// We should remove everything that has an external dependency like a socket or process
		await this.endEvent.callOptional();
		this.workerManager.end();
		this.memoryFs.unwatchAll();
	}

	async attachToBridge(bridge: MasterBridge) {
		let profiler: undefined | Profiler;

		// If we aren't a dedicated process then we should only expect a single connection
		// and when that ends. End the Master.
		if (this.options.dedicated === false) {
			bridge.endEvent.subscribe(() => {
				this.end();
			});
		}

		bridge.profilingStart.subscribe(async (data) => {
			if (profiler !== undefined) {
				throw new Error('Expected no profiler to be running');
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
				throw new Error('Expected profiler to be running');
			}
			const masterProfile = await profiler.stopProfiling();
			profiler = undefined;
			this.profiling = undefined;
			return masterProfile;
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
		// Logs are never transported from workers to the master unless there is a subscription
		let subscribedWorkers = false;
		bridge.enableWorkerLogs.subscribe(() => {
			// enableWorkerLogs could be called twice in the case of `--logs --rage`. We'll only want to setup the subscriptions once
			if (subscribedWorkers) {
				return;
			} else {
				subscribedWorkers = true;
			}

			function onLog(chunk: string) {
				bridge.log.call({origin: 'worker', chunk});
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
				`Client version ${client.version} does not match server version ${VERSION}. Goodbye lol.`,
			);
			client.bridge.end();
			return;
		}

		bridge.query.subscribe(async (request) => {
			return await this.handleRequest(client, request);
		});

		bridge.cancelQuery.subscribe(async (token) => {
			for (const req of client.requestsInFlight) {
				if (req.query.cancelToken === token) {
					req.cancel();
				}
			}
		});

		await this.clientStartEvent.callOptional(client);
	}

	async createClient(bridge: MasterBridge): Promise<MasterClient> {
		const {
			flags: rawFlags,
			useRemoteReporter,
			hasClearScreen,
			columns,
			unicode,
			format,
			version,
		} = await bridge.getClientInfo.call();

		// Turn the cwd back into a AbsoluteFilePath
		const flags: ClientFlags = {
			...rawFlags,
			cwd: createAbsoluteFilePath(rawFlags.cwd),
		};

		const outStream: ReporterStream = {
			type: 'out',
			columns,
			format,
			unicode,
			write(chunk: string) {
				if (flags.silent === true) {
					return;
				}

				// Split up stdout chunks
				if (chunk.length < STDOUT_MAX_CHUNK_LENGTH) {
					bridge.stdout.send(chunk);
				} else {
					while (chunk.length > 0) {
						const subChunk = chunk.slice(0, STDOUT_MAX_CHUNK_LENGTH);
						chunk = chunk.slice(STDOUT_MAX_CHUNK_LENGTH);
						bridge.stdout.send(subChunk);
					}
				}
			},
		};

		const errStream: ReporterStream = {
			...outStream,
			type: 'error',
			write(chunk: string) {
				bridge.stderr.send(chunk);
			},
		};

		bridge.setColumns.subscribe((columns) => {
			reporter.setStreamColumns([outStream, errStream], columns);
		});

		// Initialize the reporter
		const reporter = new Reporter({
			hasClearScreen,
			wrapperFactory: this.wrapFatal.bind(this),
			streams: [outStream, errStream],
			verbose: flags.verbose,
			markupOptions: {
				cwd: flags.cwd,
				...this.logger.markupOptions,
			},
			useRemoteProgressBars: useRemoteReporter,
		});

		reporter.sendRemoteClientMessage.subscribe((msg) => {
			bridge.reporterRemoteServerMessage.send(msg);
		});

		bridge.reporterRemoteClientMessage.subscribe((msg) => {
			reporter.receivedRemoteServerMessage(msg);
		});

		// Add reporter to connected set, important logs may be output to these
		if (!flags.silent) {
			this.connectedReporters.addStream(outStream);
		}
		this.connectedReporters.addStream(errStream);

		const client: MasterClient = {
			id: this.clientIdCounter++,
			bridge,
			reporter,
			flags,
			version,
			requestsInFlight: new Set(),
		};

		this.connectedClients.add(client);

		bridge.updatedListenersEvent.subscribe((listeners) => {
			if (listeners.has('log')) {
				this.connectedClientsListeningForLogs.add(client);
			} else {
				this.connectedClientsListeningForLogs.delete(client);
			}
		});

		bridge.endEvent.subscribe(() => {
			// Cancel any requests still in flight
			for (const req of client.requestsInFlight) {
				req.cancel();
			}

			this.connectedClients.delete(client);
			this.connectedClientsListeningForLogs.delete(client);
			this.connectedReporters.removeStream(errStream);
			this.connectedReporters.removeStream(outStream);
		});

		return client;
	}

	async handleFileDelete(path: AbsoluteFilePath) {
		this.logger.info(`[Master] File delete:`, path.join());
		this.fileChangeEvent.send(path);
	}

	async handleFileChange(path: AbsoluteFilePath) {
		this.logger.info(`[Master] File change:`, path.join());
		this.fileChangeEvent.send(path);
	}

	async handleRequestStart(req: MasterRequest) {
		this.logger.info(`[Master] Handling CLI request:`, req.query);

		// Hook used by the web server to track and collect master requests
		await this.requestStartEvent.callOptional(req);

		// Track the amount of running queries for terminateWhenIdle
		this.requestRunningCounter++;

		// Sometimes we'll want to terminate the process once all queries have finished
		if (req.query.terminateWhenIdle) {
			this.terminateWhenIdle = true;
		}
	}

	handleRequestEnd(req: MasterRequest) {
		this.requestRunningCounter--;
		this.logger.info(`[Master] Replying to CLI request:`, req.query);

		// If we're waiting to terminate ourselves when idle, then do so when there's no running requests
		if (this.terminateWhenIdle && this.requestRunningCounter === 0) {
			this.end();
		}
	}

	async handleRequest(
		client: MasterClient,
		partialQuery: PartialMasterQueryRequest,
	): Promise<MasterQueryResponse> {
		const requestFlags: ClientRequestFlags = {
			...DEFAULT_CLIENT_REQUEST_FLAGS,
			...partialQuery.requestFlags,
		};

		const query: MasterQueryRequest = {
			commandName: partialQuery.commandName,
			args: partialQuery.args === undefined ? [] : partialQuery.args,
			noData: partialQuery.noData === true,
			requestFlags,
			silent: partialQuery.silent === true || requestFlags.benchmark,
			terminateWhenIdle: partialQuery.terminateWhenIdle === true,
			commandFlags: partialQuery.commandFlags === undefined
				? {}
				: partialQuery.commandFlags,
			cancelToken: partialQuery.cancelToken,
		};

		const {bridge} = client;

		// Create a promise for the client dying so we can race it later
		let bridgeEndEvent: undefined | EventSubscription;
		const bridgeEndPromise: Promise<void> = new Promise((resolve, reject) => {
			bridgeEndEvent = bridge.endEvent.subscribe((err) => {
				reject(err);
			});
		});
		if (bridgeEndEvent === undefined) {
			throw new Error('Expected bridgeEndEvent to have been initialized');
		}

		// Support a silent option on requests so they don't write output
		let reporter = client.reporter;
		if (query.silent) {
			reporter = reporter.fork({
				streams: [],
			});
		}

		const req = new MasterRequest({
			client,
			query,
			master: this,
		});

		await req.init();

		try {
			let res: undefined | MasterQueryResponse = await this.dispatchRequest(
				req,
				bridgeEndPromise,
				[],
			);

			res = req.teardown(res);

			if (res === undefined) {
				throw new Error(
					'teardown should have returned a normalized MasterQueryResponse',
				);
			}

			return res;
		} catch (err) {
			req.teardown(undefined);
			throw err;
		} finally {
			// We no longer care if the client dies
			bridgeEndEvent.unsubscribe();
		}
	}

	async dispatchBenchmarkRequest(
		req: MasterRequest,
		bridgeEndPromise: Promise<void>,
	): Promise<MasterQueryResponse> {
		const {client} = req;
		const {reporter} = client;
		const {benchmarkIterations} = req.query.requestFlags;

		// Warmup
		const warmupStart = Date.now();
		const result = await this.dispatchRequest(
			req,
			bridgeEndPromise,
			['benchmark'],
		);
		const warmupTook = Date.now() - warmupStart;

		// Benchmark
		const progress = client.reporter.progress({title: 'Running benchmark'});
		progress.setTotal(benchmarkIterations);
		const benchmarkStart = Date.now();
		for (let i = 0; i < benchmarkIterations; i++) {
			await this.dispatchRequest(req, bridgeEndPromise, ['benchmark']);
			progress.tick();
		}
		progress.end();
		const benchmarkTook = Date.now() - benchmarkStart;

		reporter.section(
			'Benchmark results',
			() => {
				reporter.info(
					'Request artifacts may have been cached after the first run, artificially decreasing subsequent run time',
				);
				reporter.heading('Query');
				reporter.inspect(req.query);
				reporter.heading('Stats');
				reporter.list([
					`Warmup took <duration emphasis>${warmupTook}</duration>`,
					`<number emphasis>${benchmarkIterations}</number> runs`,
					`<duration emphasis>${benchmarkTook}</duration> total`,
					`<duration emphasis approx>${benchmarkTook / benchmarkIterations}</duration> per run`,
				]);
			},
		);

		return result;
	}

	async dispatchRequest(
		req: MasterRequest,
		bridgeEndPromise: Promise<void>,
		origins: Array<string>,
	): Promise<MasterQueryResponse> {
		const {query, reporter, bridge} = req;
		const {requestFlags} = query;

		if (requestFlags.benchmark && !origins.includes('benchmark')) {
			return this.dispatchBenchmarkRequest(req, bridgeEndPromise);
		}

		try {
			const defaultCommandFlags: Dict<unknown> = {};

			// A type-safe wrapper for retrieving command flags
			// TODO perhaps present this as JSON or something if this isn't a request from the CLI?
			const flagsConsumer = consume({
				filePath: createUnknownFilePath('argv'),
				parent: undefined,
				value: query.commandFlags,
				onDefinition(def) {
					// objectPath should only have a depth of 1
					defaultCommandFlags[def.objectPath[0]] = def.default;
				},
				objectPath: [],
				context: {
					category: 'flags/invalid',
					getOriginalValue: () => {
						return undefined;
					},
					normalizeKey: (key) => {
						return toKebabCase(key);
					},
					getDiagnosticPointer: (keys: ConsumePath) => {
						return req.getDiagnosticPointerFromFlags({
							type: 'flag',
							key: String(keys[0]),
							target: 'value',
						});
					},
				},
			});

			// An array of promises that we'll race, the only promise that will ever resolve will be the command one
			let promises: Array<Promise<unknown> | undefined> = [bridgeEndPromise];

			// Get command
			const masterCommand: undefined | MasterCommand<Dict<unknown>> = masterCommands.get(
				query.commandName,
			);
			if (masterCommand) {
				// Warn about disabled disk caching
				if (process.env.ROME_CACHE === '0' && !this.warnedCacheClients.has(bridge)) {
					reporter.warn(
						'Disk caching has been disabled due to the <emphasis>ROME_CACHE=0</emphasis> environment variable',
					);
					this.warnedCacheClients.add(bridge);
				}

				await validateRequestFlags(req, masterCommand);

				let commandFlags;
				if (masterCommand.defineFlags !== undefined) {
					commandFlags = masterCommand.defineFlags(flagsConsumer);
				}

				req.setNormalizedCommandFlags({
					flags: commandFlags,
					defaultFlags: defaultCommandFlags,
				});

				// @ts-ignore
				const commandPromise = masterCommand.callback(req, commandFlags);
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
			let diagnostics: undefined | Diagnostics = this.handleRequestError(req, err);

			if (diagnostics === undefined) {
				return {
					type: 'ERROR',
					fatal: false,
					handled: true,
					name: err.name,
					message: err.message,
					stack: err.stack,
				};
			} else if (err instanceof MasterRequestCancelled) {
				return {
					type: 'CANCELLED',
				};
			} else if (err instanceof MasterRequestInvalid) {
				return {
					type: 'INVALID_REQUEST',
					diagnostics,
					showHelp: err.showHelp,
				};
			} else {
				return {
					type: 'DIAGNOSTICS',
					diagnostics,
				};
			}
		}
	}

	handleRequestError(req: MasterRequest, rawErr: Error): undefined | Diagnostics {
		let err = rawErr;

		// If we can derive diagnostics from the error then create a diagnostics printer
		const diagnostics = getDiagnosticsFromError(err);
		if (diagnostics !== undefined) {
			const printer = req.createDiagnosticsPrinter(
				req.createDiagnosticsProcessor({
					origins: [
						{
							category: 'internal',
							message: 'Derived diagnostics from thrown error',
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
			if (rawErr instanceof MasterRequestInvalid) {
				shouldPrint = true;
			}
			if (!req.bridge.alive) {
				shouldPrint = false;
			}

			if (shouldPrint) {
				printer.print();

				// Don't output the footer if this is a notifier for an invalid request as it will be followed by a help screen
				if (!(rawErr instanceof MasterRequestInvalid)) {
					printer.footer();
				}
			}

			return printer.getDiagnostics();
		}

		if (!req.bridge.alive) {
			return undefined;
		}

		const printer = req.createDiagnosticsPrinter(
			req.createDiagnosticsProcessor({
				origins: [
					{
						category: 'internal',
						message: 'Error captured and converted into a diagnostic',
					},
				],
			}),
		);
		const errorDiag = deriveDiagnosticFromError(
			err,
			{
				description: {
					category: 'internalError/request',
				},
			},
		);
		printer.processor.addDiagnostic({
			...errorDiag,
			description: {
				...errorDiag.description,
				advice: [...errorDiag.description.advice, INTERNAL_ERROR_LOG_ADVICE],
			},
		});
		printer.print();

		// We could probably return printer.getDiagnostics() but we just want to print to the console

		// We will still want to send the `error` property
		return undefined;
	}
}
