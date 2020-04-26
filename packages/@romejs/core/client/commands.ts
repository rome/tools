/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import init from './commands/init';
import start from './commands/start';
import develop from './commands/develop';
import stop from './commands/stop';
import run from './commands/run';
import restart from './commands/restart';
import status from './commands/status';
import lsp from './commands/lsp';

//
import {Dict} from '@romejs/typescript-helpers';
import ClientRequest from './ClientRequest';
import {SharedCommand} from '../common/commands';
import {MasterQueryResponse} from '../common/bridges/MasterBridge';

export type LocalCommand<Flags extends Dict<unknown>> = SharedCommand<Flags> & {
  callback: (
    req: ClientRequest,
    commandFlags: Flags,
  ) => Promise<boolean | MasterQueryResponse>;
};

export function createLocalCommand<Flags extends Dict<unknown>>(
  cmd: LocalCommand<Flags>,
): LocalCommand<Flags> {
  return cmd;
}

// rome-suppress-next-line lint/noExplicitAny
export const localCommands: Map<string, LocalCommand<any>> = new Map();
localCommands.set('init', init);
localCommands.set('start', start);
localCommands.set('develop', develop);
localCommands.set('stop', stop);
localCommands.set('run', run);
localCommands.set('restart', restart);
localCommands.set('status', status);
localCommands.set('lsp', lsp);
