/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"negation else",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"if (!true) {consequent;} else {alternate;}",
					"!true ? consequent : alternate",
				],
				valid: ["if (!true) {consequent;}", "true ? consequent : alternate"],
			},
			{category: "lint/js/negationElse"},
		);
	},
);
