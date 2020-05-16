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
  'no shorthand array type',
  async (t) => {
    // TypeScript
    await testLint(
      t,
      dedent`
        let valid: Array<foo>;
        let invalid: bar[];
      `,
      {category: 'lint/noShorthandArrayType', syntax: ['ts']},
    );
  },
);
