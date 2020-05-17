/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"prefer template",
	async (t) => {
		await testLint(
			t,
			`const foo = 'bar'; console.log(foo + 'baz')`,
			{
				category: "lint/preferTemplate",
			},
		);

		await testLint(
			t,
			`console.log((1 * 2) + 'baz')`,
			{
				category: "lint/preferTemplate",
			},
		);
	},
);
