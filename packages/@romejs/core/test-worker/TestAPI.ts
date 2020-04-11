/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  DiagnosticAdvice,
  DiagnosticAdviceItem,
  getErrorStackAdvice,
} from '@romejs/diagnostics';
import SnapshotManager from './SnapshotManager';
import {TestRunnerOptions} from '../master/testing/types';
import {Event} from '@romejs/events';
import diff from '@romejs/string-diff';
import {createErrorFromStructure} from '@romejs/v8';
import prettyFormat from '@romejs/pretty-format';
import {Class} from '@romejs/typescript-helpers';
import {FileReference} from '../common/types/files';
import {markup} from '@romejs/string-markup';

type AsyncFunc = () => undefined | Promise<void>;

type SyncThrower = () => void;

type ExpectedError = undefined | string | RegExp | Class<Error>;

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

export default class TestAPI {
  constructor({
    testName,
    onTimeout,
    file,
    snapshotManager,
    options,
  }: {
    file: FileReference;
    testName: string;
    onTimeout: OnTimeout;
    snapshotManager: SnapshotManager;
    options: TestRunnerOptions;
  }) {
    this.testName = testName;
    this.options = options;
    this.snapshotManager = snapshotManager;
    this.snapshotCounter = 0;
    this.file = file;

    this.teardownEvent = new Event({name: 'TestAPI.teardown'});

    this.startTime = Date.now();
    this.onTimeout = onTimeout;
    this.timeoutMax = 0;
    this.timeoutId = undefined;
    this.setTimeout(5_000);

    this.advice = [];
  }

  startTime: number;
  options: TestRunnerOptions;
  file: FileReference;

  onTimeout: OnTimeout;
  timeoutId: undefined | NodeJS.Timeout;
  timeoutStart: undefined | number;
  timeoutMax: undefined | number;

  advice: DiagnosticAdvice;
  teardownEvent: Event<void, void>;
  testName: string;
  snapshotCounter: number;
  snapshotManager: SnapshotManager;

  buildMatchAdvice(received: unknown, expected: unknown, {
    visualMethod,
    expectedAlias,
    receivedAlias,
  }: {
    visualMethod?: string;
    expectedAlias?: string;
    receivedAlias?: string;
  } = {}): DiagnosticAdvice {
    let expectedFormat;
    let receivedFormat;
    if (typeof received === 'string' && typeof expected === 'string') {
      expectedFormat = expected;
      receivedFormat = received;
    } else {
      expectedFormat = prettyFormat(expected);
      receivedFormat = prettyFormat(received);
    }

    const advice: DiagnosticAdvice = [];

    if (expectedFormat === receivedFormat) {
      // Better error message when both values are visually identical
      advice.push({
        type: 'log',
        category: 'info',
        message: `Both the received and expected values are visually identical`,
      });

      advice.push({
        type: 'code',
        code: expectedFormat,
      });

      if (visualMethod !== undefined) {
        advice.push({
          type: 'log',
          category: 'info',
          message: `Try using t.${visualMethod} if you wanted a visual match`,
        });
      }
    } else {
      advice.push({
        type: 'log',
        category: 'info',
        message: `Expected to receive`,
      });

      advice.push({
        type: 'code',
        code: expectedFormat,
      });

      advice.push({
        type: 'log',
        category: 'info',
        message: `But got`,
      });

      advice.push({
        type: 'code',
        code: receivedFormat,
      });

      advice.push({
        type: 'log',
        category: 'info',
        message: 'Diff',
      });

      advice.push({
        type: 'diff',
        diff: diff(expectedFormat, receivedFormat),
        legend: {
          add: receivedAlias ? receivedAlias : 'What we received',
          delete: expectedAlias ? expectedAlias : 'What we expected',
        },
      });
    }

    return advice;
  }

  addToAdvice(item: DiagnosticAdviceItem): void {
    this.advice.push(item);
  }

  clearAdvice() {
    this.advice = [];
  }

  onTeardown(callback: AsyncFunc): void {
    this.teardownEvent.subscribe(callback);
  }

  clearTimeout(): void {
    if (this.timeoutId !== undefined) {
      clearTimeout(this.timeoutId);
    }

    this.timeoutMax = undefined;
    this.timeoutStart = undefined;
  }

  extendTimeout(time: number): void {
    const {timeoutMax, timeoutStart} = this;
    if (timeoutMax === undefined || timeoutStart === undefined) {
      throw new Error('No timeout set');
    }

    const elapsed = Date.now() - timeoutStart;
    const newTime = timeoutMax - elapsed + time;
    this.setTimeout(newTime);
  }

  setTimeout(time: number): void {
    this.clearTimeout();

    this.timeoutStart = Date.now();
    this.timeoutMax = time;

    this.timeoutId = setTimeout(() => {
      this.onTimeout(time);
    }, time);
  }

  checkTimeout(): void {
    const {startTime, timeoutMax} = this;
    if (timeoutMax === undefined) {
      return;
    }

    const delta = Date.now() - startTime;
    if (delta > timeoutMax) {
      throw new Error(`Test timeout - exceeded ${String(timeoutMax)}ms`);
    }
  }

  fail(
    message: string = 'Test failure triggered by t.fail()',
    advice: DiagnosticAdvice = [],
    framesToPop: number = 0,
  ): never {
    throw createErrorFromStructure({
      message,
      markupMessage: message,
      advice,
      framesToPop: framesToPop + 1,
    });
  }

  truthy(value: unknown, message: string = 'Expected value to be truthy'): void {
    if (Boolean(value) === false) {
      this.fail(message, [
        {
          type: 'log',
          category: 'info',
          message: `Received`,
        },
        {
          type: 'code',
          code: prettyFormat(value),
        },
      ], 1);
    }
  }

  falsy(value: unknown, message: string = 'Expected value to be falsy'): void {
    if (Boolean(value) === true) {
      this.fail(message, [
        {
          type: 'log',
          category: 'info',
          message: `Received`,
        },
        {
          type: 'code',
          code: prettyFormat(value),
        },
      ], 1);
    }
  }

  true(value: unknown, message: string = 'Expected value to be true'): void {
    if (value !== true) {
      this.fail(message, [
        {
          type: 'log',
          category: 'info',
          message: `Received`,
        },
        {
          type: 'code',
          code: prettyFormat(value),
        },
      ], 1);
    }
  }

  false(value: unknown, message: string = 'Expected value to be false'): void {
    if (value !== false) {
      this.fail(message, [
        {
          type: 'log',
          category: 'info',
          message: `Received`,
        },
        {
          type: 'code',
          code: prettyFormat(value),
        },
      ], 1);
    }
  }

  is(
    received: unknown,
    expected: unknown,
    message: string = 't.is() failed, using Object.is semantics',
  ): void {
    if (Object.is(received, expected) !== true) {
      this.fail(message, this.buildMatchAdvice(received, expected, {
        visualMethod: 'looksLike',
      }), 1);
    }
  }

  not(
    received: unknown,
    expected: unknown,
    message: string = 't.not() failed, using !Object.is() semantics',
  ): void {
    if (Object.is(received, expected) === true) {
      this.fail(message, this.buildMatchAdvice(received, expected, {
        visualMethod: 'notLooksLike',
      }), 1);
    }
  }

  looksLike(
    received: unknown,
    expected: unknown,
    message: string = 't.looksLike() failed, using prettyFormat semantics',
  ): void {
    const actualInspect = prettyFormat(received);
    const expectedInspect = prettyFormat(expected);

    if (actualInspect !== expectedInspect) {
      this.fail(message, this.buildMatchAdvice(received, expected), 1);
    }
  }

  notLooksLike(
    received: unknown,
    expected: unknown,
    message: string = 't.notLooksLike() failed, using !prettyFormat semantics',
  ): void {
    const actualInspect = prettyFormat(received);
    const expectedInspect = prettyFormat(expected);

    if (actualInspect === expectedInspect) {
      this.fail(message, this.buildMatchAdvice(received, expected), 1);
    }
  }

  throws(
    thrower: SyncThrower,
    expected?: ExpectedError,
    message: string = 't.throws() failed, callback did not throw an error',
  ): void {
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
  ): Promise<void> {
    throw new Error('unimplemented');
  }

  notThrows(nonThrower: SyncThrower, message?: string): void {
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

  regex(contents: string, regex: RegExp, message?: string): void {
    throw new Error('unimplemented');
  }

  notRegex(contents: string, regex: RegExp, message?: string): void {
    throw new Error('unimplemented');
  }

  snapshot(expected: unknown, message?: string): void {
    const id = this.snapshotCounter++;
    this._snapshotNamed(String(id), expected, message, 2);
  }

  snapshotNamed(name: string, expected: unknown, message?: string): void {
    this._snapshotNamed(name, expected, message, 1);
  }

  getSnapshot(snapshotName: string): unknown {
    return this.snapshotManager.get(this.testName, snapshotName);
  }

  _snapshotNamed(
    name: string,
    expected: unknown,
    message?: string,
    framesToPop?: number,
  ): void {
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
      return;
    }

    // Compare the snapshots
    if (formatted !== existingSnapshot) {
      const advice: DiagnosticAdvice = this.buildMatchAdvice(
        formatted,
        existingSnapshot,
        {
          receivedAlias: 'What the code gave us',
          expectedAlias: 'Existing snapshot',
        },
      );

      if (message === undefined) {
          message =
          markup`Snapshot ${name} at <filelink emphasis target="${this.snapshotManager.path.join()}" /> doesn't match`;
      } else {
        advice.push(
          {
            type: 'log',
            category: 'info',
            message: markup`Snapshot can be found at <filelink emphasis target="${this.snapshotManager.path.join()}" />`,
          },
        );
      }

      advice.push(
        {
          type: 'log',
          category: 'info',
          message: markup`Run <command>rome test <filelink target="${this.file.uid}" /> --update-snapshots</command> to update this snapshot`,
        },
      );

      this.fail(message, advice, framesToPop);
    }
  }
}
