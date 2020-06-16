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
	"no arguments",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						function f() {
							console.log(arguments);
						}
					`,
					dedent`
						(function () {
							console.log(arguments);
						})();
					`,
					dedent`
						class C {
							fn() {
								console.log(arguments);
							}
						}
					`,
					dedent`
						const o = {
							fn() {
								console.log(arguments);
							},
						};
					`,
				],
			},
			{category: "lint/js/noArguments"},
		);
	},
);
