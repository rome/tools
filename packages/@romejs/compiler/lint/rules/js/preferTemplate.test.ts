/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"prefer template",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					`const foo = 'bar'; console.log(foo + 'baz')`,
					`console.log((1 * 2) + 'baz')`,
				],
			},
			{
				category: "lint/js/preferTemplate",
			},
		);
	},
);
