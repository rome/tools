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

test('disallow multiple spaces in regular expression literals', async t => {
  const res1 = await testLint(
    `new RegExp("foo  bar")`,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );
  t.looksLike(res1.diagnostics, [
    {
      category: 'lint/disallowMultipleSpacesInRegularExpressionLiterals',
      message: 'Disallow multiple spaces in regular expression literals',
      mtime: undefined,
      filename: 'unknown',
      start: {index: 0, line: 1, column: 0},
      end: {index: 22, line: 1, column: 22},
      language: 'js',
      sourceType: 'module',
      origins: [{category: 'lint'}],
    },
  ]);

  const res2 = await testLint(
    `new RegExp("foo {2}bar")`,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.looksLike(res2.diagnostics, []);

  const res3 = await testLint(
    `/foo  bar/`,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );
  t.looksLike(res3.diagnostics, [
    {
      category: 'lint/disallowMultipleSpacesInRegularExpressionLiterals',
      message: 'Disallow multiple spaces in regular expression literals',
      mtime: undefined,
      filename: 'unknown',
      start: {index: 0, line: 1, column: 0},
      end: {index: 10, line: 1, column: 10},
      language: 'js',
      sourceType: 'module',
      origins: [{category: 'lint'}],
    },
  ]);

  const res4 = await testLint(
    `/foo {2}bar/`,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.looksLike(res4.diagnostics, []);
});
