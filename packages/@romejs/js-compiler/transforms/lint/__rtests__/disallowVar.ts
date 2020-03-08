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

test('disallow var', async t => {
  const res = await testLint(
    'var foobar;\nfoobar',
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );
  t.snapshot(res);

  // Redundant because of the snapshot above, but this is what we actually care about
  t.looksLike(res.diagnostics, [
    {
      category: 'lint/disallowVar',
      filename: 'unknown',
      language: 'js',
      message:
        'Variable declarations using `var` are disallowed, use `let` or `const` instead.',
      mtime: undefined,
      sourceType: 'module',
      origins: [{category: 'lint'}],
      end: {
        column: 11,
        index: 11,
        line: 1,
      },
      start: {
        column: 0,
        index: 0,
        line: 1,
      },
    },
  ]);
});
