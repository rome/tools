/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"import default basename",
	async (t) => {
		await testLint(
			t,
			{
				invalid: ["import foo from './bar';"],
				valid: ["import foo from './foo';"],
			},
			{category: "lint/js/importDefaultBasename"},
		);
	},
);
