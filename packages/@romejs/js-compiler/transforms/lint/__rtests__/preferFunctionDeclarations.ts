/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {
  testLint,
  LINT_AND_FORMAT_ENABLED_CONFIG,
} from '../../../__rtests__/lint';

test('prefer function declarations', async t => {
  // Should complain on these
  t.snapshot(
    await testLint(
      'const foo = function () {};',
      LINT_AND_FORMAT_ENABLED_CONFIG,
    ),
  );
  t.snapshot(
    await testLint('const foo = () => {};', LINT_AND_FORMAT_ENABLED_CONFIG),
  );

  // Should allow arrow functions when they have this
  t.snapshot(
    await testLint(
      'const foo = () => {this;};',
      LINT_AND_FORMAT_ENABLED_CONFIG,
    ),
  );

  // But only if it refers to the actual arrow function
  t.snapshot(
    await testLint(
      'const foo = () => {function bar() {this;}};',
      LINT_AND_FORMAT_ENABLED_CONFIG,
    ),
  );

  // Should ignore functions with return types since you can't express that with a declaration
  t.snapshot(
    await testLint(
      'const foo: any = function () {};',
      LINT_AND_FORMAT_ENABLED_CONFIG,
    ),
  );
});
