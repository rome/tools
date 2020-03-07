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

test('unsafe negation', async t => {
  const res = await testLint(
    `!1 in [1,2]`,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.snapshot(res);
});
