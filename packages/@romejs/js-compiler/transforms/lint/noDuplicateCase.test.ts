/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('no duplicated switch cases allowed', async t => {
  const duplicatedSwitchCase = await testLint(
    `
    const expr = 'a';
    switch (expr) {
      case 'a':
        break;
      case 'b':
        break;
      case 'c':
        break;
      case 'd':
        break;
      case 'c':
        break;
      default:
        break;
    }
  `,
  );

  t.truthy(
    duplicatedSwitchCase.diagnostics.find(
      d => d.category === 'lint/noDuplicateCase',
    ),
  );
});
