/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('disallow unnecessary boolean casts', async (t) => {
  await testLint(t, `
    if (Boolean(foo)) {
      return foo;
    }
    `, {category: 'lint/noExtraBooleanCast'});

  await testLint(t, `
    while (!!foo) {
      return foo;
    }
    `, {category: 'lint/noExtraBooleanCast'});

  await testLint(t, `
    let x = 1;

    do {
        1+1;
    } while (Boolean(x));
    `, {category: 'lint/noExtraBooleanCast'});

  await testLint(t, `
    for (; !!foo; ) {
      return 1+1;
    }
    `, {category: 'lint/noExtraBooleanCast'});
});
