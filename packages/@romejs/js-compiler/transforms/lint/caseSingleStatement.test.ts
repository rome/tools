/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLintMultiple} from '../../api/lint.test';

test('case single statement', async (t) => {
  await testLintMultiple(
    t,
    [
      // VALID
      "switch (foo) {case true: case false: return 'yes';}", // Single statement
      'switch (foo) {case true: {}}', // Single block
      'switch (foo) {case true:}', // Nothing

      // INVALID
      "switch (foo) {case true: case false: let foo = ''; foo;}", // Multiple statements
    ],
    {category: 'lint/caseSingleStatement'},
  );
});
