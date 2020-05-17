/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"import default basename",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"import foo from './bar';",
				// VALID
				"import foo from './foo';",
			],
			{category: "lint/importDefaultBasename"},
		);
	},
);
