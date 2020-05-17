/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"confusing language",
	async (t) => {
		await testLintMultiple(
			t,
			[
				"// the blacklist",
				"/* the\nblacklist */",
				"blacklist;",
				"BLACKLIST;",
				"someBlacklist;",
				"SOME_BLACKLIST;",
			],
			{category: "lint/confusingLanguage"},
		);
	},
);
