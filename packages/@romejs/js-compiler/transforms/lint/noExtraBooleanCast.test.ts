/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('disallow unnecessary boolean casts', async (t) => {
  const ifTest = await testLint(`
    if (Boolean(foo)) {
      return foo;
    }
    `);

  t.truthy(ifTest.diagnostics.find((d) => d.description.message.value ===
    `Redundant double negation.`));

  const whileTest = await testLint(`
    while (!!foo) {
      return foo;
    }
    `);

  t.truthy(whileTest.diagnostics.find((d) => d.description.message.value ===
    `Redundant double negation.`));

  const doWhileTest = await testLint(`
    let x = 1;

    do {
        1+1;
    } while (Boolean(x));
    `);

  t.truthy(doWhileTest.diagnostics.find((d) => d.description.message.value ===
    `Redundant double negation.`));

  const forTest = await testLint(`
    for (; !!foo; ) {
      return 1+1;
    }
    `);

  t.truthy(forTest.diagnostics.find((d) => d.description.message.value ===
    `Redundant double negation.`));
});
