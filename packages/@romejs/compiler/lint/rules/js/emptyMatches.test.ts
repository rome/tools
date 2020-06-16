/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"empty matches; may match infinitely",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					// infinite match possible
					"let a = /a*/",
					"let b = /a*(abc)?[1,2,3]*/",
				],
				valid: ["let a = /a*(abc)+[1,2,3]?/", "let b = /a+(abc)*/"],
			},
			{category: "lint/js/emptyMatches"},
		);
	},
);
