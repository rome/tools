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

test('disallows comparing negative zero', async t => {
  const sourceTextA = '(1 >= -0)';

  const sourceTextB = '(1 >= 0)';

  const res1 = await testLint(sourceTextA, LINT_ENABLED_FORMAT_DISABLED_CONFIG);
  t.looksLike(res1.diagnostics, [
    {
      category: 'lint/noCompareNegZero',
      filename: 'unknown',
      language: 'js',
      message: "Do not use the '>=' operator to compare against -0.",
      mtime: undefined,
      sourceType: 'module',
      origins: [{category: 'lint'}],
      end: {
        column: 8,
        index: 8,
        line: 1,
      },
      start: {
        column: 1,
        index: 1,
        line: 1,
      },
    },
  ]);

  const res2 = await testLint(sourceTextB, LINT_ENABLED_FORMAT_DISABLED_CONFIG);
  t.looksLike(res2.diagnostics, []);
});
