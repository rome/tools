/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLint} from '../../api/lint.test';

test('no debugger', async (t) => {
  await testLint(t, `const test = { debugger: 1 };
  test.debugger;
  console.log(test); // To not trigger the unused var rule.
  `, {category: 'lint/noDebugger'});

  await testLint(t, 'debugger;', {category: 'lint/noDebugger'});
});
