/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';
import {PartialDiagnostic} from '@romejs/diagnostics/types';

test('no POSIX in regular expression', async t => {
  function checkCategory(diagnostic: PartialDiagnostic): Boolean {
    return diagnostic.category === 'lint/noPosixInRegularExpression';
  }

  const testCases = [
    'const pattern = /[[:alpha:]]/',
    'const pattern = /[[.ch.]]/',
  ];

  for (const testCase of testCases) {
    const {diagnostics} = await testLint(testCase);
    t.truthy(diagnostics.find(checkCategory));
  }
});
