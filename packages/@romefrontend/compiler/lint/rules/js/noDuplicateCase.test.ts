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
	"no duplicated switch cases allowed",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						const expr = 'a';
						switch (expr) {
							case 'a':
								break;
							case 'b':
								break;
							case 'c':
								break;
							case 'd':
								break;
							case 'c':
								break;
							default:
								break;
						}
					`,
					dedent`
						const expr = 3;
						switch (expr) {
							case 1:
								break;
							case 2:
								break;
							case 3:
								break;
							case 2:
								break;
							default:
								break;
						}
					`,
					dedent`
						const expr = 3;
						switch (expr) {
							case 1:
								break;
							case 2n:
								break;
							case 3:
								break;
							case 2n:
								break;
							default:
								break;
						}
                    `,
					dedent`
						const foo = 'a';
						switch ('a') {
							case foo:
								break;
							case foo:
								break;
							default:
								break;
						}
					`,
					dedent`
						const foo = 'a';
						switch ('a') {
							case 'foo':
								break;
							case "foo":
								break;
							default:
								break;
						}
					`,
					dedent`
						const foo = 'a';
						switch ('a') {
							case null:
								break;
							case null:
								break;
							default:
								break;
						}
					`,
				],
				valid: [
					dedent`
						const expr = 'a';
						switch (expr) {
							case 'a':
								break;
							case 'b':
								break;
							case 'c':
								break;
							case 'd':
								break;
							default:
								break;
						}
					`,
					dedent`
						const expr = 3;
						switch (expr) {
							case 1:
								break;
							case 2:
								break;
							case 3:
								break;
							case 2n:
								break;
							default:
								break;
						}
                    `,
					dedent`
						const expr = 3;
						switch (expr) {
							case 1:
								break;
							case "1":
								break;
							case 1n:
								break;
							case "null":
								break;
							case null:
								break;
							case foo:
								break;
							case "foo":
								break;
							default:
								break;
						}
                	`,
				],
			},
			{category: "lint/js/noDuplicateCase"},
		);
	},
);
