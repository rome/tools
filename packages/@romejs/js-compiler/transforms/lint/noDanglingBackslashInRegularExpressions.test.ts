/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('Dangling backslash in regex', async (t) => {
  const validTestCases = [
    String.raw`let foo = /\\/;foo;`,
    String.raw`let foo = /2\/1=2/;foo;`,
    String.raw`let foo = /\/\//;foo;`,
    String.raw`let foo = /\/\//
    foo;`,
  ];

  const invalidTestCases = [
    String.raw`let foo = /\/;foo;`,
    String.raw`let foo = /\\\/;foo;`,
    String.raw`let foo = /\/\/;\/;foo;`,
    String.raw`let foo = /\/\/;\/
    foo;`,
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
