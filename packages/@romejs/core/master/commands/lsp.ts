/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {commandCategories} from '../../common/commands';
import {createMasterCommand} from '../commands';
import LSPServer from '../lsp/LSPServer';

export default createMasterCommand({
  category: commandCategories.PROJECT_MANAGEMENT,
  description: 'TODO',
  usage: '',
  examples: [],

  defineFlags() {
    return {};
  },

  async callback(req: MasterRequest): Promise<void> {
    const {master, bridge} = req;

    const lspServer = new LSPServer(req);
    master.connectedLSPServers.add(lspServer);

    bridge.endEvent.subscribe(() => {
      master.connectedLSPServers.delete(lspServer);
    });

    bridge.lspFromClientBuffer.subscribe((chunk) => {
      lspServer.append(chunk);
    });

    await bridge.endEvent.wait();
  },
});
