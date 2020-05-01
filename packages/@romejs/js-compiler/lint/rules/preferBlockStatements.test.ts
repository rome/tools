/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {dedent} from '@romejs/string-utils';
import {testLintMultiple} from '../testHelpers';

test(
  'prefer block statements',
  async (t) => {
    await testLintMultiple(t, [`if (x) x;`, dedent`
          if (x) {
            x;
          } else y;
        `, dedent`
          if (x) {
            x
          } else if (y) y;
        `], {category: 'lint/preferBlockStatements'});

    await testLintMultiple(
      t,
      [
        `for (;;);`,
        `for (p in obj);`,
        `for (x of xs);`,
        `do; while (x);`,
        `while (x);`,
        `with (x);`,
      ],
      {category: 'lint/preferBlockStatements'},
    );
  },
);
