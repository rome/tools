/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('no cond assign', async (t) => {
  await testLint(t, `for (let i = 1; i = 10; i++) {
    console.log('foo')
  }`, {category: 'lint/noCondAssign'});

  await testLint(t, `if(foo = 'bar') {
    console.log('foo')
  }`, {category: 'lint/noCondAssign'});

  await testLint(t, `while (foo = 'bar' {
    console.log('foo')
  }`, {category: 'lint/noCondAssign'});

  await testLint(t, `do {
    console.log('foo)
  } while (foo = 'bar')`, {category: 'lint/noCondAssign'});
});
