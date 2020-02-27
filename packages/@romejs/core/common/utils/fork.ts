/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {VERSION, CHILD_ARGS, BIN} from '@romejs/core';
import child = require('child_process');

export default function fork(
  processType: string,
  opts?: child.ForkOptions,
): child.ChildProcess {
  return child.fork(BIN.join(), CHILD_ARGS, {
    stdio: 'inherit',
    ...opts,
    env: {
      ...process.env,
      ROME_PROCESS_VERSION: VERSION,
      ROME_PROCESS_TYPE: processType,
    },
  });
}
