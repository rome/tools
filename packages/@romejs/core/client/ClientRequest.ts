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
import {LocalCommand, localCommands} from './commands';
import Client from './Client';
import {consumeUnknown} from '@romejs/consume';
import {BridgeError} from '@romejs/events';
import review from './review';

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

  fork(query: PartialMasterQueryRequest): ClientRequest {
    return new ClientRequest(this.client, this.type, query);
  }

  async init(): Promise<MasterQueryResponse> {
    try {
      const {requestFlags} = this.query;
      if (requestFlags !== undefined && requestFlags.review) {
        return await this.initReview();
      } else {
        return await this.initCommand();
      }
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

  async initReview(): Promise<MasterQueryResponse> {
    return review(this);
  }

  async initCommand(): Promise<MasterQueryResponse> {
    const localCommand = localCommands.get(this.query.command);

    if (this.type === 'master' || localCommand === undefined) {
      return this.initFromMaster();
    } else {
      return this.initFromLocal(localCommand);
    }
  }

  async initFromLocal( // rome-suppress-next-line lint/noExplicitAny
  localCommand: LocalCommand<any>): Promise<MasterQueryResponse> {
    const {query} = this;

    let flags;
    if (localCommand.defineFlags !== undefined) {
      flags = localCommand.defineFlags(consumeUnknown(
        query.commandFlags,
        'flags/invalid',
      ));
    }

    const res = await localCommand.callback(this, flags);
    if (res === true) {
      return {
        type: 'SUCCESS',
        data: undefined,
        hasData: false,
        markers: [],
      };
    } else if (res === false) {
      return {
        type: 'ERROR',
        fatal: false,
        // Local command would have printed something
        handled: true,
        name: 'Error',
        message: 'Command was not successful',
        stack: undefined,
      };
    } else {
      return res;
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
