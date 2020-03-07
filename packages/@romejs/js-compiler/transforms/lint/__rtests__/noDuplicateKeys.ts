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

test('no duplicate keys', async t => {
  const res = await testLint(
    `
    const foo = {
      test: true,
      test2: true,
      test: false,
    }

    // mark const as used
    console.log(foo);
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.looksLike(res.diagnostics, [
    {
      category: 'lint/noDuplicateKeys',
      filename: 'unknown',
      language: 'js',
      message: 'Duplicate key <emphasis>test</emphasis>',
      mtime: undefined,
      sourceType: 'module',
      origins: [{category: 'lint'}],
      end: {
        column: 17,
        index: 73,
        line: 5,
      },
      start: {
        column: 6,
        index: 62,
        line: 5,
      },
    },
  ]);
});
