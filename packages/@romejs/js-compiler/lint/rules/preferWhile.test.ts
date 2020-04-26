/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLintMultiple} from '../testHelpers';

test(
  'prefer while',
  async (t) => {
    await testLintMultiple(
      t,
      [`for (; x.running;) x.step();`, `for (;;) doSomething();`],
      {
        category: 'lint/preferWhile',
      },
    );
  },
);
