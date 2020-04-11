/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('no shorthand array type', async (t) => {
  // TypeScript
  const ts = await testLint(`
    let valid: Array<foo>;
    let invalid = bar[];
  `, true, 'module', ['ts']);
  t.snapshot(ts);

  // Flow
  const flow = await testLint(`
    let valid: Array<foo>;
    let invalid = bar[];
  `, true, 'module', ['flow']);
  t.snapshot(flow);
});
