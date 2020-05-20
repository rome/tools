/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "./rules/testHelpers";

test(
	"format disabled in project config should not regenerate the file",
	async (t) => {
		// Intentionally weird formatting
		await testLint(t, 'foobar ( "yes" );', {category: undefined});
	},
);

test(
	"format enabled in project config should result in regenerated file",
	async (t) => {
		await testLint(
			t,
			'foobar ( "yes" );',
			{
				category: undefined,
			},
		);
	},
);
