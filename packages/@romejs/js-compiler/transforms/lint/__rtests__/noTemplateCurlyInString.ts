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

test('no template curly in string', async t => {
  const res = await testLint(
    `
    const user = "Faustina";
    const helloUser = "Hello, \${user}!";

    // mark consts as used
    console.log(user, helloUser)
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.looksLike(res.diagnostics, [
    {
      category: 'lint/noTemplateCurlyInString',
      filename: 'unknown',
      language: 'js',
      message: 'Unexpected template string expression.',
      mtime: undefined,
      sourceType: 'module',
      origins: [{category: 'lint'}],
      end: {column: 39, index: 69, line: 3},
      start: {column: 22, index: 52, line: 3},
    },
  ]);
});
