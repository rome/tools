/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLint} from '../testHelpers';

test(
  'restricted globals',
  async (t) => {
    await testLint(
      t,
      'console.log(event);',
      {
        category: 'lint/restrictedGlobals',
      },
    );

    await testLint(t, `
    // valid use of event into the function scope.
    function foo(event) {
      console.info(event);
    }

    // invalid, event is used as a global.
    foo(event)
    `, {category: 'lint/restrictedGlobals'});
  },
);
