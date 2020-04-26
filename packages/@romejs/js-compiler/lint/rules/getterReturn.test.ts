/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLint} from '../testHelpers';

test(
  'getter return',
  async (t) => {
    await testLint(t, `
  class p {
    get name() {
      console.log('hello')
    };
  }
  console.log(new p())
  `, {category: 'lint/getterReturn'});

    await testLint(t, `
  let p;
  p = {
    get name() {
      console.log('hello')
    }
  };
  console.log(p)
  `, {category: 'lint/getterReturn'});

    await testLint(t, `
  let p = {};
  Object.defineProperty(p, {
    get: function (){
        console.log('hello')
    }
  });
  console.log(p)
  `, {category: 'lint/getterReturn'});
  },
);
