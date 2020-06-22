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
	"no shorthand array type",
	async (t) => {
		// TypeScript
		await testLint(
			t,
			{
				invalid: [
					dedent`
						let valid: Array<foo>;
						let invalid: bar[];
					`,
				],
			},
			{category: "lint/js/noShorthandArrayType", syntax: ["ts"]},
		);
	},
);
