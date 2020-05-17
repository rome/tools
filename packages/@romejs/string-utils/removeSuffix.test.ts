/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {removeSuffix} from "./removeSuffix";
import {test} from "rome";

test(
	"removeSuffix",
	(t) => {
		const testCases = [
			{firstInput: "romeTest", secondInput: "Test", expected: "rome"},
			{firstInput: "romeTest", secondInput: "rome", expected: "romeTest"},
			{firstInput: "romeTest", secondInput: "123", expected: "romeTest"},
		];

		testCases.forEach((td) => {
			t.is(removeSuffix(td.firstInput, td.secondInput), td.expected);
		});
	},
);
