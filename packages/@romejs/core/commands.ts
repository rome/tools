/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer} from '@romejs/consume';
import MasterRequest from './master/MasterRequest';
import ClientRequest from './client/ClientRequest';
import {ClientFlags, ClientRequestFlags} from './common/types/client';
import {JSONPropertyValue} from '@romejs/codec-json';

export type MasterCommand<T> = {
  category: string;
  description: string;
  overrideClientFlags?: Partial<ClientFlags>;
  overrideRequestFlags?: Partial<ClientRequestFlags>;
  usage?: string;
  examples?: Array<{
    description: string;
    command: string;
  }>;
  defineFlags?: (c: Consumer) => T;
  default: (
    req: MasterRequest,
    commandFlags: T,
  ) => undefined | Promise<JSONPropertyValue>;
};

export type LocalCommand<T> = {
  category: string;
  description: string;
  usage?: string;
  examples?: Array<{
    description: string;
    command: string;
  }>;
  callback: (api: ClientRequest, commandFlags: T) => Promise<boolean>;
  defineFlags?: (c: Consumer) => T;
};

export function createMasterCommand<T>(
  cmd: MasterCommand<T>,
): MasterCommand<T> {
  return cmd;
}

export const commandCategories = {
  PROCESS_MANAGEMENT: 'Process Management',
  CODE_QUALITY: 'Code Quality',
  SOURCE_CODE: 'Source Code',
  PROJECT_MANAGEMENT: 'Project Management',
  SOURCE_CONTROL: 'Source Control',
  INTERNAL: 'Internal',
};
