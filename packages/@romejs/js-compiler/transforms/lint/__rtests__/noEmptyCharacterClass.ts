/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {
  testLint,
  LINT_ENABLED_FORMAT_DISABLED_CONFIG,
} from '../../../__rtests__/lint';

test('no empty character class in regular expression', async t => {
  const validTestCases = [
    'let foo = /^abc[a-zA-Z]/;foo;',
    'let regExp = new RegExp("^abc[]");regExp;',
    'let foo = /^abc/;foo;',
    'let foo = /[\\[]/;foo;',
    'let foo = /[\\]]/;foo;',
    'let foo = /[a-zA-Z\\[]/;foo;',
    'let foo = /[[]/;foo;',
    'let foo = /[\\[a-z[]]/;foo;',
    'let foo = /[\\-\\[\\]\\/\\{\\}\\(\\)\\*\\+\\?\\.\\\\^\\$\\|]/g;foo;',
    'let foo = /[\\]]/uy;foo;',
    'let foo = /[\\]]/s;foo;',
    'let foo = /\\[]/;foo;',
  ];

  const invalidTestCases = [
    'let foo = /^abc[]/;foo;',
    'let foo = /foo[]bar/;foo;',
    'let foo = "";if (foo.match(/^abc[]/)) { foo; }',
    'let foo = /[]]/;foo;',
    'let foo = /\\[[]/;foo;',
    'let foo = /\\[\\[\\]a-z[]/;foo;',
  ];

  for (const validTestCase of validTestCases) {
    const {diagnostics} = await testLint(
      validTestCase,
      LINT_ENABLED_FORMAT_DISABLED_CONFIG,
    );
    t.is(diagnostics.length, 0);
  }

  for (const invalidTestCase of invalidTestCases) {
    let res = await testLint(
      invalidTestCase,
      LINT_ENABLED_FORMAT_DISABLED_CONFIG,
    );
    t.snapshot(res);
  }
});
