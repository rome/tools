/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {
  testLint,
  LINT_ENABLED_FORMAT_DISABLED_CONFIG,
} from '../../../__rtests__/lint';

test('disallow unnecessary boolean casts', async t => {
  const ifTest = await testLint(
    `
    if (Boolean(foo)) {
      return foo;
    }
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.truthy(
    ifTest.diagnostics.find(d => d.message === `Redundant double negation.`),
  );

  const whileTest = await testLint(
    `
    while (!!foo) {
      return foo;
    }
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.truthy(
    whileTest.diagnostics.find(d => d.message === `Redundant double negation.`),
  );

  const doWhileTest = await testLint(
    `
    let x = 1;

    do {
        1+1;
    } while (Boolean(x));
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.truthy(
    doWhileTest.diagnostics.find(
      d => d.message === `Redundant double negation.`,
    ),
  );

  const forTest = await testLint(
    `
    for (; !!foo; ) {
      return 1+1;
    }
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.truthy(
    forTest.diagnostics.find(d => d.message === `Redundant double negation.`),
  );
});
