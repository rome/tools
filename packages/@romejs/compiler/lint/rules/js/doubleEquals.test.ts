/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"double equals",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// VALID
				"foo == null",
				"foo != null",
				"null == foo",
				"null != foo",
				// INVALID
				"foo == bar",
			],
			{category: "lint/js/doubleEquals"},
		);
	},
);
