/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('regex quantifier range', async t => {
  const validTestCases = [
    'let foo = /a{1,2}/;foo;',
    'let foo = /[0-9]{1,10}/;foo;',
    'let foo = /[A-Z]{2,}/;foo;',
  ];

  const invalidTestCases = [
    'let foo = /d{2,1}/;foo;',
    'let foo = /[0-9]{10,1}/;foo;',
  ];

  for (const validTestCase of validTestCases) {
    const {diagnostics} = await testLint(validTestCase);
    t.is(diagnostics.length, 0);
  }

  for (const invalidTestCase of invalidTestCases) {
    let res = await testLint(invalidTestCase);
    t.snapshot(res);
  }
});
