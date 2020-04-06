/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test(
  'prefer template',
  async (t) => {
    const template1 = await testLint(
      `const foo = 'bar'; console.log(foo + 'baz')`,
    );

    t.snapshot(template1);

    const template2 = await testLint(`console.log((1 * 2) + 'baz')`);

    t.snapshot(template2);
  },
);
