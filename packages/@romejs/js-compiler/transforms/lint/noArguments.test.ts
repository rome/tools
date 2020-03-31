/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('no arguments', async (t) => {
  t.snapshot(await testLint(`
    function f() {
      console.log(arguments);
    }
    f();
  `));

  t.snapshot(await testLint(`
    (function () {
      console.log(arguments);
    })();
  `));

  t.snapshot(await testLint(`
    class C {
      fn() {
        console.log(arguments);
      }
    }
    (new C()).fn();
  `));

  t.snapshot(await testLint(`
    const o = {
      fn() {
        console.log(arguments);
      }
    };
    o.fn();
  `));
});
