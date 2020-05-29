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
	"disallow unsafe usage of break, continue, throw and return",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
					function greet1() {
						try {
							throw new Error("Try")
						} catch(err) {
							throw err;
						} finally {
							return 1;
						}
					}
				`,
					dedent`
					function greet2() {
						try {
							throw new Error("Try")
						} catch(err) {
							throw err;
						} finally {
							break;
						}
					}
				`,
					dedent`
					function greet3() {
						try {
							throw new Error("Try")
						} catch(err) {
							throw err;
						} finally {
							continue;
						}
					}
				`,
					dedent`
					function greet4() {
						try {
							throw new Error("Try")
						} catch(err) {
							throw err;
						} finally {
							throw new Error("Finally");
						}
					}
				`,
				],
			},
			{category: "lint/js/noUnsafeFinally"},
		);
	},
);
