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

test('no async promise executor', async t => {
  const validTestCases = [
    'new Promise(() => {})',
    'new Promise(() => {}, async function unrelated() {})',
    'class Foo {} new Foo(async () => {})',
  ];
  const invalidTestCases = [
    'new Promise(async function foo() {})',
    'new Promise(async () => {})',
    'new Promise(((((async () => {})))))',
  ];
  for (const validTestCase of validTestCases) {
    const {diagnostics} = await testLint(
      validTestCase,
      LINT_ENABLED_FORMAT_DISABLED_CONFIG,
    );
    t.is(diagnostics.length, 0);
  }
  for (const invalidTestCase of invalidTestCases) {
    t.snapshot(
      await testLint(invalidTestCase, LINT_ENABLED_FORMAT_DISABLED_CONFIG),
    );
  }
});
