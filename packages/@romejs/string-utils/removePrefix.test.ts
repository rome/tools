/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {removePrefix} from "./removePrefix";
import {test} from "rome";

test(
	"removePrefix",
	(t) => {
		const testCases = [
			{firstInput: "romeTest", secondInput: "rome", expected: "Test"},
			{firstInput: "Testrome", secondInput: "rome", expected: "Testrome"},
			{firstInput: "romeTest", secondInput: "123", expected: "romeTest"},
		];

		testCases.forEach((td) => {
			t.is(removePrefix(td.firstInput, td.secondInput), td.expected);
		});
	},
);
