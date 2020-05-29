/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"no async promise executor",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"new Promise(async function foo() {})",
					"new Promise(async () => {})",
					"new Promise(((((async () => {})))))",
				],
				valid: [
					"new Promise(() => {})",
					"new Promise(() => {}, async function unrelated() {})",
					"class Foo {} new Foo(async () => {})",
				],
			},
			{category: "lint/js/noAsyncPromiseExecutor"},
		);
	},
);
