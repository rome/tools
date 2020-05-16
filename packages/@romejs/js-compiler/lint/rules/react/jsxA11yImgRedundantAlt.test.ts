/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';

import {testLintMultiple} from '../testHelpers';

test(
	'disallow redundant alt descriptions on img tags',
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				'<img src="src" alt="photo" />',
				'<img src="src" alt="picture" />',
				'<img src="src" alt="image" />',
				'<img src="src" alt="Photo" />',
				'<img src="src" alt="Picture" />',
				'<img src="src" alt="Image" />',
				// VALID
				'<img src="src" alt="alt" />',
				'<img src="src" alt={photo} />',
			],
			{category: 'lint/jsxA11yImgRedundantAlt'},
		);
	},
);
