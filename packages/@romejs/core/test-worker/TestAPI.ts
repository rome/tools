/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  PartialDiagnosticAdvice,
  PartialDiagnosticAdviceItem,
  getErrorStackAdvice,
} from '@romejs/diagnostics';
import SnapshotManager from './SnapshotManager';
import {TestRunnerOptions} from '../master/testing/types';
import {Event} from '@romejs/events';
import diff from '@romejs/string-diff';
import {createErrorFromStructure} from '@romejs/v8';
import prettyFormat from '@romejs/pretty-format';
import {Class} from '@romejs/typescript-helpers';

type AsyncFunc = () => undefined | Promise<void>;
type SyncThrower = () => void;
type ExpectedError = undefined | string | RegExp | Class<Error>;

function removeCRLF(str: string): string {
  return str.replace(/\r/g, '');
}

function formatExpectedError(expected: ExpectedError): string {
  if (typeof expected === 'string') {
    return JSON.stringify(expected);
  }

  if (expected instanceof RegExp) {
    return String(expected);
  }

  if (typeof expected === 'function') {
    return expected.name;
  }

  return 'unknown';
}

function matchExpectedError(error: Error, expected: ExpectedError): boolean {
  if (expected === undefined) {
    return true;
  }

  if (typeof expected === 'string') {
    return error.message.includes(expected);
  }

  if (expected instanceof RegExp) {
    return expected.test(error.message);
  }

  if (typeof expected === 'function') {
    return error instanceof expected;
  }

  return false;
}

export type OnTimeout = (time: number) => void;

const TRUNCATION_MATCH_LIMIT = 500;

function maybeTruncate(str: string, noTruncate: boolean): string {
  if (noTruncate || str.length < TRUNCATION_MATCH_LIMIT) {
    return str;
  } else {
    return `${str.slice(0, TRUNCATION_MATCH_LIMIT)}...`;
  }
}

export default class TestAPI {
  constructor(
    testName: string,
    onTimeout: OnTimeout,
    snapshotManager: SnapshotManager,
    options: TestRunnerOptions,
  ) {
    this.testName = testName;
    this.options = options;
    this.snapshotManager = snapshotManager;
    this.snapshotCounter = 0;

    this.teardownEvent = new Event({name: 'TestAPI.teardown'});

    this.startTime = Date.now();
    this.onTimeout = onTimeout;
    this.timeoutMax = 0;
    this.timeoutId = undefined;
    this.setTimeout(5000);

    this.advice = [];
  }

  startTime: number;
  options: TestRunnerOptions;

  onTimeout: OnTimeout;
  timeoutId: undefined | NodeJS.Timeout;
  timeoutStart: undefined | number;
  timeoutMax: undefined | number;

  advice: PartialDiagnosticAdvice;
  teardownEvent: Event<void, void>;
  testName: string;
  snapshotCounter: number;
  snapshotManager: SnapshotManager;

  buildMatchParts(
    received: unknown,
    expected: unknown,
    visualMethod?: string,
  ): PartialDiagnosticAdvice {
    let expectedFormat;
    let receivedFormat;
    if (typeof received === 'string' && typeof expected === 'string') {
      expectedFormat = expected;
      receivedFormat = received;
    } else {
      expectedFormat = prettyFormat(expected);
      receivedFormat = prettyFormat(received);
    }

    const expectedFormatCode = maybeTruncate(
      expectedFormat,
      this.options.verboseDiagnostics,
    );
    const receivedFormatCode = maybeTruncate(
      receivedFormat,
      this.options.verboseDiagnostics,
    );
    const hasTruncated =
      expectedFormatCode !== expectedFormat ||
      receivedFormatCode !== receivedFormat;
    const hasAllTruncated =
      expectedFormatCode !== expectedFormat &&
      receivedFormatCode !== receivedFormat;

    const advice: PartialDiagnosticAdvice = [];

    if (expectedFormat === receivedFormat) {
      // Better error message when both values are visually identical
      advice.push({
        type: 'log',
        category: 'info',
        message: `Both the received and expected values are visually identical`,
      });

      advice.push({
        type: 'code',
        code: expectedFormatCode,
      });

      if (visualMethod !== undefined) {
        advice.push({
          type: 'log',
          category: 'info',
          message: `Try using t.${visualMethod} if you wanted a visual match`,
        });
      }
    } else {
      if (expectedFormat.trim() === receivedFormat.trim()) {
        advice.push({
          type: 'log',
          category: 'info',
          message: 'Only difference is leading and trailing whitespace',
        });
      }

      const expectedFormatNoCRLF = removeCRLF(expectedFormat);
      const receivedFormatNoCRLF = removeCRLF(receivedFormat);
      if (expectedFormat === receivedFormatNoCRLF) {
        advice.push({
          type: 'log',
          category: 'info',
          message:
            'Identical except the received uses CRLF newlines, while the expected does not',
        });
      }
      if (receivedFormat === expectedFormatNoCRLF) {
        advice.push({
          type: 'log',
          category: 'info',
          message:
            'Identical except the expected uses CRLF newlines, while the received does not',
        });
      }

      if (!hasAllTruncated) {
        // TODO detect newlines

        // If there was no truncation then display the full code of both values
        advice.push({
          type: 'log',
          category: 'info',
          message: `Expected to receive`,
        });

        advice.push({
          type: 'code',
          code: expectedFormatCode,
        });

        advice.push({
          type: 'log',
          category: 'info',
          message: `But got`,
        });

        advice.push({
          type: 'code',
          code: receivedFormatCode,
        });
      }

      // Produce a diff to better visualize differences
      // TODO what about truncation...?

      advice.push({
        type: 'log',
        category: 'info',
        message: `Diff`,
      });

      advice.push({
        type: 'diff',
        diff: diff(receivedFormat, expectedFormat),
      });
    }

    // If there was truncation then warn
    if (hasAllTruncated) {
      advice.push({
        type: 'log',
        category: 'info',
        message:
          'Add the --verbose-diagnostics flag to show the values being compared',
      });
    } else if (hasTruncated) {
      advice.push({
        type: 'log',
        category: 'info',
        message:
          'Some values have been truncated for being too long, add the --verbose-diagnostics flag to disable truncation',
      });
    }

    return advice;
  }

  addToAdvice(item: PartialDiagnosticAdviceItem) {
    this.advice.push(item);
  }

  onTeardown(callback: AsyncFunc) {
    this.teardownEvent.subscribe(callback);
  }

  clearTimeout() {
    if (this.timeoutId !== undefined) {
      clearTimeout(this.timeoutId);
    }

    this.timeoutMax = undefined;
    this.timeoutStart = undefined;
  }

  extendTimeout(time: number) {
    const {timeoutMax, timeoutStart} = this;
    if (timeoutMax === undefined || timeoutStart === undefined) {
      throw new Error('No timeout set');
    }

    const elapsed = Date.now() - timeoutStart;
    const newTime = timeoutMax - elapsed + time;
    this.setTimeout(newTime);
  }

  setTimeout(time: number) {
    this.clearTimeout();

    this.timeoutStart = Date.now();
    this.timeoutMax = time;

    this.timeoutId = setTimeout(() => {
      this.onTimeout(time);
    }, time);
  }

  checkTimeout() {
    const {startTime, timeoutMax} = this;
    if (timeoutMax === undefined) {
      return undefined;
    }

    const delta = Date.now() - startTime;
    if (delta > timeoutMax) {
      throw new Error(`Test timeout - exceeded ${String(timeoutMax)}ms`);
    }
  }

  fail(
    message?: string,
    advice?: PartialDiagnosticAdvice,
    framesToPop: number = 0,
  ) {
    const actualMessage =
      message === undefined ? 'Test failure triggered by t.fail()' : message;
    const error = createErrorFromStructure({
      message: actualMessage,
      advice,
      framesToPop: framesToPop + 1,
    });
    throw error;
  }

  truthy(value: unknown, message: string = 'Expected value to be truthy') {
    if (Boolean(value) === false) {
      this.fail(
        message,
        [
          {
            type: 'log',
            category: 'info',
            message: `Received`,
          },
          {
            type: 'code',
            code: prettyFormat(value),
          },
        ],
        1,
      );
    }
  }

  falsy(value: unknown, message: string = 'Expected value to be falsy') {
    if (Boolean(value) === true) {
      this.fail(
        message,
        [
          {
            type: 'log',
            category: 'info',
            message: `Received`,
          },
          {
            type: 'code',
            code: prettyFormat(value),
          },
        ],
        1,
      );
    }
  }

  true(value: unknown, message: string = 'Expected value to be true') {
    if (value !== true) {
      this.fail(
        message,
        [
          {
            type: 'log',
            category: 'info',
            message: `Received`,
          },
          {
            type: 'code',
            code: prettyFormat(value),
          },
        ],
        1,
      );
    }
  }

  false(value: unknown, message: string = 'Expected value to be false') {
    if (value !== false) {
      this.fail(
        message,
        [
          {
            type: 'log',
            category: 'info',
            message: `Received`,
          },
          {
            type: 'code',
            code: prettyFormat(value),
          },
        ],
        1,
      );
    }
  }

  is(
    received: unknown,
    expected: unknown,
    message: string = 't.is() failed, using Object.is semantics',
  ) {
    if (Object.is(received, expected) !== true) {
      this.fail(
        message,
        this.buildMatchParts(received, expected, 'looksLike'),
        1,
      );
    }
  }

  not(
    received: unknown,
    expected: unknown,
    message: string = 't.not() failed, using !Object.is() semantics',
  ) {
    if (Object.is(received, expected) === true) {
      this.fail(
        message,
        this.buildMatchParts(received, expected, 'notLooksLike'),
        1,
      );
    }
  }

  looksLike(
    received: unknown,
    expected: unknown,
    message: string = 't.looksLike() failed, using prettyFormat semantics',
  ) {
    const actualInspect = prettyFormat(received);
    const expectedInspect = prettyFormat(expected);

    if (actualInspect !== expectedInspect) {
      this.fail(message, this.buildMatchParts(received, expected), 1);
    }
  }

  notLooksLike(
    received: unknown,
    expected: unknown,
    message: string = 't.notLooksLike() failed, using !prettyFormat semantics',
  ) {
    const actualInspect = prettyFormat(received);
    const expectedInspect = prettyFormat(expected);

    if (actualInspect === expectedInspect) {
      this.fail(message, this.buildMatchParts(received, expected), 1);
    }
  }

  throws(
    thrower: SyncThrower,
    expected?: ExpectedError,
    message: string = 't.throws() failed, callback did not throw an error',
  ) {
    try {
      thrower();
    } catch (err) {
      if (matchExpectedError(err, expected)) {
        return undefined;
      } else {
        this.fail(
          `t.throws() expected an error to be thrown that matches ${formatExpectedError(
            expected,
          )} but got ${err.name}: ${JSON.stringify(err.message)}`,
          getErrorStackAdvice(err, 'Incorrect error stack trace'),
          1,
        );
      }
    }

    this.fail(message, undefined, 1);
  }

  async throwsAsync(
    thrower: AsyncFunc,
    expected?: ExpectedError,
    message?: string,
  ) {
    throw new Error('unimplemented');
  }

  notThrows(nonThrower: SyncThrower, message?: string) {
    try {
      nonThrower();
    } catch (err) {
      // TODO
      message;
      throw err;
    }
  }

  async notThrowsAsync(nonThrower: AsyncFunc, message?: string): Promise<void> {
    throw new Error('unimplemented');
  }

  regex(contents: string, regex: RegExp, message?: string) {
    throw new Error('unimplemented');
  }

  notRegex(contents: string, regex: RegExp, message?: string) {
    throw new Error('unimplemented');
  }

  snapshot(expected: unknown, message?: string) {
    const id = this.snapshotCounter++;
    return this._snapshotNamed(String(id), expected, message, 2);
  }

  snapshotNamed(name: string, expected: unknown, message?: string) {
    return this._snapshotNamed(name, expected, message, 1);
  }

  getSnapshot(snapshotName: string): unknown {
    return this.snapshotManager.get(this.testName, snapshotName);
  }

  _snapshotNamed(
    name: string,
    expected: unknown,
    message: string = "Snapshots don't match",
    framesToPop?: number,
  ) {
    let language: undefined | string;

    let formatted = '';
    if (typeof expected === 'string') {
      formatted = expected;
    } else {
      language = 'javascript';
      formatted = prettyFormat(expected);
    }

    // Get the current snapshot
    const existingSnapshot = this.snapshotManager.get(this.testName, name);
    if (existingSnapshot === undefined) {
      // No snapshot exists, let's save this one!
      this.snapshotManager.set({
        testName: this.testName,
        snapshotName: String(name),
        value: formatted,
        language,
      });
      return undefined;
    }

    // Compare the snapshots
    if (formatted !== existingSnapshot) {
      this.fail(
        message,
        [
          {
            type: 'log',
            category: 'info',
            message: 'Snapshot diff',
          },
          {
            type: 'diff',
            diff: diff(existingSnapshot, formatted),
          },
        ],
        framesToPop,
      );
    }
  }
}
