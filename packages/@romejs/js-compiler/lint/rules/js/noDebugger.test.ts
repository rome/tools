/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";
import {dedent} from "@romejs/string-utils";

test(
	"no debugger",
	async (t) => {
		await testLint(
			t,
			dedent`
        const test = { debugger: 1 };
        test.debugger;
      `,
			{category: "lint/js/noDebugger"},
		);

		await testLint(
			t,
			dedent`
        debugger;
      `,
			{category: "lint/js/noDebugger"},
		);
	},
);
