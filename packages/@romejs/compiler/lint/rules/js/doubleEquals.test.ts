/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"double equals",
	async (t) => {
		await testLint(
			t,
			{
				invalid: ["foo == bar"],
				valid: ["foo == null", "foo != null", "null == foo", "null != foo"],
			},
			{category: "lint/js/doubleEquals"},
		);
	},
);
