/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import setProcessTitle from './utils/setProcessTitle';
import {createBridgeFromParentProcess} from '@romejs/events';
import {Worker, WorkerBridge} from '@romejs/core';

export default async function worker() {
  setProcessTitle('worker');
  const bridge = createBridgeFromParentProcess(
    WorkerBridge,
    {
      type: 'server',
    },
  );
  const worker = new Worker({
    bridge,
    globalErrorHandlers: true,
  });
  await worker.init();
  bridge.handshake();
}
