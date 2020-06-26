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
	"getter return",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						class p {
							get name() {
								console.log('hello');
							}
						}
					`,
					dedent`
						let p = {
							get name() {
								console.log('hello');
							},
						};
				 `,
				],
				valid: [
					dedent`
						let p = {};
						Object.defineProperty(p, {
							get: function () {
								console.log('hello');
							},
						});
				 `,
				],
			},
			{category: "lint/js/getterReturn"},
		);
	},
);
