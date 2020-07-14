/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y img redundant alt",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					'<img src="src" alt="photo content" />',
					'<img src="src" alt="picture content" />',
					'<img src="src" alt="image content" />',
					'<img src="src" alt="Photo content" />',
					'<img src="src" alt="Picture content" />',
					'<img src="src" alt="Image content" />',
				],
				valid: ['<img src="src" alt="alt" />', '<img src="src" alt={photo} />'],
				filename: "file.tsx",
				category: "lint/jsx-a11y/imgRedundantAlt",
			},
		);
	},
);
