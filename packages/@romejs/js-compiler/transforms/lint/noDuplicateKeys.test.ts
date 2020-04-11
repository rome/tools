/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('no duplicate keys', async (t) => {
  await testLint(t, `
  const foo = {
    test: true,
    test2: true,
    test: false,
  }

  // mark const as used
  console.log(foo);
  `, {category: 'lint/noDuplicateKeys'});
});
