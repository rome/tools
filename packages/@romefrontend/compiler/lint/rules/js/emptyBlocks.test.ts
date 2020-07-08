/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";
import {dedent} from "@romefrontend/string-utils";

test(
	"empty blocks",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						if (foo) {}
					`,
					dedent`
						if (foo) {
							// foo;
						} else {}
					`,
				],
				valid: [
					dedent`
						if (foo) foo;
					`,
					dedent`
						if (foo) {
							foo;
						}
					`,
					dedent`
						if (foo) {
							// empty
						}
					`,
				],
			},
			{category: "lint/js/emptyBlocks"},
		);
	},
);
