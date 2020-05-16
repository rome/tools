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
	'no cond assign',
	async (t) => {
		await testLint(
			t,
			dedent`
        for (let i = 1; i = 10; i++) {
          console.log('foo');
        }
      `,
			{category: 'lint/noCondAssign'},
		);

		await testLint(
			t,
			dedent`
        if (foo = 'bar') {
          console.log('foo');
        }
      `,
			{category: 'lint/noCondAssign'},
		);

		await testLint(
			t,
			dedent`
        while (foo = 'bar') {
          console.log('foo');
        }
      `,
			{category: 'lint/noCondAssign'},
		);

		await testLint(
			t,
			dedent`
        do {
          console.log('foo');
        } while (foo = 'bar')
      `,
			{category: 'lint/noCondAssign'},
		);
	},
);
