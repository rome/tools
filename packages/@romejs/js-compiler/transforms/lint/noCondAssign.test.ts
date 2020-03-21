/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('no cond assign', async (t) => {
  const forLoop = await testLint(`for (let i = 1; i = 10; i++) {
      console.log('foo')
    }`);

  t.snapshot(forLoop);

  const ifCondition = await testLint(`if(foo = 'bar') {
      console.log('foo')
    }`);

  t.snapshot(ifCondition);

  const whileLoop = await testLint(`while (foo = 'bar' {
      console.log('foo')
    }`);

  t.snapshot(whileLoop);

  const DoWhileLoop = await testLint(`do {
      console.log('foo)
    } while (foo = 'bar')`);

  t.snapshot(DoWhileLoop);
});
