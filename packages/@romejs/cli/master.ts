/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {CLI_SOCKET_PATH, Master, MasterBridge, SOCKET_PATH} from '@romejs/core';
import {createBridgeFromSocket} from '@romejs/events';
import setProcessTitle from './utils/setProcessTitle';
import net = require('net');

import {exists, unlink} from '@romejs/fs';

export default async function master() {
  setProcessTitle('master');

  const master = new Master({
    dedicated: true,
    globalErrorHandlers: true,
  });

  await master.init();

  const socketServer = net.createServer(function(socket) {
    const client = createBridgeFromSocket(
      MasterBridge,
      socket,
      {
        type: 'client',
      },
    );
    master.attachToBridge(client);
  });

  if (await exists(SOCKET_PATH)) {
    await unlink(SOCKET_PATH);
  }

  socketServer.listen(
    SOCKET_PATH.join(),
    () => {
      const socket = net.createConnection(
        CLI_SOCKET_PATH.join(),
        () => {
          socket.end();
        },
      );

      socket.on(
        'error',
        (err) => {
          // Socket error occured, cli could have died before it caught us
          err;
          console.log(err);
          process.exit();
        },
      );
    },
  );
}
