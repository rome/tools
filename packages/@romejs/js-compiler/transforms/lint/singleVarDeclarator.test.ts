/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('enforce single var declarator', async (t) => {
  // Autofix
  await testLint(t, `let foo, bar;`, {
    category: 'lint/singleVarDeclarator',
    format: true,
  });

  // Ignores loop heads
  await testLint(t, `for (let i = 0, x = 1; i < arr.length; i++) {}`, {
    category: 'lint/singleVarDeclarator',
  });
});
