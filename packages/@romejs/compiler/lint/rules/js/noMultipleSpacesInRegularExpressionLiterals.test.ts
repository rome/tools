/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"disallow multiple spaces in regular expression literals",
	async (t) => {
		await testLint(
			t,
			{invalid: [`/foo  bar/`], valid: [`/foo {2}bar/`]},
			{
				category: "lint/js/noMultipleSpacesInRegularExpressionLiterals",
			},
		);
	},
);
