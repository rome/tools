/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('enforce single var declarator', async (t) => {
  t.snapshot(await testLint(`let foo, bar;`, true));

  // Ignores loop heads
  t.snapshot(await testLint(`for (let i = 0, x = 1; i < arr.length; i++) {}`));
});
