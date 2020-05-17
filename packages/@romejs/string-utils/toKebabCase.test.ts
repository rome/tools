/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {toKebabCase} from "./toKebabCase";
import {test} from "rome";

test(
	"toKebabCase",
	(t) => {
		const testCases = [
			{input: "rometest", expected: "rometest"},
			{input: "rome test", expected: "rome-test"},
			{input: "RoMe TeSt", expected: "ro-me-te-st"},
			{input: "ROME TEST", expected: "rome-test"},
		];

		testCases.forEach((td) => {
			t.is(toKebabCase(td.input), td.expected);
		});
	},
);
