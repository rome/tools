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

test('no duplicated args allowed', async t => {
  const duplicatedArgs = await testLint(
    `
  function hello(a, a) {
    console.log("Hello);
  }
  hello();
  `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.truthy(
    duplicatedArgs.diagnostics.find(d => d.category === 'lint/noDupeArgs'),
  );
});
