/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  MasterBridge,
  MasterQueryResponse,
  MasterQueryRequest,
} from '@romejs/core';
import {
  Diagnostics,
  PartialDiagnostics,
  INTERNAL_ERROR_LOG_ADVICE,
  DiagnosticOrigin,
} from '@romejs/diagnostics';
import {MasterCommand} from '../commands';
import {
  DiagnosticsPrinter,
  DiagnosticsFileReader,
  readDiagnosticsFileLocal,
} from '@romejs/cli-diagnostics';
import {consume, ConsumePath} from '@romejs/consume';
import {Event, EventSubscription} from '@romejs/events';
import MasterRequest, {MasterRequestInvalid} from './MasterRequest';
import {
  getDiagnosticsFromError,
  deriveDiagnosticFromError,
} from '@romejs/diagnostics';
import {DiagnosticsProcessor} from '@romejs/diagnostics';
import ProjectManager from './project/ProjectManager';
import WorkerManager from './WorkerManager';
import Resolver from './fs/Resolver';
import FileAllocator from './fs/FileAllocator';
import Logger from '../common/utils/Logger';
import MemoryFileSystem from './fs/MemoryFileSystem';
import Cache from './Cache';
import {masterCommands} from './commands/index';
import {Reporter} from '@romejs/cli-reporter';
import {Profiler} from '@romejs/v8';
import {
  ProfilingStartData,
  PartialMasterQueryRequest,
} from '../common/bridges/MasterBridge';
import {ReporterStream} from '@romejs/cli-reporter';
import {
  ClientFlags,
  DEFAULT_CLIENT_REQUEST_FLAGS,
  ClientRequestFlags,
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

const STDOUT_MAX_CHUNK_LENGTH = 100_000;

export type MasterClient = {
  id: number;
  reporter: Reporter;
  bridge: MasterBridge;
  flags: ClientFlags;
  version: string;
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

export type MasterMarker =
  & MasterUnfinishedMarker
  & {// End time in milliseconds
    end: number};

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

    this.logger = new Logger('master', () => {
      return this.logEvent.hasSubscribers() ||
      this.connectedClientsListeningForLogs.size > 0;
    }, {
      streams: [
        {
          type: 'all',
          format: 'none',
          columns: 0,
          write: (chunk) => {
            this.emitMasterLog(chunk);
          },
        },
      ],
    });

    this.connectedReporters = new Reporter({
      wrapperFactory: this.wrapFatal.bind(this),
    });

    this.connectedClientsListeningForLogs = new Set();
    this.connectedClients = new Set();

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
  resolver: Resolver;
  projectManager: ProjectManager;
  workerManager: WorkerManager;
  fileAllocator: FileAllocator;
  cache: Cache;
  connectedReporters: Reporter;
  logger: Logger;

  connectedClients: Set<MasterClient>;
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

  // rome-suppress lint/noExplicitAny
  wrapFatal<T extends (...args: Array<any>) => any>(callback: T): T {
    // rome-suppress lint/noExplicitAny
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

  async handleDisconnectedDiagnostics(diagnostics: PartialDiagnostics) {
    this.connectedReporters.error(
      'Generated diagnostics without a current request',
    );
    const printer = new DiagnosticsPrinter({
      origins: [],
      reporter: this.connectedReporters,
      readFile: this.readDiagnosticsPrinterFile.bind(this),
    });
    printer.addDiagnostics(diagnostics);
    await printer.print();
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

  createDisconnectedDiagnosticsProcessor(
    origins: Array<DiagnosticOrigin>,
  ): DiagnosticsProcessor {
    return new DiagnosticsProcessor({
      onDiagnostics: (diagnostics: PartialDiagnostics) => {
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
    await this.workerManager.init();
  }

  async end() {
    // We should remove everything that has an external dependency like a socket or process

    // TODO terminate all queries in flight

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

    await this.clientStartEvent.callOptional(client);

    bridge.query.subscribe(async (request) => {
      return await this.handleRequest(client, request);
    });
  }

  async createClient(bridge: MasterBridge): Promise<MasterClient> {
    const {
      flags: rawFlags,
      useRemoteReporter,
      hasClearScreen,
      columns,
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
      silent: flags.silent,
      markupOptions: {
        cwd: flags.cwd,
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
        },
        normalizeFilename: (filename) => {
          const path = this.projectManager.getFilePathFromUid(filename);
          if (path === undefined) {
            return filename;
          } else {
            return path.join();
          }
        },
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
    this.connectedReporters.addStream(errStream);
    this.connectedReporters.addStream(outStream);

    const client: MasterClient = {
      id: this.clientIdCounter++,
      bridge,
      reporter,
      flags,
      version,
    };

    this.connectedClients.add(client);

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

    bridge.updatedListenersEvent.subscribe((listeners) => {
      if (listeners.has('log')) {
        this.connectedClientsListeningForLogs.add(client);
      } else {
        this.connectedClientsListeningForLogs.delete(client);
      }
    });

    bridge.endEvent.subscribe(() => {
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

  async handleRequest(
    client: MasterClient,
    partialQuery: PartialMasterQueryRequest,
  ): Promise<MasterQueryResponse> {
    const requestFlags: ClientRequestFlags = {
      ...DEFAULT_CLIENT_REQUEST_FLAGS,
      ...partialQuery.requestFlags,
    };

    const query: MasterQueryRequest = {
      commandName: partialQuery.command,
      args: partialQuery.args === undefined ? [] : partialQuery.args,
      noData: partialQuery.noData === true,
      requestFlags,
      silent: partialQuery.silent === true || requestFlags.benchmark,
      terminateWhenIdle: partialQuery.terminateWhenIdle === true,
      commandFlags: partialQuery.commandFlags === undefined
        ? {} : partialQuery.commandFlags,
    };

    const {bridge} = client;
    this.logger.info(`[Master] Handling CLI request:`, query);

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

    // Create master request wrapper which is just a bucket of objects
    const req = new MasterRequest({
      client,
      query,
      master: this,
      reporter,
      bridge,
    });

    // Hook used by the web server to track and collect master requests
    await this.requestStartEvent.callOptional(req);

    // Track the amount of running queries for terminateWhenIdle
    this.requestRunningCounter++;

    // Sometimes we'll want to terminate the process once all queries have finished
    if (query.terminateWhenIdle) {
      this.terminateWhenIdle = true;
    }

    try {
      const res: MasterQueryResponse = await this.dispatchRequest(
        req,
        bridgeEndPromise,
        [],
      );

      req.teardown(res);

      // If the query asked for no data then strip all diagnostics and data values
      if (query.noData) {
        if (res.type === 'SUCCESS') {
          return {
            type: 'SUCCESS',
            hasData: res.data !== undefined,
            data: undefined,
            markers: res.markers,
          };
        } else if (res.type === 'DIAGNOSTICS') {
          return {
            type: 'DIAGNOSTICS',
            diagnostics: [],
          };
        } else if (res.type === 'INVALID_REQUEST') {
          return {
            type: 'INVALID_REQUEST',
            diagnostics: [],
          };
        }
      }

      return res;
    } finally {
      this.requestRunningCounter--;

      this.logger.info(`[Master] Replying to CLI request:`, query);

      // We no longer care if the client dies
      bridgeEndEvent.unsubscribe();

      // If we're waiting to terminate ourselves when idle, then do so when there's no running requests
      if (this.terminateWhenIdle && this.requestRunningCounter === 0) {
        this.end();
      }
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
    const result = await this.dispatchRequest(req, bridgeEndPromise, [
      'benchmark',
    ]);
    const warmupTook = Date.now() - warmupStart;

    // Benchmark
    const progress = client.reporter.progress();
    progress.setTitle('Running benchmark');
    progress.setTotal(benchmarkIterations);
    const benchmarkStart = Date.now();
    for (let i = 0; i < benchmarkIterations; i++) {
      await this.dispatchRequest(req, bridgeEndPromise, ['benchmark']);
      progress.tick();
    }
    progress.end();
    const benchmarkTook = Date.now() - benchmarkStart;

    reporter.section('Benchmark results', () => {
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
    });

    return result;
  }

  async dispatchRequest(
    req: MasterRequest,
    bridgeEndPromise: Promise<void>,
    origins: Array<string>,
  ): Promise<MasterQueryResponse> {
    const {query, reporter, bridge} = req;

    if (query.requestFlags.benchmark && !origins.includes('benchmark')) {
      return this.dispatchBenchmarkRequest(req, bridgeEndPromise);
    }

    const markers: Array<MasterMarker> = [];

    if (query.requestFlags.collectMarkers) {
      req.markerEvent.subscribe((marker) => {
        markers.push(marker);
      });
    }

    try {
      const defaultCommandFlags: Dict<unknown> = {};

      // A type-safe wrapper for retrieving command flags

      // TODO perhaps present this as JSON or something if this isn't a request from the CLI?
      const commandFlagsConsumer = consume({
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
      const commandOpts: undefined | MasterCommand<Dict<unknown>> =
        masterCommands.get(query.commandName);
      if (commandOpts) {
        // Warn about disabled disk caching
        if (process.env.ROME_CACHE === '0' && !this.warnedCacheClients.has(
          bridge,
        )) {
          reporter.warn(
            'Disk caching has been disabled due to the <emphasis>ROME_CACHE=0</emphasis> environment variable',
          );
          this.warnedCacheClients.add(bridge);
        }

        let commandFlags;
        if (commandOpts.defineFlags !== undefined) {
          commandFlags = commandOpts.defineFlags(commandFlagsConsumer);
        }

        req.setNormalizedCommandFlags({
          flags: commandFlags,
          defaultFlags: defaultCommandFlags,
        });

        // @ts-ignore
        const commandPromise = commandOpts.default(req, commandFlags);
        promises.push(commandPromise);

        await Promise.race(promises);

        // Only the command promise should have won the race with a resolve
        const data = await commandPromise;
        return {
          type: 'SUCCESS',
          hasData: data !== undefined,
          data,
          markers,
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
          type: 'ERROR',
          fatal: false,
          handled: true,
          name: err.name,
          message: err.message,
          stack: err.stack,
        };
      } else if (diagnostics.length === 0) {
        // Maybe DIAGNOSTICS and an empty array still makes sense instead of SUCCESS?
        return {
          type: 'SUCCESS',
          hasData: false,
          data: undefined,
          markers,
        };
      } else {
        if (err instanceof MasterRequestInvalid) {
          return {
            type: 'INVALID_REQUEST',
            diagnostics,
          };
        } else {
          return {
            type: 'DIAGNOSTICS',
            diagnostics,
          };
        }
      }
    }
  }

  async handleRequestError(
    req: MasterRequest,
    rawErr: Error,
  ): Promise<undefined | Diagnostics> {
    let err = rawErr;

    // If we can derive diagnostics from the error then create a diagnostics printer
    const diagnostics = getDiagnosticsFromError(err);
    if (diagnostics !== undefined) {
      const printer = req.createDiagnosticsPrinter({
        category: 'internal',
        message: 'Derived diagnostics from thrown error',
      });
      printer.addDiagnostics(diagnostics);
      err = printer;
    }

    // Print it!
    if (err instanceof DiagnosticsPrinter) {
      const printer = err;
      if (req.bridge.alive) {
        await printer.print();

        // Don't output the footer if this is a notifier for an invalid request as it will be followed by a help screen
        if (!(rawErr instanceof MasterRequestInvalid)) {
          printer.footer();
        }
      }
      return printer.getDiagnostics();
    }

    if (!req.bridge.alive) {
      return;
    }

    const printer = req.createDiagnosticsPrinter({
      category: 'internal',
      message: 'Error captured and converted into a diagnostic',
    });
    const errorDiag = deriveDiagnosticFromError({
      category: 'internalError/request',
      error: err,
    });
    printer.addDiagnostic({
      ...errorDiag,
      advice: [...(errorDiag.advice || []), INTERNAL_ERROR_LOG_ADVICE],
    });
    await printer.print();

    // We could probably return printer.getDiagnostics() but we just want to print to the console

    // We will still want to send the `error` property
    return;
  }
}
