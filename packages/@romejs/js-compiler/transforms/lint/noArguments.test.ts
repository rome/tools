/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLint} from '../../api/lint.test';

test('no arguments', async (t) => {
  await testLint(t, `
    function f() {
      console.log(arguments);
    }
    f();
  `, {category: 'lint/noArguments'});

  await testLint(t, `
    (function () {
      console.log(arguments);
    })();
  `, {category: 'lint/noArguments'});

  await testLint(t, `
    class C {
      fn() {
        console.log(arguments);
      }
    }
    (new C()).fn();
  `, {category: 'lint/noArguments'});

  await testLint(t, `
    const o = {
      fn() {
        console.log(arguments);
      }
    };
    o.fn();
  `, {category: 'lint/noArguments'});
});
