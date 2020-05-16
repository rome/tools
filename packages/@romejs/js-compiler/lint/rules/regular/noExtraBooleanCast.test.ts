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
	'disallow unnecessary boolean casts',
	async (t) => {
		await testLint(
			t,
			dedent`
        if (Boolean(foo)) {
          return foo;
        }
      `,
			{category: 'lint/noExtraBooleanCast'},
		);

		await testLint(
			t,
			dedent`
        while (!!foo) {
          return foo;
        }
      `,
			{category: 'lint/noExtraBooleanCast'},
		);

		await testLint(
			t,
			dedent`
        let x = 1;
        do {
          1 + 1;
        } while (Boolean(x));
      `,
			{category: 'lint/noExtraBooleanCast'},
		);

		await testLint(
			t,
			dedent`
        for (; !!foo; ) {
          return 1 + 1;
        }
      `,
			{category: 'lint/noExtraBooleanCast'},
		);
	},
);
