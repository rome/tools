/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterQueryResponse} from '@romejs/core';
import {Event} from '@romejs/events';
import {ClientFlags, ClientFlagsJSON} from '../common/types/client';
import {ClientRequestType} from './ClientRequest';
import {Reporter} from '@romejs/cli-reporter';
import {DEFAULT_CLIENT_FLAGS} from '../common/types/client';
import ClientRequest from './ClientRequest';
import Master from '../master/Master';
import {MasterBridge, SOCKET_PATH, CLI_SOCKET_PATH} from '@romejs/core';
import fork from '../common/utils/fork';
import {createBridgeFromLocal, createBridgeFromSocket} from '@romejs/events';
import {ReporterDerivedStreams} from '@romejs/cli-reporter';
import prettyFormat from '@romejs/pretty-format';
import {VERSION} from '../common/constants';
import {TarWriter} from '@romejs/codec-tar';
import {Trace, Profiler, Profile, TraceEvent} from '@romejs/v8';
import {PartialMasterQueryRequest} from '../common/bridges/MasterBridge';
import {loadUserConfig, UserConfig} from '../common/userConfig';
import stream = require('stream');

import net = require('net');

import zlib = require('zlib');

import fs = require('fs');

import {unlink} from '@romejs/fs';
import {JSONValue} from '@romejs/codec-json';
import child = require('child_process');

export function getFilenameTimestamp(): string {
  return new Date().toISOString().replace(/[^0-9a-zA-Z]/g, '');
}

const NEW_SERVER_INIT_TIMEOUT = 10_000;

type ClientOptions = {
  globalErrorHandlers?: boolean;
  stdout?: stream.Writable;
  stderr?: stream.Writable;
  stdin?: NodeJS.ReadStream;
  flags: Partial<Omit<ClientFlags, 'clientName'>> & {clientName: string};
};

export type ClientProfileOptions = {
  samplingInterval: number;
  timeoutInterval: number;
  includeWorkers: boolean;
};

type BridgeStatus = {
  bridge: MasterBridge;
  dedicated: boolean;
};

type ClientRequestResponseResult = {
  request: PartialMasterQueryRequest;
  response: MasterQueryResponse;
};

export default class Client {
  constructor(opts: ClientOptions) {
    this.options = opts;
    this.userConfig = loadUserConfig();

    this.flags = {
      ...DEFAULT_CLIENT_FLAGS,
      ...opts.flags,
    };

    this.requestResponseEvent = new Event({
      name: 'Client.requestResponseEvent',
    });
    this.endEvent = new Event({name: 'Client.endEvent', serial: true});
    this.bridgeStatus = undefined;

    this.bridgeAttachedEvent = new Event({
      name: 'Client.bridgeAttached',
    });

    this.reporter = new Reporter({
      stdin: opts.stdin,
      silent: this.flags.silent === true || opts.stdout === undefined ||
      opts.stderr === undefined,
      verbose: this.flags.verbose === true,
      markupOptions: {
        cwd: this.flags.cwd,
      },
    });

    this.derivedReporterStreams = this.reporter.attachStdoutStreams(
      opts.stdout,
      opts.stderr,
    );

    this.endEvent.subscribe(() => {
      this.reporter.teardown();
    });
  }

  userConfig: UserConfig;
  options: ClientOptions;
  flags: ClientFlags;
  reporter: Reporter;
  derivedReporterStreams: ReporterDerivedStreams;
  bridgeStatus: undefined | BridgeStatus;
  bridgeAttachedEvent: Event<void, void>;

  requestResponseEvent: Event<ClientRequestResponseResult, void>;
  endEvent: Event<void, void>;

  getClientJSONFlags(): ClientFlagsJSON {
    return {
      ...this.flags,
      cwd: this.flags.cwd.join(),
    };
  }

  async profile(
    opts: ClientProfileOptions,
    callback: (profile: Array<TraceEvent>) => Promise<void>,
  ) {
    const {samplingInterval, timeoutInterval, includeWorkers} = opts;

    this.reporter.info('Starting CPU profile...');

    // Start server and start profiling
    const bridge = await this.findOrStartMaster();
    await bridge.profilingStart.call({
      samplingInterval,
    });

    // Start cli profiling
    let cliProfiler: undefined | Profiler;
    const bridgeStatus = this.getBridge();
    if (bridgeStatus === undefined || bridgeStatus.dedicated) {
      cliProfiler = new Profiler();
      await cliProfiler.startProfiling(samplingInterval);
    }

    // Start a profile timer if one was specified
    let hasProfiled: undefined | Promise<void>;
    let timeout: undefined | NodeJS.Timeout;
    if (timeoutInterval > 0) {
      timeout = setTimeout(() => {
        hasProfiled = stopProfile(true);
      }, timeoutInterval);
    }

    const stopProfile = async (isTimeout: boolean) => {
      // This is to prevent stopping the profile multiple times via the timeout and then at the end

      // It's a promise so that the final stopProfile call will block until the first has finished
      if (hasProfiled) {
        return hasProfiled;
      }

      // Stop the timeout if it hasn't been triggered
      if (timeout !== undefined) {
        clearTimeout(timeout);
      }

      //
      const trace = new Trace();
      const fetchers: Array<[string, () => Promise<Profile>]> = [];

      // CLI
      if (cliProfiler !== undefined) {
        const cliProfilerAssert = cliProfiler;
        fetchers.push([
          'CLI',
          async () => {
            return cliProfilerAssert.stopProfiling();
          },
        ]);
      }

      // Master
      fetchers.push([
        cliProfiler === undefined ? 'Master/CLI' : 'Master',
        async () => {
          return await bridge.profilingStop.call(undefined, {
            priority: true,
          });
        },
      ]);

      // Workers
      if (includeWorkers) {
        const workerIds = await bridge.profilingGetWorkers.call();
        for (const id of workerIds) {
          fetchers.push([
            `Worker ${id}`,
            async () => {
              return await bridge.profilingStopWorker.call(id, {
                priority: true,
              });
            },
          ]);
        }
      }

      // Fetch profiles
      const progress = this.reporter.progress();
      progress.setTotal(fetchers.length);
      progress.setTitle('Fetching profiles');
      for (const [text, callback] of fetchers) {
        progress.setText(text);
        const profile = await callback();
        trace.addProfile(text, profile);
        progress.tick();
      }
      progress.end();

      const events = trace.build();
      await callback(events);

      // If we're a timeout than separate these logs from the
      if (isTimeout) {
        this.reporter.hr();
      }
    };

    this.endEvent.subscribe(() => {
      return stopProfile(false);
    });
  }

  async subscribeLogs(
    includeWorkerLogs: boolean,
    callback: (chunk: string) => void,
  ): Promise<void> {
    const bridge = await this.findOrStartMaster();

    if (includeWorkerLogs) {
      await bridge.enableWorkerLogs.call();
    }

    bridge.log.subscribe(({origin, chunk}) => {
      if (origin === 'worker' && !includeWorkerLogs) {
        // We allow multiple calls to bridge.enableWorkerLogs

        // Filter the event if necessary if it wasn't requested by this log subscription
        return;
      }

      callback(chunk);
    });
  }

  async rage(ragePath: string, profileOpts: ClientProfileOptions) {
    if (this.bridgeStatus !== undefined) {
      throw new Error(
        'rage() can only be called before a query has been dispatched',
      );
    }

    let logs = '';
    await this.subscribeLogs(true, (chunk) => {
      logs += chunk;
    });

    // Collect CPU profile

    // Callback will be called later once it has been collected

    // Initial async work is just connecting to the processes and setting up handlers
    let profileEvents: Array<TraceEvent> = [];
    await this.profile(profileOpts, async (_profileEvents) => {
      profileEvents = _profileEvents;
    });

    // Collect all responses
    const responses: Array<ClientRequestResponseResult> = [];
    this.requestResponseEvent.subscribe((result) => {
      responses.push(result);
    });

    this.endEvent.subscribe(async () => {
      const stream = zlib.createGzip();
      stream.pipe(fs.createWriteStream(ragePath));

      const writer = new TarWriter(stream);

      writer.append({name: 'profile.json'}, stringify(profileEvents));
      writer.append({name: 'logs.txt'}, logs);

      // Add requests
      for (let i = 0; i < responses.length; i++) {
        const {request, response} = responses[i];
        const dirname = `requests/${i}-${request.commandName}`;
        writer.append({name: `${dirname}/request.json`}, stringify(request));
        writer.append({name: `${dirname}/response.json`}, stringify(response));
      }

      // Add client flags
      writer.append({name: 'clientFlags.json'}, stringify(
        this.getClientJSONFlags(),
      ));

      function stringify(val: JSONValue): string {
        return JSON.stringify(val, null, '  ');
      }

      function indent(val: unknown): string {
        const str = typeof val === 'string' ? val : prettyFormat(val, {
          compact: true,
        });
        const lines = str.trim().split('\n');
        const indented = lines.join('\n  ');
        return `\n  ${indented}`;
      }

      const env = [];
      env.push(`PATH: ${indent(process.env.PATH)}`);
      env.push(`Rome version: ${indent(VERSION)}`);
      env.push(`Node version: ${indent(process.versions.node)}`);
      env.push(`Platform: ${indent(`${process.platform} ${process.arch}`)}`);
      writer.append({name: 'environment.txt'}, `${env.join('\n\n')}\n`);

      // Don't do this if we never connected to the master
      const bridgeStatus = this.getBridge();
      if (bridgeStatus !== undefined) {
        const status = await this.query({
          silent: true,
          command: 'status',
        });
        if (status.type === 'SUCCESS') {
          writer.append({name: 'status.txt'}, `${prettyFormat(status.data, {
            compact: true,
          })}\n`);
        }
      }

      await writer.finalize();
      this.reporter.success('Rage archive written to', ragePath);
    });
  }

  async query(
    query: PartialMasterQueryRequest,
    type?: ClientRequestType,
  ): Promise<MasterQueryResponse> {
    const request = new ClientRequest(this, type, query);
    const res = await request.init();
    this.requestResponseEvent.send({request: query, response: res});
    return res;
  }

  getBridge(): undefined | BridgeStatus {
    return this.bridgeStatus;
  }

  async end() {
    await this.endEvent.callOptional();

    const status = this.bridgeStatus;
    if (status !== undefined) {
      status.bridge.end();
      this.bridgeStatus = undefined;
    }
  }

  async attachBridge(bridge: MasterBridge, dedicated: boolean) {
    const {stdout, stderr, columnsUpdated} = this.derivedReporterStreams;

    if (this.bridgeStatus !== undefined) {
      throw new Error('Already attached bridge to API');
    }

    this.bridgeStatus = {bridge, dedicated};

    bridge.stderr.subscribe((chunk) => {
      stderr.write(chunk);
    });

    bridge.stdout.subscribe((chunk) => {
      stdout.write(chunk);
    });

    bridge.reporterRemoteServerMessage.subscribe((msg) => {
      this.reporter.processRemoteClientMessage(msg);
    });

    this.reporter.sendRemoteServerMessage.subscribe((msg) => {
      bridge.reporterRemoteClientMessage.send(msg);
    });

    // Listen for resize column events if stdout is a TTY
    columnsUpdated.subscribe((columns: number) => {
      bridge.setColumns.call(columns);
    });

    await Promise.all([
      bridge.getClientInfo.wait({
        version: VERSION,
        format: stdout.format,
        hasClearScreen: this.reporter.hasClearScreen,
        columns: stdout.columns,
        useRemoteReporter: true,
        flags: this.getClientJSONFlags(),
      }),

      bridge.handshake(),
    ]);

    await this.bridgeAttachedEvent.call();
  }

  async findOrStartMaster(): Promise<MasterBridge> {
    // First check if we already have a bridge connection
    const connected = this.getBridge();
    if (connected !== undefined) {
      return connected.bridge;
    }

    // Then check if there's already a running daemon
    const runningDaemon = await this.tryConnectToExistingDaemon();
    if (runningDaemon) {
      return runningDaemon;
    }

    // Otherwise, start a master inside this process
    const master = new Master({
      dedicated: false,
      globalErrorHandlers: this.options.globalErrorHandlers === true,
    });
    await master.init();

    const bridge = createBridgeFromLocal(MasterBridge, {});
    await Promise.all([
      master.attachToBridge(bridge),
      this.attachBridge(bridge, false),
    ]);

    this.endEvent.subscribe(async () => {
      await master.end();
    });

    return bridge;
  }

  async forceStartDaemon(): Promise<MasterBridge> {
    const daemon = await this.startDaemon();
    if (daemon === undefined) {
      this.reporter.error('Failed to start daemon');
      throw new Error('Failed to start daemon');
    } else {
      return daemon;
    }
  }

  async startDaemon(): Promise<undefined | MasterBridge> {
    const {reporter} = this;

    if (this.bridgeStatus !== undefined) {
      throw new Error('Already started master');
    }

    reporter.info('No running daemon found. Starting one...');

    let exited = false;
    let proc: undefined | child.ChildProcess;

    const newDaemon: undefined | MasterBridge = await new Promise((resolve) => {
      const timeout = setTimeout(() => {
        reporter.error('Daemon connection timed out');
        cleanup();
        resolve();
      }, NEW_SERVER_INIT_TIMEOUT);

      const socketServer = net.createServer(() => {
        cleanup();
        resolve(this.tryConnectToNewDaemon());
      });

      function listen() {
        socketServer.listen(CLI_SOCKET_PATH.join());

        proc = fork('master', {
          detached: true,
        });
        proc.unref();

        proc.on('close', () => {
          exited = true;
          cleanup();
          resolve();
        });
      }

      unlink(CLI_SOCKET_PATH).finally(() => {
        listen();
      });

      function cleanup() {
        clearTimeout(timeout);
        socketServer.close();
      }
    });
    if (newDaemon) {
      return newDaemon;
    }

    // as a final precaution kill the server
    if (exited) {
      reporter.error('Daemon died while initialising.');
    } else {
      reporter.error('Failed to connect. Killing daemon.');
    }

    if (proc !== undefined) {
      proc.kill();
    }

    console.log('ughhh???');
    return undefined;
  }

  async tryConnectToNewDaemon(): Promise<undefined | MasterBridge> {
    const bridge = await this.tryConnectToExistingDaemon();
    if (bridge !== undefined) {
      this.reporter.success(`Started daemon!`);
      return bridge;
    }
  }

  async tryConnectToExistingDaemon(): Promise<undefined | MasterBridge> {
    const promise: Promise<undefined | net.Socket> = new Promise((
      resolve,
      reject,
    ) => {
      const socket = net.createConnection({
        path: SOCKET_PATH.join(),
      }, () => {
        resolve(socket);
      });

      socket.on('error', (err: NodeJS.ErrnoException) => {
        if (err.code === 'ENOENT' || err.code === 'ECONNREFUSED' || err.code ===
        'EADDRINUSE') {
          resolve();
        } else {
          reject(err);
        }
      });
    });

    const socket = await promise;
    if (socket === undefined) {
      return undefined;
    }

    const server = createBridgeFromSocket(MasterBridge, socket, {
      type: 'server',
    });
    await this.attachBridge(server, true);
    this.reporter.success('Connected to daemon');
    return server;
  }
}
