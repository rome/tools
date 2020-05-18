/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"no comma operator",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"(0, 1, 2)",
			],
			{category: "lint/javascript/noCommaOperator"},
		);
	},
);
