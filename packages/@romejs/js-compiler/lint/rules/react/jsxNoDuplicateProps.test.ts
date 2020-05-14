/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';

import {testLintMultiple} from '../testHelpers';

test(
  'disallow duplicate props on JSX elements',
  async (t) => {
    await testLintMultiple(
      t,
      [
        // INVALID
        '<div id="foo" id="bar">children</div>',
        '<Invalid id="foo" id="bar">children</Invalid>',
        // VALID
        '<div id="foo">children</div>',
        '<Valid id="foo">children</Valid>',
      ],
      {category: 'lint/jsxNoDuplicateProps'},
    );
  },
);
