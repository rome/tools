/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('disallow unnecessary boolean casts', async t => {
  const regexExtraElseTest = await testLint(
    `
    let reg = /|Hello|World|/;
    reg.test('Hello');
    `,
  );

  t.truthy(
    regexExtraElseTest.diagnostics.find(
      d => d.message === `Extra else in regular expressions is not allowed.`,
    ),
  );
});
