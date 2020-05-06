/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLintMultiple} from '../testHelpers';

test(
  'no children props',
  async (t) => {
    await testLintMultiple(
      t,
      [
        // INVALID
        `<MyComponent children={'foo'}></MyComponent>`,
        `React.createElement('div', {children: 'foo'})`,
        // VALID
        `<MyComponent><AnotherComponent /></MyComponent  >`,
        `React.createElement('div', {}, 'children')`,
        `React.createElement('div', child1, 'child2')`,
      ],
      {category: 'lint/noChildrenProp'},
    );
  },
);
