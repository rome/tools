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

test('disallow unsafe usage of break, continue, throw and return', async t => {
  const returnTest = await testLint(
    `
    function greet1() {
      try {
        throw new Error("Try")
      } catch(err) {
          throw err;
      } finally {
          return 1;
      }
    }

    greet1();
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.truthy(
    returnTest.diagnostics.find(
      d => d.message === `Unsafe usage of ReturnStatement.`,
    ),
  );

  const breakTest = await testLint(
    `

    function greet2() {
      try {
        throw new Error("Try")
      } catch(err) {
          throw err;
      } finally {
          break;
      }
    }

    greet2();
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.truthy(
    breakTest.diagnostics.find(
      d => d.message === `Unsafe usage of BreakStatement.`,
    ),
  );

  const continueTest = await testLint(
    `
    function greet3() {
      try {
        throw new Error("Try")
      } catch(err) {
          throw err;
      } finally {
          continue;
      }
    }

    greet3();
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.truthy(
    continueTest.diagnostics.find(
      d => d.message === `Unsafe usage of ContinueStatement.`,
    ),
  );

  const throwTest = await testLint(
    `
    function greet4() {
      try {
        throw new Error("Try")
      } catch(err) {
          throw err;
      } finally {
          throw new Error("Finally");
      }
    }

    greet4();
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.truthy(
    throwTest.diagnostics.find(
      d => d.message === `Unsafe usage of ThrowStatement.`,
    ),
  );
});
