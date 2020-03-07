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

test('no debugger', async t => {
  const goodRes = await testLint(
    `const test = { debugger: 1 };
    test.debugger;
    console.log(test); // To not trigger the unused var rule.
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.is(goodRes.diagnostics.length, 0);

  const badRes = await testLint(
    'debugger;',
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.snapshot(badRes.diagnostics);
});
