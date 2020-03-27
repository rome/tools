/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {UnknownObject} from '@romejs/typescript-helpers';
import {
  DiagnosticAdvice,
  Diagnostic,
  getDiagnosticsFromError,
  INTERNAL_ERROR_LOG_ADVICE,
  createSingleDiagnosticError,
  descriptions,
  createBlessedDiagnosticMessage,
  deriveDiagnosticFromError,
} from '@romejs/diagnostics';
import {TestCallback, TestOptions, GlobalTestOptions} from '@romejs/test';
import {
  default as TestWorkerBridge,
  TestWorkerBridgeRunOptions,
} from '../common/bridges/TestWorkerBridge';
import {TestRunnerOptions} from '../master/testing/types';
import SnapshotManager from './SnapshotManager';
import TestAPI, {OnTimeout} from './TestAPI';
import executeMain from '../common/utils/executeMain';
import {
  FileReference,
  convertTransportFileReference,
} from '../common/types/files';
import {createAbsoluteFilePath, AbsoluteFilePath} from '@romejs/path';

const MAX_RUNNING_TESTS = 20;

export default class TestWorkerRunner {
  constructor(opts: TestWorkerBridgeRunOptions, bridge: TestWorkerBridge) {
    this.opts = opts;
    this.locked = false;
    this.file = convertTransportFileReference(opts.file);
    this.options = opts.options;
    this.bridge = bridge;
    this.projectFolder = createAbsoluteFilePath(opts.projectFolder);

    this.snapshotManager = new SnapshotManager(this, createAbsoluteFilePath(
      opts.file.real,
    ));

    this.hasFocusedTest = false;
    this.foundTests = new Map();
  }

  foundTests: Map<string, {
    callsiteError: Error;
    options: TestOptions;
    callback: undefined | TestCallback;
  }>;
  hasFocusedTest: boolean;

  bridge: TestWorkerBridge;
  projectFolder: AbsoluteFilePath;
  file: FileReference;
  options: TestRunnerOptions;
  snapshotManager: SnapshotManager;
  opts: TestWorkerBridgeRunOptions;
  locked: boolean;

  //  Global variables to expose to tests
  getEnvironment(): UnknownObject {
    const testOptions: GlobalTestOptions = {
      dirname: this.file.real.getParent().join(),
      register: (
        callsiteError: Error,
        opts: TestOptions,
        callback?: TestCallback,
      ) => {
        this.registerTest(callsiteError, opts, callback);
      },
    };

    return {
      __ROME__TEST_OPTIONS__: testOptions,
    };
  }

  async emitDiagnostic(diagnostic: Diagnostic) {
    await this.bridge.testError.call({
      ref: undefined,
      diagnostic,
    });
  }

  // execute the test file and discover tests
  async discoverTests() {
    const {code, sourceMap} = this.opts;

    const res = await executeMain({
      path: this.file.real,
      code,
      sourceMap,
      globals: this.getEnvironment(),
    });

    if (res.syntaxError !== undefined) {
      const message =
        `A bundle was generated that contained a syntax error: ${res.syntaxError.description.message.value}`;

      throw createSingleDiagnosticError({
        ...res.syntaxError,
        description: {
          ...res.syntaxError.description,
          message: createBlessedDiagnosticMessage(message),
          advice: [INTERNAL_ERROR_LOG_ADVICE],
        },
        location: {
          ...res.syntaxError.location,
          filename: this.file.uid,
        },
      });
    }
  }

  lockTests() {
    this.locked = true;
  }

  registerTest(
    callsiteError: Error,
    options: TestOptions,
    callback: undefined | TestCallback,
  ) {
    if (this.locked) {
      throw new Error('Test can\'t be added outside of init');
    }

    let testName = options.name;
    if (Array.isArray(testName)) {
      testName = testName.join(' > ');
    }

    if (this.foundTests.has(testName)) {
      throw new Error(`Test ${testName} has already been defined`);
    }

    this.foundTests.set(testName, {
      callback,
      options,
      callsiteError,
    });

    if (options.only === true) {
      this.hasFocusedTest = true;
    }
  }

  onError(
    testName: undefined | string,
    opts: {
      error: Error;
      firstAdvice: DiagnosticAdvice;
      lastAdvice: DiagnosticAdvice;
    },
  ) {
    const filename = this.file.real.join();

    let ref = undefined;
    if (testName === undefined) {
      testName = 'unknown';
    } else {
      ref = {
        filename,
        testName,
      };
    }

    let diagnostic: Diagnostic = deriveDiagnosticFromError({
      error: opts.error,
      category: 'tests/failure',
      label: testName,
      filename,
      cleanFrames(frames) {
        // TODO we should actually get the frames before module init and do it that way

        // Remove everything before the original module factory
        let latestTestWorkerFrame = frames.find((frame, i) => {
          if (frame.typeName === 'global' && frame.methodName === undefined &&
            frame.functionName === undefined) {
            // We are the global.<anonymous> frame

            // Now check for Script.runInContext
            const nextFrame = frames[i + 1];
            if (nextFrame !== undefined && nextFrame.typeName === 'Script' &&
              nextFrame.methodName === 'runInContext') {
              // Yes!

              // TODO also check for ___$romejs$core$common$utils$executeMain_ts$default (packages/romejs/core/common/utils/executeMain.ts:69:17)
              return true;
            }
          }

          return false;
        });

        // And if there was no module factory frame, then we must be inside of a test
        if (latestTestWorkerFrame === undefined) {
          latestTestWorkerFrame = frames.find((frame) => {
            return frame.filename !== undefined && frame.filename.includes(
              'core/test-worker',
            );
          });
        }

        if (latestTestWorkerFrame === undefined) {
          return frames;
        }

        return frames.slice(0, frames.indexOf(latestTestWorkerFrame));
      },
    });

    diagnostic = {
      ...diagnostic,
      description: {
        ...diagnostic.description,
        advice: [
          ...opts.firstAdvice,
          ...(diagnostic.description.advice || []),
          ...opts.lastAdvice,
        ],
      },
    };

    this.bridge.testError.send({
      ref,
      diagnostic,
    });
  }

  async teardownTest(testName: string, api: TestAPI) {
    api.clearTimeout();

    try {
      await api.teardownEvent.callOptional();
    } catch (err) {
      this.onError(testName, {
        error: err,
        firstAdvice: [],
        lastAdvice: [
          {
            type: 'log',
            category: 'info',
            message: `Error occured while running <emphasis>teardown</emphasis> for test <emphasis>${testName}</emphasis>`,
          },
        ],
      });
    }
  }

  async runTest(testName: string, callback: TestCallback) {
    let onTimeout: OnTimeout = () => {
      throw new Error('Promise wasn\'t created. Should be impossible.');
    };

    const timeoutPromise = new Promise((resolve, reject) => {
      onTimeout = (time: number) => {
        reject(new Error(`Test timeout - exceeded ${String(time)}ms`));
      };
    });

    const api = new TestAPI({
      file: this.file,
      testName,
      onTimeout,
      snapshotManager: this.snapshotManager,
      options: this.options,
    });

    try {
      const res = callback(api);

      // Ducktyping this to detect a cross-realm Promise
      if (res !== undefined && typeof res.then === 'function') {
        await Promise.race([timeoutPromise, res]);
      }

      this.bridge.testSuccess.send({
        ref: {
          filename: this.file.real.join(),
          testName,
        },
      });
    } catch (err) {
      this.onError(testName, {
        error: err,
        firstAdvice: [],
        lastAdvice: api.advice,
      });
    } finally {
      await this.teardownTest(testName, api);
    }
  }

  async run(): Promise<void> {
    const promises: Set<Promise<void>> = new Set();

    const {foundTests} = this;
    if (foundTests.size === 0) {
      this.bridge.testError.send({
        ref: undefined,
        diagnostic: {
          location: {
            filename: this.file.uid,
          },
          description: descriptions.TESTS.UNDECLARED,
        },
      });
    }

    // Execute all the tests
    for (const [testName, {options, callback}] of foundTests) {
      if (callback === undefined) {
        continue;
      }

      this.bridge.testStart.send({
        ref: {
          filename: this.file.real.join(),
          testName,
        },
        timeout: options.timeout,
      });

      const promise = this.runTest(testName, callback);

      if (this.options.syncTests) {
        await promise;
      } else {
        promise.then(() => {
          promises.delete(promise);
        });
        promises.add(promise);

        // if there's 5 promises, then wait for one of them to finish
        if (promises.size > MAX_RUNNING_TESTS) {
          await Promise.race(Array.from(promises));
        }
      }
    }

    // Execute the remaining tests
    await Promise.all(Array.from(promises));

    // Save the snapshot
    await this.snapshotManager.save();
  }

  async emitFoundTests() {
    const tests = [];

    for (const [testName, {callback, options}] of this.foundTests) {
      let isSkipped = callback === undefined;
      if (this.hasFocusedTest && options.only !== true) {
        isSkipped = true;
      }

      tests.push({
        ref: {
          filename: this.file.real.join(),
          testName,
        },
        isSkipped,
      });
    }

    await this.bridge.testsFound.call(tests);
  }

  async wrap(callback: () => Promise<void>): Promise<void> {
    try {
      await callback();
    } catch (err) {
      const diagnostics = getDiagnosticsFromError(err);
      if (diagnostics === undefined) {
        this.onError(undefined, {
          error: err,
          firstAdvice: [],
          lastAdvice: [
            {
              type: 'log',
              category: 'info',
              message: `Error occured while executing test file <filelink emphasis target="${this.file.uid}" />`,
            },
            INTERNAL_ERROR_LOG_ADVICE,
          ],
        });
      } else {
        for (const diagnostic of diagnostics) {
          await this.bridge.testError.call({
            ref: undefined,
            diagnostic,
          });
        }
      }
    }
  }

  async prepare(): Promise<void> {
    return this.wrap(async () => {
      // Setup
      await this.snapshotManager.load();
      await this.discoverTests();
      await this.emitFoundTests();

      // Execute
      this.lockTests();
    });
  }
}
