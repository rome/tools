/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostic, DiagnosticOrigin} from '@romejs/diagnostics';
import {TestMasterRunnerOptions} from '../../master/testing/types';
import {Bridge} from '@romejs/events';
import {JSONFileReference} from '../types/files';
import {TestWorkerFileResult} from '@romejs/core/test-worker/TestWorkerRunner';

export type TestRef = {
  filename: string;
  testName: string;
};

export type TestWorkerBridgeRunOptions = {
  id: number;
  file: JSONFileReference;
  projectFolder: string;
  code: string;
  cwd: string;
  options: TestMasterRunnerOptions;
};

export default class TestWorkerBridge extends Bridge {
  inspectorDetails = this.createEvent<
    void,
    {
      inspectorUrl: undefined | string;
    }
  >({
    name: 'inspectorDetails',
    direction: 'server->client',
  });

  prepareTest = this.createEvent<TestWorkerBridgeRunOptions, void>({
    name: 'prepareTest',
    direction: 'server->client',
  });

  runTest = this.createEvent<number, TestWorkerFileResult>({
    name: 'runTest',
    direction: 'server->client',
  });

  testsFound = this.createEvent<
    Array<{
      ref: TestRef;
      isSkipped: boolean;
    }>,
    void
  >({
    name: 'onTestFounds',
    direction: 'server<-client',
  });

  testStart = this.createEvent<
    {
      ref: TestRef;
      timeout: undefined | number;
    },
    void
  >({
    name: 'onTestStart',
    direction: 'server<-client',
  });

  testDiagnostic = this.createEvent<
    {
      diagnostic: Diagnostic;
      origin: undefined | DiagnosticOrigin;
    },
    void
  >({
    name: 'testDiagnostic',
    direction: 'server<-client',
  });

  testFinish = this.createEvent<
    {
      success: boolean;
      ref: TestRef;
    },
    void
  >({
    name: 'onTestSuccess',
    direction: 'server<-client',
  });
}
