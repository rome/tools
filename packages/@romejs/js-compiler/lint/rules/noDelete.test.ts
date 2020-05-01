/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLintMultiple} from '../testHelpers';

test(
  'no delete',
  async (t) => {
    await testLintMultiple(
      t,
      [`
				const arr = [['a','b','c'], [1, 2, 3]];
				delete arr[0][2];
			`, `
				const obj = { a: { b: { c: 123 } } };
				delete obj.a.b.c;
			`],
      {
        category: 'lint/noDelete',
      },
    );
  },
);
