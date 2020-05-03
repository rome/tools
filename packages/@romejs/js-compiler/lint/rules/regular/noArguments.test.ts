/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLint} from '../testHelpers';
import {dedent} from '@romejs/string-utils';

test(
  'no arguments',
  async (t) => {
    await testLint(
      t,
      dedent`
        function f() {
          console.log(arguments);
        }
      `,
      {category: 'lint/noArguments'},
    );

    await testLint(
      t,
      dedent`
        (function () {
          console.log(arguments);
        })();
      `,
      {category: 'lint/noArguments'},
    );

    await testLint(
      t,
      dedent`
        class C {
          fn() {
            console.log(arguments);
          }
        }
      `,
      {category: 'lint/noArguments'},
    );

    await testLint(
      t,
      dedent`
        const o = {
          fn() {
            console.log(arguments);
          },
        };
      `,
      {category: 'lint/noArguments'},
    );
  },
);
