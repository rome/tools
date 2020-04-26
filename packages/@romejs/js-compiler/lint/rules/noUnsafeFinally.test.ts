/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLint} from '../testHelpers';

test(
  'disallow unsafe usage of break, continue, throw and return',
  async (t) => {
    await testLint(t, `
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
    `, {category: 'lint/noUnsafeFinally'});

    await testLint(t, `
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
    `, {category: 'lint/noUnsafeFinally'});

    await testLint(t, `
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
    `, {category: 'lint/noUnsafeFinally'});

    await testLint(t, `
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
      `, {category: 'lint/noUnsafeFinally'});
  },
);
