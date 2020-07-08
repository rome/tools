/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"undeclared variable",
	async (t) => {
		await testLint(
			t,
			{invalid: ["foobar;"]},
			{category: "lint/js/undeclaredVariables"},
		);
	},
);
