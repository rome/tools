/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLint} from '../../api/lint.test';

test('no ex-assign', async (t) => {
  const res = await testLint(t, `
    try {
      console.log('hello');
    } catch (e) {
      e = 10;
    }
  `, {category: 'lint/noExAssign'});

  t.snapshot(res);
});
