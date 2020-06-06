/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"no empty character class in regular expression",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"let foo = /^abc[]/;foo;",
					"let foo = /foo[]bar/;foo;",
					'let foo = "";if (foo.match(/^abc[]/)) { foo; }',
					"let foo = /[]]/;foo;",
					"let foo = /\\[[]/;foo;",
					"let foo = /\\[\\[\\]a-z[]/;foo;",
				],
				valid: [
					"let foo = /^abc[a-zA-Z]/;foo;",
					'let regExp = new RegExp("^abc[]");regExp;',
					"let foo = /^abc/;foo;",
					"let foo = /[\\[]/;foo;",
					"let foo = /[\\]]/;foo;",
					"let foo = /[a-zA-Z\\[]/;foo;",
					"let foo = /[[]/;foo;",
					"let foo = /[\\[a-z[]]/;foo;",
					"let foo = /[\\-\\[\\]\\/\\{\\}\\(\\)\\*\\+\\?\\.\\\\^\\$\\|]/g;foo;",
					"let foo = /[\\]]/uy;foo;",
					"let foo = /[\\]]/s;foo;",
					"let foo = /\\[]/;foo;",
				],
			},
			{category: "lint/js/noEmptyCharacterClass"},
		);
	},
);
