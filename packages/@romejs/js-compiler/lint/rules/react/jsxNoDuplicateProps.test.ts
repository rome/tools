/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLintMultiple} from '../../testHelpers';

test(
  'jsx no duplicate props',
  async (t) => {
    await testLintMultiple(
      t,
      [`const t = <div a="a" a="b" />`],
      {
        category: 'lint/react/jsxNoDuplicateProps',
      },
    );
  },
);
