/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLint} from '../../api/lint.test';

test('prefer function declarations', async (t) => {
  // Should complain on these
  await testLint(t, 'const foo = function () {};', {
    category: 'lint/preferFunctionDeclarations',
  });
  await testLint(t, 'const foo = () => {};', {
    category: 'lint/preferFunctionDeclarations',
  });

  // Should allow arrow functions when they have this
  await testLint(t, 'const foo = () => {this;};', {
    category: 'lint/preferFunctionDeclarations',
  });

  // But only if it refers to the actual arrow function
  await testLint(t, 'const foo = () => {function bar() {this;}};', {
    category: 'lint/preferFunctionDeclarations',
  });

  // Should ignore functions with return types since you can't express that with a declaration
  await testLint(t, 'const foo: string = function () {};', {
    category: 'lint/preferFunctionDeclarations',
    syntax: ['ts'],
  });
});
