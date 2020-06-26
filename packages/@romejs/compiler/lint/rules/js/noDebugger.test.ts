/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";
import {dedent} from "@romejs/string-utils";

test(
	"no debugger",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						debugger;
					`,
				],
				valid: [
					dedent`
						const test = { debugger: 1 };
						test.debugger;
					`,
				],
			},
			{category: "lint/js/noDebugger"},
		);
	},
);
