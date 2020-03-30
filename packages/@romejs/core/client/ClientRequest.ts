/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  MasterQueryResponse,
  PartialMasterQueryRequest,
} from '../common/bridges/MasterBridge';
import {LocalCommand} from '../commands';
import Client from './Client';
import {consumeUnknown} from '@romejs/consume';
import {BridgeError} from '@romejs/events';
import {localCommands} from './commands';
export type ClientRequestType = 'local' | 'master';

export default class ClientRequest {
  constructor(
    client: Client,
    type: ClientRequestType = 'local',
    query: PartialMasterQueryRequest,
  ) {
    this.client = client;
    this.type = type;
    this.query = query;
  }

  query: PartialMasterQueryRequest;
  type: ClientRequestType;
  client: Client;

  async init(): Promise<MasterQueryResponse> {
    try {
      return await this.initCommand();
    } catch (err) {
      return {
        type: 'ERROR',
        fatal: false,
        handled: false,
        name: err.name,
        message: err.message,
        stack: err.stack,
      };
    }
  }

  async initCommand(): Promise<MasterQueryResponse> {
    const localCommand = localCommands.get(this.query.command);

    if (this.type === 'master' || localCommand === undefined) {
      return this.initFromMaster();
    } else {
      return this.initFromLocal(localCommand);
    }
  }

  async initFromLocal(
    // rome-suppress-next-line lint/noExplicitAny
    localCommand: LocalCommand<any>,
  ): Promise<MasterQueryResponse> {
    const {query} = this;

    let flags;
    if (localCommand.defineFlags !== undefined) {
      flags = localCommand.defineFlags(consumeUnknown(
        query.commandFlags,
        'flags/invalid',
      ));
    }

    const success = await localCommand.callback(this, flags);
    if (success) {
      return {
        type: 'SUCCESS',
        data: undefined,
        hasData: false,
        markers: [],
      };
    } else {
      return {
        type: 'ERROR',
        fatal: false,
        // Local command would have printed something
        handled: true,
        name: 'Error',
        message: 'Command was not successful',
        stack: undefined,
      };
    }
  }

  async initFromMaster(): Promise<MasterQueryResponse> {
    const {client} = this;

    try {
      const bridge = await client.findOrStartMaster();
      return await bridge.query.call(this.query);
    } catch (err) {
      if (err instanceof BridgeError) {
        return {
          type: 'ERROR',
          fatal: true,
          handled: false,
          name: 'Error',
          message: 'Server died while processing command. Results may be incomplete.',
          stack: undefined,
        };
      } else {
        throw err;
      }
    }
  }
}
