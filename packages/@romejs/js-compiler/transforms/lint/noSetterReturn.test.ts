/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('no setter return', async (t) => {
  const badClass = await testLint(`
    class p {
      set name(value) {
        if (!value) {
          return 'wrong';
        }
      }
    }
    console.log(new p());
  `);

  t.snapshot(badClass);

  const badStaticClass = await testLint(`
    class p {
      static set name(value) {
        if (!value) {
          return 'wrong';
        }
      }
    }
    console.log(p);
  `);

  t.snapshot(badStaticClass);

  const badObject = await testLint(`
    let p = {
      set name(value) {
        if (!value) {
          return 'wrong';
        }
      }
    };
    console.log(p);
  `);

  t.snapshot(badObject);

  const blankReturn = await testLint(`
    class p {
      set name(value) {
        if (!value) {
          return;
        }
      }
    };
    console.log(p);
  `);

  t.snapshot(blankReturn);
});
