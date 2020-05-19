/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLintMultiple} from "../testHelpers";
import {dedent} from "@romejs/string-utils";

test(
	"no delete",
	async (t) => {
		await testLintMultiple(
			t,
			[
				dedent`
          const arr = [['a','b','c'], [1, 2, 3]];
          delete arr[0][2];
        `,
				dedent`
          const obj = {a: {b: {c: 123}}};
          delete obj.a.b.c;
        `,
			],
			{category: "lint/js/noDelete"},
		);
	},
);
