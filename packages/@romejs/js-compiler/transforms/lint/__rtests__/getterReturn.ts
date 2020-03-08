/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {
  testLint,
  LINT_AND_FORMAT_ENABLED_CONFIG,
} from '../../../__rtests__/lint';

test('getter return', async t => {
  const badClass = await testLint(
    `
    class p {
      get name() {
        console.log('hello')
      };
    }
    console.log(new p())
    `,
    LINT_AND_FORMAT_ENABLED_CONFIG,
  );

  t.snapshot(badClass);

  const badObject = await testLint(
    `
    let p;
    p = {
      get name() {
        console.log('hello')
      }
    };
    console.log(p)
    `,
    LINT_AND_FORMAT_ENABLED_CONFIG,
  );

  t.snapshot(badObject);

  const badDefinedProperty = await testLint(
    `
    let p = {};
    Object.defineProperty(p, {
      get: function (){
          console.log('hello')
      }
    });
    console.log(p)
    `,
    LINT_AND_FORMAT_ENABLED_CONFIG,
  );

  t.snapshot(badDefinedProperty);
});
