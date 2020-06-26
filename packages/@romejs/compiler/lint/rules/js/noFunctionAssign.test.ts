/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"no function reassignment",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"function foo() {}; foo = bar;",
					"function foo() { foo = bar; }",
					"foo = bar; function foo() { };",
					"[foo] = bar; function foo() { };",
					"({x: foo = 0} = bar); function foo() { };",
					"function foo() { [foo] = bar; }",
					"(function() { ({x: foo = 0} = bar); function foo() { }; })();",
				],
				valid: [
					"function foo() { var foo = bar; }",
					"function foo(foo) { foo = bar; }",
					"function foo() { var foo; foo = bar; }",
					"var foo = () => {}; foo = bar;",
					"var foo = function() {}; foo = bar;",
					"var foo = function() { foo = bar; };",
					`import bar from 'bar'; function foo() { var foo = bar; }`,
				],
			},
			{category: "lint/js/noFunctionAssign"},
		);
	},
);
