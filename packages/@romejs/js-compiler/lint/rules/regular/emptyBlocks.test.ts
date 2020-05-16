/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLintMultiple} from '../testHelpers';
import {dedent} from '@romejs/string-utils';

test(
	'empty blocks',
	async (t) => {
		await testLintMultiple(
			t,
			[
				// VALID
				dedent`
          if (foo) foo;
        `,
				dedent`
          if (foo) {
            foo;
          }
        `,
				dedent`
          if (foo) {
            // empty
          }
        `,
				// INVALID
				dedent`
          if (foo) {}
        `,
				dedent`
          if (foo) {
            // foo;
          } else {}
        `,
			],
			{category: 'lint/emptyBlocks'},
		);
	},
);
