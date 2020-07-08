/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"enforce single var declarator",
	async (t) => {
		await testLint(
			t,
			{
				invalid: ["let foo, bar;"],
				valid: [
					// Ignores loop heads
					"for (let i = 0, x = 1; i < arr.length; i++) {}",
				],
			},
			{
				category: "lint/js/singleVarDeclarator",
			},
		);
	},
);
