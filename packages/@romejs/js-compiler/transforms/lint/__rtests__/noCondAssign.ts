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

test('no cond assign', async t => {
  const forLoop = await testLint(
    `for (let i = 1; i = 10; i++) {
      console.log('foo')
    }`,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.snapshot(forLoop);

  const ifCondition = await testLint(
    `if(foo = 'bar') {
      console.log('foo')
    }`,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.snapshot(ifCondition);

  const whileLoop = await testLint(
    `while (foo = 'bar' {
      console.log('foo')
    }`,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.snapshot(whileLoop);

  const DoWhileLoop = await testLint(
    `do {
      console.log('foo)
    } while (foo = 'bar')`,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.snapshot(DoWhileLoop);
});
