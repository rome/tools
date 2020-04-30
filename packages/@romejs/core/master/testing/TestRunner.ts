/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter} from '@romejs/cli-reporter';
import {
  DiagnosticOrigin,
  deriveDiagnosticFromError,
  descriptions,
} from '@romejs/diagnostics';
import {TestRef} from '../../common/bridges/TestWorkerBridge';
import {Master, MasterRequest, TestWorkerBridge} from '@romejs/core';
import {DiagnosticsPrinter} from '@romejs/cli-diagnostics';
import {createClient} from '@romejs/codec-websocket';
import {humanizeNumber} from '@romejs/string-utils';
import {
  Bridge,
  BridgeError,
  createBridgeFromChildProcess,
} from '@romejs/events';
import {
  CoverageCollector,
  ErrorFrame,
  InspectorClient,
  InspectorClientCloseError,
  NativeStructuredError,
  StructuredError,
  sourceMapManager,
  urlToFilename,
} from '@romejs/v8';
import fork from '../../common/utils/fork';
import {ManifestDefinition} from '@romejs/codec-js-manifest';
import {AbsoluteFilePath, createAbsoluteFilePath} from '@romejs/path';
import {ob1Coerce0To1} from '@romejs/ob1';
import {
  CoverageFolder,
  TestRunnerConstructorOptions,
  TestRunnerOptions,
  TestSource,
  TestSources,
  TestWorkerContainer,
  TestWorkerContainers,
} from './types';
import {formatPercent, percentInsideCoverageFolder, sortMapKeys} from './utils';
import {escapeMarkup, markup} from '@romejs/string-markup';
import {MAX_WORKER_COUNT} from '@romejs/core/common/constants';
import {TestWorkerFlags} from '@romejs/core/test-worker/TestWorker';
import net = require('net');

class BridgeStructuredError extends NativeStructuredError {
  constructor(struct: Partial<StructuredError>, bridge: Bridge) {
    super(struct);
    this.bridge = bridge;
  }

  bridge: Bridge;
}

function getProgressTestRefText(ref: TestRef) {
  return markup`<filelink target="${ref.filename}" />: ${escapeMarkup(
    ref.testName,
  )}`;
}

function findAvailablePort(): Promise<number> {
  return new Promise((resolve, reject) => {
    // When you create a server without specifying a port then the OS will choose a port number for you!
    const server = net.createServer();
    server.unref();
    server.on('error', reject);
    server.listen(
      undefined,
      () => {
        const address = server.address();
        if (address == null || typeof address === 'string') {
          throw new Error('Invalid address value');
        }

        server.close(() => {
          resolve(address.port);
        });
      },
    );
  });
}

export default class TestRunner {
  constructor(opts: TestRunnerConstructorOptions) {
    this.sources = opts.sources;
    this.reporter = opts.request.reporter;
    this.master = opts.request.master;
    this.cwd = opts.request.client.flags.cwd;
    this.request = opts.request;
    this.options = opts.options;

    this.ignoreBridgeEndError = new Set();

    this.sourcesQueue = Array.from(opts.sources.entries());

    this.coverageCollector = new CoverageCollector();

    this.progress = {
      totalTests: 0,
      startedTests: 0,
      finishedTests: 0,
      updatedSnapshots: 0,
      deletedSnapshots: 0,
      createdSnapshots: 0,
    };

    this.runningTests = new Map();
    this.testFileCounter = 0;

    this.printer = opts.request.createDiagnosticsPrinter(
      this.request.createDiagnosticsProcessor({
        origins: [
          {
            category: 'test',
            message: 'Run initiated',
          },
        ],
      }),
    );
    this.printer.processor.addDiagnostics(opts.addDiagnostics);

    // Add source maps
    for (const [filename, {code, sourceMap}] of opts.sources) {
      const consumer = sourceMap.toConsumer();
      this.coverageCollector.addSourceMap(filename, code, consumer);
      this.printer.processor.sourceMaps.add(filename, consumer);
    }
  }

  coverageCollector: CoverageCollector;
  options: TestRunnerOptions;
  request: MasterRequest;
  reporter: Reporter;
  sources: TestSources;
  workers: undefined | TestWorkerContainers;
  master: Master;
  cwd: AbsoluteFilePath;
  printer: DiagnosticsPrinter;
  sourcesQueue: Array<[string, TestSource]>;
  testFileCounter: number;
  ignoreBridgeEndError: Set<Bridge>;

  runningTests: Map<
    string,
    {
      ref: TestRef;
      timeout: undefined | NodeJS.Timeout;
    }
  >;

  progress: {
    totalTests: number;
    startedTests: number;
    finishedTests: number;
    updatedSnapshots: number;
    deletedSnapshots: number;
    createdSnapshots: number;
  };

  async runWorker({bridge, process, inspector}: TestWorkerContainer) {
    const {options: opts, sourcesQueue} = this;
    const req = this.request;
    const {flags} = req.client;

    if (inspector !== undefined && opts.coverage === true) {
      await inspector.call('Profiler.enable');
      await inspector.call(
        'Profiler.startPreciseCoverage',
        {
          // Turning this on disables V8 optimizations https://v8.dev/blog/javascript-code-coverage#precise-coverage-(function-granularity)
          callCount: false,
          // Otherwise coverage will only have function granularity
          detailed: true,
        },
      );
    }

    const nextTest = async () => {
      if (sourcesQueue.length === 0) {
        return;
      }

      const item = sourcesQueue.pop();
      if (item === undefined) {
        throw new Error('testQueue.length was validated above');
      }
      const [filename, {path, code, sourceMap}] = item;

      // Source map locations will always be resolved in the worker, but this is in case we need to resolve them in master in the case of an unresponsive worker
      // TODO remove this after test has ran
      const removeSourceMap = sourceMapManager.addSourceMap(
        filename,
        () => sourceMap.toConsumer(),
      );

      const id = this.testFileCounter;
      this.testFileCounter++;

      try {
        await bridge.prepareTest.call({
          id,
          options: opts,
          projectFolder: req.master.projectManager.assertProjectExisting(path).folder.join(),
          file: req.master.projectManager.getTransportFileReference(path),
          cwd: flags.cwd.join(),
          code,
        });

        await bridge.runTest.call(id);
      } finally {
        removeSourceMap();
      }

      await nextTest();
    };

    try {
      await nextTest();
    } catch (err) {
      if (err instanceof BridgeError || err instanceof BridgeStructuredError) {
        if (!this.ignoreBridgeEndError.has(err.bridge)) {
          this.printer.processor.addDiagnostic(
            deriveDiagnosticFromError({
              category: 'tests/timeout',
              error: err,
            }),
          );
        }
      } else {
        throw err;
      }
    } finally {
      if (inspector !== undefined) {
        if (opts.coverage) {
          if (inspector.alive) {
            const profile = await inspector.call('Profiler.takePreciseCoverage');
            this.coverageCollector.addCoverage(profile.get('result').asAny());

            // Not really necessary but let's clean up anyway for completeness
            await inspector.call('Profiler.stopPreciseCoverage');
            await inspector.call('Profiler.disable');
          } else {
            // TODO log that we failed to fetch some coverage
          }
        }

        inspector.end();
      }

      process.kill();
    }
  }

  async spawnWorker(flags: TestWorkerFlags): Promise<TestWorkerContainer> {
    const proc = fork(
      'test-worker',
      {
        stdio: 'pipe',
      },
      ['--inspector-port', String(flags.inspectorPort)],
    );

    const {stdout, stderr} = proc;
    if (stdout == null || stderr == null) {
      throw new Error('stdout or stderr was undefined for a spawned Worker');
    }

    stdout.on(
      'data',
      (chunk) => {
        process.stdout.write(chunk);
      },
    );

    // Suppress any debugger logs
    stderr.on(
      'data',
      (chunk) => {
        const str = chunk.toString();

        if (str.startsWith('Debugger listening on ws://')) {
          return;
        }

        if (
          str.startsWith('For help, see: https://nodejs.org/en/docs/inspector')
        ) {
          return;
        }

        if (str.startsWith('Debugger attached')) {
          return;
        }

        process.stderr.write(chunk);
      },
    );

    const bridge = createBridgeFromChildProcess(
      TestWorkerBridge,
      proc,
      {
        type: 'client',
      },
    );
    await bridge.handshake();

    const {inspectorUrl} = await bridge.inspectorDetails.call();

    let inspector;
    if (inspectorUrl !== undefined) {
      const locInspector = new InspectorClient(await createClient(inspectorUrl));
      inspector = locInspector;
      await locInspector.call('Debugger.enable');

      bridge.endEvent.subscribe(() => {
        locInspector.end();
      });
    }

    return {
      bridge,
      process: proc,
      inspector,
    };
  }

  async setupWorkers(): Promise<TestWorkerContainers> {
    // TODO some smarter logic. we may not need all these workers
    const containerPromises: Array<Promise<TestWorkerContainer>> = [];
    for (let i = 0; i < MAX_WORKER_COUNT; i++) {
      const inspectorPort = await findAvailablePort();
      containerPromises.push(this.spawnWorker({inspectorPort}));
    }

    const containers: TestWorkerContainers = await Promise.all(
      containerPromises,
    );

    // Every 5 seconds, ping the worker and wait a max of 5 seconds, if we receive no response then consider the worker dead
    for (const container of containers) {
      container.bridge.monitorHeartbeat(
        5_000,
        async () => {
          this.handleWorkerTimeout('10 seconds', container);
        },
      );
    }

    return containers;
  }

  async init() {
    this.workers = await this.setupWorkers();
    const teardown = this.setupProgress();

    const workerContainers: TestWorkerContainers = this.getWorkers();

    await Promise.all(
      workerContainers.map((container) => this.runWorker(container)),
    );

    teardown();

    this.throwPrinter();
  }

  async handleWorkerTimeout(
    duration: string,
    container: TestWorkerContainer,
  ): Promise<void> {
    return new Promise((resolve, reject) => {
      const timeout = setTimeout(
        () => {
          resolve(
            container.bridge.end(
              `Test worker was unresponsive for ${duration}. We tried to collect some additional metadata but we timed out again trying to fetch it...`,
            ),
          );
        },
        3_000,
      );

      this._handleWorkerTimeout(duration, container).then(() => {
        clearTimeout(timeout);
        resolve();
      }).catch((err) => {
        clearTimeout(timeout);
        if (err instanceof InspectorClientCloseError) {
          return container.bridge.end(
            `Test worker was unresponsive for ${duration}. We tried to collect some additional metadata but the inspector connection closed abruptly`,
          );
        } else {
          reject(err);
        }
      });
    });
  }

  async _handleWorkerTimeout(
    duration: string,
    {bridge, inspector}: TestWorkerContainer,
  ): Promise<void> {
    if (inspector === undefined) {
      bridge.end(
        `Test worker was unresponsive for ${duration}. There was no inspector connected so we were unable to capture stack frames before it was terminated.`,
      );
      return undefined;
    }

    inspector.call('Debugger.pause');

    const params = await inspector.wait('Debugger.paused');

    const frames: Array<ErrorFrame> = [];

    const callFrames = params.get('callFrames').asArray().slice(0, 20);
    for (const callFrame of callFrames) {
      const loc = callFrame.get('location');

      const resolved = sourceMapManager.resolveLocation(
        urlToFilename(callFrame.get('url').asString()),
        ob1Coerce0To1(loc.get('lineNumber').asZeroIndexedNumber()),
        loc.get('columnNumber').asZeroIndexedNumber(),
      );

      const name = callFrame.get('scopeChain').asArray()[0].get('name').asString(
        '',
      ).split('$').pop();

      frames.push({
        resolvedLocation: resolved.found,
        typeName: undefined,
        functionName: name,
        methodName: undefined,
        filename: resolved.filename,
        lineNumber: resolved.line,
        columnNumber: resolved.column,
        isTopLevel: false,
        isEval: false,
        isNative: false,
        isConstructor: false,
        isAsync: false,
      });
    }

    bridge.endWithError(
      new BridgeStructuredError(
        {
          message: `Test worker was unresponsive for <emphasis>${duration}</emphasis>. Possible infinite loop. Below is a stack trace before the test was terminated.`,
          frames,
          advice: [
            {
              type: 'log',
              category: 'info',
              text: `You can find the specific test that caused this by running <command>rome test --sync-tests</command>`,
            },
          ],
        },
        bridge,
      ),
    );
  }

  getWorkers(): TestWorkerContainers {
    if (this.workers === undefined) {
      throw new Error('TestRunner.init has not been called yet');
    } else {
      return this.workers;
    }
  }

  refToKey(ref: TestRef): string {
    return `${ref.filename}: ${ref.testName}`;
  }

  onTestStart(
    container: TestWorkerContainer,
    ref: TestRef,
    timeoutMs: undefined | number,
  ) {
    this.progress.startedTests++;

    let timeout = undefined;
    if (timeoutMs !== undefined) {
      timeout = setTimeout(
        () => {
          // TODO This will kill the whole worker, maybe it's possible to just terminate the current test? Throw an error, see if the next test was ran, or else terminate completely
          this.handleWorkerTimeout(`${String(timeoutMs)}ms`, container);
        },
        timeoutMs,
      );
    }

    this.runningTests.set(
      this.refToKey(ref),
      {
        ref,
        timeout,
      },
    );
  }

  onTestFound(data: TestRef, isSkipped: boolean) {
    if (isSkipped) {
      return;
    }

    data;
    this.progress.totalTests++;
  }

  onTestFinished(ref: TestRef) {
    const key = this.refToKey(ref);
    const running = this.runningTests.get(key);
    if (running === undefined) {
      throw new Error('Expected there to be a running test');
    }

    if (running.timeout !== undefined) {
      clearTimeout(running.timeout);
    }
    this.runningTests.delete(key);

    this.progress.finishedTests++;
  }

  setupProgress(): () => void {
    const workers = this.getWorkers();

    const progress = this.request.reporter.progress({
      persistent: true,
      title: 'Running',
    });

    for (let i = 0; i < workers.length; i++) {
      const container = workers[i];
      const {bridge} = container;

      const ourRunningTests: Set<string> = new Set();

      bridge.endEvent.subscribe((error) => {
        // Cancel all currently running tests
        const cancelTests: Array<TestRef> = [];

        for (const key of ourRunningTests) {
          const test = this.runningTests.get(key);
          if (test !== undefined) {
            cancelTests.push(test.ref);
          }
        }

        for (const ref of cancelTests) {
          this.onTestFinished(ref);

          if (cancelTests.length === 1) {
            // If we only have one test to cancel then let's only point the bridge error to this test
            this.ignoreBridgeEndError.add(bridge);

            const errDiag = deriveDiagnosticFromError({
              label: ref.testName,
              category: 'tests/failure',
              filename: ref.filename,
              error,
            });

            this.printer.processor.addDiagnostic({
              ...errDiag,
              description: {
                ...errDiag.description,
                // We don't care about the advice
                advice: [],
              },
            });
          } else {
            this.printer.processor.addDiagnostic({
              label: ref.testName,
              description: descriptions.TESTS.CANCELLED,
              location: {
                filename: ref.filename,
              },
            });
          }
        }
      });

      bridge.snapshotUpdated.subscribe(({event}) => {
        switch (event) {
          case 'create': {
            this.progress.createdSnapshots++;
            break;
          }

          case 'update': {
            this.progress.updatedSnapshots++;
            break;
          }

          case 'delete': {
            this.progress.deletedSnapshots++;
            break;
          }
        }
      });

      bridge.testsFound.subscribe((tests) => {
        for (const {ref, isSkipped} of tests) {
          this.onTestFound(ref, isSkipped);
        }
        progress.setTotal(this.progress.totalTests);
      });

      bridge.testStart.subscribe((data) => {
        ourRunningTests.add(this.refToKey(data.ref));
        this.onTestStart(container, data.ref, data.timeout);
        progress.pushText(getProgressTestRefText(data.ref));
      });

      bridge.testError.subscribe((data) => {
        let origin: DiagnosticOrigin = {
          category: 'test/error',
          message: 'Generated from a test worker without being attached to a test',
        };

        const {ref} = data;
        if (ref !== undefined) {
          const uid = this.master.projectManager.getUid(
            createAbsoluteFilePath(ref.filename),
          );
          origin.message = markup`Generated from the file <filelink target="${uid}" /> and test name "${ref.testName}"`;
          this.onTestFinished(ref);
          progress.popText(getProgressTestRefText(ref));
          progress.tick();
        }

        this.printer.processor.addDiagnostic(data.diagnostic, origin);
      });

      bridge.testSuccess.subscribe((data) => {
        this.onTestFinished(data.ref);
        progress.popText(getProgressTestRefText(data.ref));
        progress.tick();
      });
    }

    return function() {
      progress.end();
    };
  }

  printCoverageReport(isError: boolean) {
    const {reporter, master, coverageCollector} = this;

    if (isError && this.options.showAllCoverage) {
      // Only show coverage for errors when --show-all-coverage has been specified
      return;
    }

    if (!this.options.coverage) {
      return;
    }

    reporter.info('Generating coverage');

    // Fetch coverage entries
    const files = coverageCollector.generate();
    if (files.length === 0) {
      return;
    }

    reporter.heading('Code coverage');

    // Get the packages associated with all the ran tests, we will filter code coverage to those packages only
    const testedPackages: Set<undefined | ManifestDefinition> = new Set();
    for (const {path} of this.sources.values()) {
      testedPackages.add(master.memoryFs.getOwnedManifest(path));
    }

    let root: CoverageFolder = {
      name: undefined,
      folders: new Map(),
      files: new Map(),
    };

    let totalFiles = 0;

    // Turn the flat list of filenames into a directory tree
    for (const file of files) {
      const {filename} = file;

      // Get the absolute filename
      const absolute = master.projectManager.getFilePathFromUid(filename);
      if (absolute === undefined) {
        continue;
      }

      // Filter out untested packages
      const pkg = master.memoryFs.getOwnedManifest(absolute);
      if (testedPackages.has(pkg) === false) {
        continue;
      }

      // TODO maybe filter out test files too?

      // Track unfiltered files
      totalFiles++;

      const filenameParts = filename.split('/');
      const basename = filenameParts.pop();
      if (basename === undefined) {
        throw new Error('Should always be at least one element from a split()');
      }

      let target: CoverageFolder = root;

      for (const part of filenameParts) {
        const existingFolder = target.folders.get(part);
        if (existingFolder === undefined) {
          const newFolder = {
            name: part,
            folders: new Map(),
            files: new Map(),
          };
          target.folders.set(part, newFolder);
          target = newFolder;
        } else {
          target = existingFolder;
        }
      }

      target.files.set(basename, file);
    }

    // Continuously merge all entries with only a single folder from the root
    while (root.folders.size === 1 && root.files.size === 0) {
      // Awkward way to get the first value out of the folders map...
      const newRoot = root.folders.values().next().value;
      root = {
        ...newRoot,
        name: root.name !== undefined && newRoot.name !== undefined
          ? `${root.name}/${newRoot.name}`
          : newRoot.name,
      };
    }

    const rows: Array<Array<string>> = [];

    // If there's more than 15 files to show, and we don't have the explicit showAllCoverage flag

    // then truncate the output
    const showAllCoverage = this.options.showAllCoverage || totalFiles < 15;

    function buildRows(folder: CoverageFolder, depth: number) {
      const name = folder.name === undefined ? 'All files' : `${folder.name}/`;
      const folderPercent = percentInsideCoverageFolder(folder);

      rows.push([
        ' '.repeat(depth) + `<emphasis>${name}</emphasis>`,
        formatPercent(folderPercent.functions),
        formatPercent(folderPercent.branches),
        formatPercent(folderPercent.lines),
      ]);

      // Don't ever show anything deeper than a single level when showAllCoverage is off
      if (!showAllCoverage && depth > 0) {
        return;
      }

      const fileIndent = ' '.repeat(depth + 1);
      for (const [name, file] of sortMapKeys(folder.files)) {
        let absolute = file.filename;

        // Exchange any UIDs
        const absolutePath = master.projectManager.getFilePathFromUid(
          file.filename,
        );
        if (absolutePath !== undefined) {
          absolute = absolutePath.join();
        }

        rows.push([
          fileIndent + markup`<filelink target="${absolute}">${name}</filelink>`,
          formatPercent(file.functions.percent),
          formatPercent(file.branches.percent),
          formatPercent(file.lines.percent),
        ]);
      }

      for (const subFolder of sortMapKeys(folder.folders).values()) {
        buildRows(subFolder, depth + 1);
      }
    }

    buildRows(root, 0);

    reporter.table(['File', '% Functions', '% Branches', '% Lines'], rows);

    if (!showAllCoverage) {
      reporter.br();
      reporter.info(
        'Additional coverage information available. Refine the executed tests or add the <emphasis>--show-all-coverage</emphasis> flag',
      );
    }

    reporter.hr();
  }

  getSourceCode(filename: string): undefined | string {
    const testSource = this.sources.get(filename);
    if (testSource === undefined) {
      return undefined;
    } else {
      return testSource.code;
    }
  }

  printSnapshotCounts(reporter: Reporter) {
    const {
      createdSnapshots,
      deletedSnapshots,
      updatedSnapshots,
    } = this.progress;

    const snapshotCounts: Array<{
      count: number;
      noun: string;
    }> = [
      {count: createdSnapshots, noun: 'created'},
      {count: updatedSnapshots, noun: 'updated'},
      {count: deletedSnapshots, noun: 'deleted'},
    ].filter(({count}) => count > 0);

    if (snapshotCounts.length === 0) {
      return;
    }

    const first = snapshotCounts.shift()!;
    const parts = [
      `<number emphasis>${first.count}</number> snapshots ${first.noun}`,
    ];

    for (const {count, noun} of snapshotCounts) {
      parts.push(`<number emphasis>${count}</emphasis> ${noun}`);
    }

    reporter.success(parts.join(', '));
  }

  throwPrinter() {
    const {printer} = this;

    printer.onFooterPrint((reporter, isError) => {
      this.printCoverageReport(isError);
      this.printSnapshotCounts(reporter);

      if (!isError) {
        reporter.success(
          `All <emphasis>${humanizeNumber(this.progress.totalTests)}</emphasis> tests passed!`,
        );
        return true;
      }

      // Show default footer
      return false;
    });

    throw printer;
  }
}
