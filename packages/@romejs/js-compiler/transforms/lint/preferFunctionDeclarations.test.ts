/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('prefer function declarations', async (t) => {
  // Should complain on these
  t.snapshot(await testLint('const foo = function () {};', true));
  t.snapshot(await testLint('const foo = () => {};', true));

  // Should allow arrow functions when they have this
  t.snapshot(await testLint('const foo = () => {this;};', true));

  // But only if it refers to the actual arrow function
  t.snapshot(await testLint('const foo = () => {function bar() {this;}};', true));

  // Should ignore functions with return types since you can't express that with a declaration
  t.snapshot(await testLint(
    'const foo: string = function () {};',
    true,
    'module',
    [
      'ts',
      'flow',
    ],
  ));
});
