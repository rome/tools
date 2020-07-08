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
	"case single statement",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					// Multiple statements
					dedent`
						switch (foo) {
							case true:
							case false:
								let foo = '';
								foo;
						}
					`,
				],
				valid: [
					// Single statement
					dedent`
						switch (foo) {
							case true:
							case false:
								return 'yes';
						}
					`,
					// Single block
					dedent`
						switch (foo) {
							case true: {
								// empty
							}
						}
					`,
					// Nothing
					dedent`
						switch (foo) {
							case true:
						}
					`,
				],
			},
			{category: "lint/js/caseSingleStatement"},
		);
	},
);
