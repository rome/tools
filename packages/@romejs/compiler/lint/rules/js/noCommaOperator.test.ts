/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"no comma operator",
	async (t) => {
		await testLint(
			t,
			{
				invalid: ["(0, 1, 2)", "test(), rome()"],
				valid: ["foo(0, 1, 2)", "[1, 2,]", "[1,,,3]", "let a, b, c;"],
			},
			{category: "lint/js/noCommaOperator"},
		);
	},
);
