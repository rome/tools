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

test('no delete vars', async t => {
  const res = await testLint(
    `
    const foo = "test";
    delete foo;
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
    'script',
  );

  t.looksLike(res.diagnostics, [
    {
      category: 'lint/noDeleteVars',
      message: 'Variables should not be deleted.',
      mtime: undefined,
      filename: 'unknown',
      start: {index: 29, line: 3, column: 4},
      end: {index: 39, line: 3, column: 14},
      language: 'js',
      sourceType: 'script',
      origins: [{category: 'lint'}],
    },
  ]);
});
