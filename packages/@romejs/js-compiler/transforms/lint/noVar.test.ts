/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('disallow var', async (t) => {
  const res = await testLint('var foobar;\nfoobar');
  t.snapshot(res);
});
