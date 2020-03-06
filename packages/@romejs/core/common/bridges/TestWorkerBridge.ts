/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {PartialDiagnostic} from '@romejs/diagnostics';
import {SourceMap} from '@romejs/codec-source-map';
import {TestRunnerOptions} from '../../master/testing/types';
import {Bridge} from '@romejs/events';
import {JSONFileReference} from '../types/files';
import {AbsoluteFilePath} from '@romejs/path';

export type TestRef = {
  filename: string;
  testName: string;
};

export type TestWorkerBridgeRunOptions = {
  file: JSONFileReference;
  projectFolder: string;
  code: string;
  cwd: string;
  sourceMap: SourceMap;
  options: TestRunnerOptions;
};

export default class TestWorkerBridge extends Bridge {
  inspectorDetails = this.createEvent<void, {inspectorUrl: undefined | string}>(
    {
      name: 'inspectorDetails',
      direction: 'server->client',
    },
  );

  runTest = this.createEvent<TestWorkerBridgeRunOptions, void>({
    name: 'runTest',
    direction: 'server->client',
  });

  testFound = this.createEvent<{ref: TestRef; isSkipped: boolean}, void>({
    name: 'onTestFound',
    direction: 'server<-client',
  });

  testStart = this.createEvent<
    {ref: TestRef; timeout: undefined | number},
    void
  >({
    name: 'onTestStart',
    direction: 'server<-client',
  });

  testError = this.createEvent<
    {
      ref: undefined | TestRef;
      diagnostic: PartialDiagnostic;
    },
    void
  >({name: 'onTestError', direction: 'server<-client'});

  testSuccess = this.createEvent<{ref: TestRef}, void>({
    name: 'onTestSuccess',
    direction: 'server<-client',
  });
}
