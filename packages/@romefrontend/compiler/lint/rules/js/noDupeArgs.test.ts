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
	"no duplicated args allowed",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						function hello(a, a) {
							//
						}
					`,
					dedent`
						const hello = (a, a) => {
							//
						}
					`,
					dedent`
						const hello = function (a, a) {
							//
						}
					`,
				],
				valid: [
					dedent`
						function foo(foo) {
							console.log(foo)
						}
					`,
					dedent`
						const foo = "test"
						function bar(foo) {
							console.log(foo)
						}
					`,
				],
			},
			{category: "lint/js/noDupeArgs"},
		);
	},
);
