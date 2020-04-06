/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('duplicate import source', async (t) => {
  const res = await testLint(`
    import foo from './testdummy.ts';
    import {bar} from './testdummy.ts';
    import type {fooType} from './testdummy.ts';
  
    const typedFoo: fooType = {
      type: 'foo'
    } 
    console.log(typedFoo, foo, bar);
    `);
  t.snapshot(res);
});
