/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('disallow unmatched opening square bracket in regular expressions', async t => {
  let validTestCases = [
    `let test = /\\[]/;test;`,
    `let test = /\\[\\[]/;test;`,
    `let test = /\\[test]/;test;`,
  ];

  let invalidTestCases = [
    `let test = /\\[/;test;`,
    `let test = /\\[]\\[/;test;`,
    `let test = /\\[test]\\[/;test;`,
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
