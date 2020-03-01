/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter} from '@romejs/cli-reporter';
import {DiagnosticOrigin} from '@romejs/diagnostics';
import {TestRef} from '../../common/bridges/TestWorkerBridge';
import {Master, MasterRequest} from '@romejs/core';
import {DiagnosticsPrinter} from '@romejs/cli-diagnostics';
import {createClient} from '@romejs/codec-websocket';
import {humanizeNumber} from '@romejs/string-utils';
import {createBridgeFromChildProcess} from '@romejs/events';
import {
  InspectorClientCloseError,
  InspectorClient,
  CoverageCollector,
  sourceMapManager,
} from '@romejs/v8';
import {deriveDiagnosticFromError} from '@romejs/diagnostics';
import {TestWorkerBridge} from '@romejs/core';
import fork from '../../common/utils/fork';
import {ManifestDefinition} from '@romejs/codec-js-manifest';
import {createAbsoluteFilePath, AbsoluteFilePath} from '@romejs/path';
import {urlToFilename} from '@romejs/v8';
import {coerce0to1} from '@romejs/ob1';
import {createErrorFromStructure, ErrorFrame} from '@romejs/v8';
import {BridgeError} from '@romejs/events';
import {
  TestRunnerConstructorOptions,
  TestRunnerOptions,
  TestSources,
  TestWorkerContainers,
  TestSource,
  TestWorkerContainer,
  CoverageFolder,
} from './types';
import {percentInsideCoverageFolder, formatPercent, sortMapKeys} from './utils';

export default class TestRunner {
  constructor(opts: TestRunnerConstructorOptions) {
    this.sources = opts.sources;
    this.reporter = opts.request.reporter;
    this.master = opts.request.master;
    this.cwd = opts.request.client.flags.cwd;
    this.request = opts.request;
    this.options = opts.options;

    this.sourcesQueue = Array.from(opts.sources.entries());

    this.coverageCollector = new CoverageCollector();

    this.progress = {
      total: 0,
      started: 0,
      finished: 0,
    };

    this.runningTests = new Map();

    this.printer = opts.request.createDiagnosticsPrinter({
      category: 'test',
      message: 'Run initiated',
    });
    this.printer.addDiagnostics(opts.addDiagnostics);
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

  runningTests: Map<
    string,
    {
      ref: TestRef;
      timeout: undefined | NodeJS.Timeout;
    }
  >;

  progress: {
    total: number;
    started: number;
    finished: number;
  };

  async runWorker({bridge, process, inspector}: TestWorkerContainer) {
    const {options: opts, sourcesQueue} = this;
    const req = this.request;
    const {flags} = req.client;

    if (inspector !== undefined && opts.coverage === true) {
      await inspector.call('Profiler.enable');
      await inspector.call('Profiler.startPreciseCoverage', {
        // Turning this on disables V8 optimizations https://v8.dev/blog/javascript-code-coverage#precise-coverage-(function-granularity)
        callCount: false,
        // Otherwise coverage will only have function granularity
        detailed: true,
      });
    }

    const nextTest = async () => {
      if (sourcesQueue.length === 0) {
        return undefined;
      }

      const item = sourcesQueue.pop();
      if (item === undefined) {
        throw new Error('testQueue.length was validated above');
      }
      const [filename, {path, code, sourceMap}] = item;

      this.coverageCollector.addSourceMap(filename, code, sourceMap);

      // Source map locations will always be resolved in the worker, but this is in case we need to resolve them in master in the case of an unresponsive worker
      // TODO remove this after test has ran
      const removeSourceMap = sourceMapManager.addSourceMap(
        filename,
        sourceMap,
      );

      try {
        await bridge.runTest.call({
          options: opts,
          file: req.master.projectManager.getTransportFileReference(path),
          cwd: flags.cwd.join(),
          code,
          sourceMap,
        });
      } finally {
        removeSourceMap();
      }

      await nextTest();
    };

    try {
      await nextTest();
    } catch (err) {
      if (err instanceof BridgeError) {
        // When a worker dies, we automatically mark all it's currently running tests as failed
        // However, it will cause all the pending messages to throw an error which will bubble up here
        // We can safely ignore these since we've already handled it
      } else {
        throw err;
      }
    } finally {
      if (inspector !== undefined) {
        if (opts.coverage) {
          if (inspector.alive) {
            const profile = await inspector.call(
              'Profiler.takePreciseCoverage',
            );
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

  async spawnWorker(): Promise<TestWorkerContainer> {
    const proc = fork('test-worker', {
      stdio: 'pipe',
    });

    const {stdout, stderr} = proc;
    if (stdout == null || stderr == null) {
      throw new Error('stdout or stderr was undefined for a spawned Worker');
    }

    stdout.on('data', chunk => {
      process.stdout.write(chunk);
    });

    // Suppress any debugger logs
    stderr.on('data', chunk => {
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
    });

    const bridge = createBridgeFromChildProcess(TestWorkerBridge, proc, {
      type: 'client',
    });
    await bridge.handshake();

    const {inspectorUrl} = await bridge.inspectorDetails.call();

    let inspector;
    if (inspectorUrl !== undefined) {
      const locInspector = new InspectorClient(
        await createClient(inspectorUrl),
      );
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
    const containers: TestWorkerContainers = [await this.spawnWorker()];

    // Every 5 seconds, ping the worker and wait a max of 5 seconds, if we receive no response then consider the worker dead
    for (const container of containers) {
      container.bridge.monitorHeartbeat(5000, async () => {
        this.handleWorkerTimeout('10 seconds', container);
      });
    }

    return containers;
  }

  async init() {
    this.workers = await this.setupWorkers();
    this.setupProgress();

    const workerContainers: TestWorkerContainers = this.getWorkers();

    await Promise.all(
      workerContainers.map(container => this.runWorker(container)),
    );

    this.printTestResults();
  }

  async handleWorkerTimeout(
    duration: string,
    container: TestWorkerContainer,
  ): Promise<void> {
    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        container.bridge.end(
          `Test worker was unresponsive for ${duration}. We tried to collect some additional metadata but we timed out again trying to fetch it...`,
        );
        resolve();
      }, 3000);

      this._handleWorkerTimeout(duration, container)
        .then(() => {
          clearTimeout(timeout);
          resolve();
        })
        .catch(err => {
          clearTimeout(timeout);
          if (err instanceof InspectorClientCloseError) {
            container.bridge.end(
              `Test worker was unresponsive for ${duration}. We tried to collect some additional metadata but the inspector connection closed abruptly`,
            );
            resolve();
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

    const callFrames = params
      .get('callFrames')
      .asArray()
      .slice(0, 20);
    for (const callFrame of callFrames) {
      const loc = callFrame.get('location');

      const resolved = sourceMapManager.resolveLocation(
        urlToFilename(callFrame.get('url').asString()),
        coerce0to1(loc.get('lineNumber').asZeroIndexedNumber()),
        loc.get('columnNumber').asZeroIndexedNumber(),
      );

      const name = callFrame
        .get('scopeChain')
        .asArray()[0]
        .get('name')
        .asString('')
        .split('$')
        .pop();

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
      createErrorFromStructure({
        message: `Test worker was unresponsive for <emphasis>${duration}</emphasis>. Possible infinite loop. Below is a stack trace before the test was terminated.`,
        frames,
      }),
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
    this.progress.started++;

    let timeout = undefined;
    if (timeoutMs !== undefined) {
      timeout = setTimeout(() => {
        // TODO This will kill the whole worker, maybe it's possible to just terminate the current test? Throw an error, see if the next test was ran, or else terminate completely
        this.handleWorkerTimeout(`${String(timeoutMs)}ms`, container);
      }, timeoutMs);
    }

    this.runningTests.set(this.refToKey(ref), {
      ref,
      timeout,
    });
  }

  onTestFound(data: TestRef, isSkipped: boolean) {
    if (isSkipped) {
      return;
    }

    data;
    this.progress.total++;
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

    this.progress.finished++;
  }

  setupProgress() {
    const workers = this.getWorkers();

    for (let i = 0; i < workers.length; i++) {
      const container = workers[i];
      const {bridge} = container;

      const ourRunningTests: Set<string> = new Set();

      bridge.endEvent.subscribe(error => {
        // Fail all currently running tests
        for (const key of ourRunningTests) {
          const test = this.runningTests.get(key);
          if (test === undefined) {
            // Test has already finished
            continue;
          }

          const {ref} = test;
          this.onTestFinished(ref);
          this.printer.addDiagnostic(
            deriveDiagnosticFromError({
              error,
              category: ref.testName,
              filename: ref.filename,
            }),
            {
              category: 'test',
              message:
                'Worker died and this diagnostic was automatically generated to fail a test',
            },
          );
        }
      });

      bridge.testFound.subscribe(data => {
        this.onTestFound(data.ref, data.isSkipped);
      });

      bridge.testStart.subscribe(data => {
        ourRunningTests.add(this.refToKey(data.ref));
        this.onTestStart(container, data.ref, data.timeout);
      });

      bridge.testError.subscribe(data => {
        let origin: DiagnosticOrigin = {
          category: 'test',
          message:
            'Generated from a test worker without being attached to a test',
        };

        const {ref} = data;
        if (ref !== undefined) {
          const uid = this.master.projectManager.getUid(
            createAbsoluteFilePath(ref.filename),
          );
          origin.message = `Generated from the file <filelink target="${uid}" /> and test name "${ref.testName}"`;
          this.onTestFinished(ref);
        }

        this.printer.addDiagnostic(data.diagnostic, origin);
      });

      bridge.testSuccess.subscribe(data => {
        this.onTestFinished(data.ref);
      });
    }
  }

  printCoverageReport() {
    const {reporter, master} = this;

    // Fetch coverage entries
    const files = this.coverageCollector.generate();
    if (files.length === 0) {
      return undefined;
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
        name:
          root.name !== undefined && newRoot.name !== undefined
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
        return undefined;
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
          fileIndent + `<filelink target="${absolute}">${name}</filelink>`,
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
      reporter.spacer();
      reporter.info(
        'Additional coverage information available. Refine the executed tests or add the <emphasis>--show-all-coverage</emphasis> flag',
      );
    }

    reporter.hr();
  }

  printTestResults() {
    if (this.printer.hasDiagnostics()) {
      this.throwErrorDiagnosticsPrinter();
    } else {
      this.printTestSuccess();
    }
  }

  printTestSuccess() {
    this.printCoverageReport();
    this.reporter.success(
      `All <emphasis>${humanizeNumber(
        this.progress.total,
      )}</emphasis> tests passed!`,
    );
  }

  getSourceCode(filename: string): undefined | string {
    const testSource = this.sources.get(filename);
    if (testSource === undefined) {
      return undefined;
    } else {
      return testSource.code;
    }
  }

  throwErrorDiagnosticsPrinter() {
    const {printer} = this;

    // Only show code coverage for errors when `--show-all-coverage` has been passed
    if (this.options.showAllCoverage) {
      printer.onBeforeFooterPrint(() => {
        this.printCoverageReport();
      });
    }

    throw printer;
  }
}
