/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import setProcessTitle from './utils/setProcessTitle';
import {TestWorker} from '@romejs/core';

export default function testWorker() {
  setProcessTitle('test-worker');
  const worker = new TestWorker();
  worker.init();
}
