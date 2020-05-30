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
	"no cond assign",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						for (let i = 1; i = 10; i++) {
							console.log('foo');
						}
					`,
					dedent`
						if (foo = 'bar') {
							console.log('foo');
						}
					`,
					dedent`
						while (foo = 'bar') {
							console.log('foo');
						}
					`,
					dedent`
						do {
							console.log('foo');
						} while (foo = 'bar')
                    `,
                    dedent`
                        (foo = bar) ? foo() : baz();
					`,
				],
				valid: [
					dedent`
						while ((foo = foo.bar) !== undefined) {
							console.log(foo);
						}
					`,
					dedent`
						if (foo++ === 3) {
							console.log(foo);
						}
                    `,
                    dedent`
                        foo = bar ? foo() : baz();
                    `,
				],
			},
			{category: "lint/js/noCondAssign"},
		);
	},
);
