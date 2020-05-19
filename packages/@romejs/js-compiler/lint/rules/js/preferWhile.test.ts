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
	"prefer while",
	async (t) => {
		await testLintMultiple(
			t,
			[
				dedent`
          for (; x.running;) {
            x.step();
          }
        `,
				dedent`
          for (;;) {
            doSomething();
          }
        `,
			],
			{category: "lint/js/preferWhile"},
		);
	},
);
