/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"prefer function declarations",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"const foo = function () {};",
					"const foo = () => {};",
					// Doesn't need to be an arrow function because 'this' isn't from outer scope
					"const foo = () => {function bar() {this;}};",
				],
				valid: [
					// Allow arrow functions that use 'this' from outer scope
					"const foo = () => {this;};",
					// Allow functions with return types since you can't express that with a declaration
					"const foo: string = function () {};",
				],
			},
			{
				category: "lint/js/preferFunctionDeclarations",
			},
		);
	},
);
