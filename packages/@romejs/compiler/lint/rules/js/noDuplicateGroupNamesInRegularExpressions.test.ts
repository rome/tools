/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"disallow duplicate group names in regular expression",
	async (t) => {
		await testLint(
			t,
			`/(?<month>[0-9])-(?<year>[0-9])-(?<month>[0-9])-(?<year>[0-9])-(?<day>[0-9])-([0-9])-(?<month>[0-9])/`,
			{category: "lint/js/noDuplicateGroupNamesInRegularExpressions"},
		);
	},
);
