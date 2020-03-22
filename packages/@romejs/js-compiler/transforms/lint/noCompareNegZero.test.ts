/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('disallows comparing negative zero', async (t) => {
  const sourceTextA = '(1 >= -0)';

  const sourceTextB = '(1 >= 0)';

  const res1 = await testLint(sourceTextA);
  t.snapshot(res1);

  const res2 = await testLint(sourceTextB);
  t.snapshot(res2);
});
