/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Profile} from '@romejs/v8';
import {Diagnostics} from '@romejs/diagnostics';
import {ClientFlags, ClientRequestFlags} from '../types/client';
import {Bridge} from '@romejs/events';
import {JSONPropertyValue} from '@romejs/codec-json';
import {
  RemoteReporterClientMessage,
  RemoteReporterReceiveMessage,
  ReporterStream,
} from '@romejs/cli-reporter';
import {MasterMarker} from '../../master/Master';
import {Dict} from '@romejs/typescript-helpers';

export type MasterQueryRequest = {
  requestFlags: ClientRequestFlags;
  commandFlags: Dict<unknown>;
  args: Array<string>;
  commandName: string;
  silent: boolean;
  noData: boolean;
  terminateWhenIdle: boolean;
};

export type PartialMasterQueryRequest =
  & Partial<Omit<MasterQueryRequest, 'requestFlags'>>
  & {
    requestFlags?: Partial<ClientRequestFlags>;
    command: string;
  };

export type MasterQueryResponseSuccess = {
  type: 'SUCCESS';
  hasData: boolean;
  data: JSONPropertyValue;
  markers: Array<MasterMarker>;
};

export type MasterQueryResponseError = {
  type: 'ERROR';
  fatal: boolean;
  handled: boolean;
  name: string;
  message: string;
  stack: undefined | string;
};

export type MasterQueryResponseDiagnostics = {
  type: 'DIAGNOSTICS';
  diagnostics: Diagnostics;
};

export type MasterQueryResponseInvalid = {
  type: 'INVALID_REQUEST';
  diagnostics: Diagnostics;
};

export type MasterQueryResponse =
  | MasterQueryResponseInvalid
  | MasterQueryResponseSuccess
  | MasterQueryResponseError
  | MasterQueryResponseDiagnostics;

export type ProfilingStartData = {samplingInterval: number};

export type MasterBridgeJSONFlags = Omit<ClientFlags, 'cwd'> & {cwd: string};

export type MasterBridgeInfo = {
  version: string;
  columns: number;
  hasClearScreen: boolean;
  useRemoteReporter: boolean;
  format: ReporterStream['format'];
  flags: MasterBridgeJSONFlags;
};

export default class MasterBridge extends Bridge {
  getClientInfo = this.createEvent<void, MasterBridgeInfo>({
    name: 'getClientInfo',
    direction: 'server->client',
  });

  stdout = this.createEvent<string, void>({
    name: 'stdout',
    direction: 'server->client',
  });

  stderr = this.createEvent<string, void>({
    name: 'stderr',
    direction: 'server->client',
  });

  enableWorkerLogs = this.createEvent<void, void>({
    name: 'enableWorkerLogs',
    direction: 'server<-client',
  });

  log = this.createEvent<{
    origin: 'master' | 'worker';
    chunk: string;
  }, void>({
    name: 'log',
    direction: 'server->client',
  });

  setColumns = this.createEvent<number, void>({
    name: 'columns.set',
    direction: 'server<-client',
  });

  reporterRemoteServerMessage = this.createEvent<
    RemoteReporterClientMessage,
    void
  >({
    name: 'reporterRemoteToLocalMessage',
    direction: 'server->client',
  });

  reporterRemoteClientMessage = this.createEvent<
    RemoteReporterReceiveMessage,
    void
  >({
    name: 'reporterLocalToRemoteMessage',
    direction: 'server<-client',
  });

  query = this.createEvent<PartialMasterQueryRequest, MasterQueryResponse>({
    name: 'query',
    direction: 'server<-client',
  });

  profilingGetWorkers = this.createEvent<void, Array<number>>({
    name: 'profiling.getWorkers',
    direction: 'server<-client',
  });

  profilingStart = this.createEvent<ProfilingStartData, void>({
    name: 'profiling.start',
    direction: 'server<-client',
  });

  profilingStop = this.createEvent<void, Profile>({
    name: 'profiling.stop',
    direction: 'server<-client',
  });

  profilingStopWorker = this.createEvent<number, Profile>({
    name: 'profile.stopWorker',
    direction: 'server<-client',
  });
}
