/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Number0, number0} from '@romejs/ob1';

export function createIndexTracker(): IndexTracker {
  return {index: number0};
}

export type IndexTracker = {index: Number0};
