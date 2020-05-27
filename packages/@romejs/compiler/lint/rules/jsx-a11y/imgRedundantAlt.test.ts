/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";

import {testLintMultiple} from "../testHelpers";

test(
	"jsx-a11y img redundant alt",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				'<img src="src" alt="photo content" />',
				'<img src="src" alt="picture content" />',
				'<img src="src" alt="image content" />',
				'<img src="src" alt="Photo content" />',
				'<img src="src" alt="Picture content" />',
				'<img src="src" alt="Image content" />',
				// VALID
				'<img src="src" alt="alt" />',
				'<img src="src" alt={photo} />',
			],
			{category: "lint/jsx-a11y/imgRedundantAlt"},
		);
	},
);
