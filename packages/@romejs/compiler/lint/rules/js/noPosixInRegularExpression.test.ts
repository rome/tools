/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"no POSIX in regular expression",
	async (t) => {
		testLint(
			t,
			{invalid: ["const pattern = /[[:alpha:]]/", "const pattern = /[[.ch.]]/"]},
			{category: "lint/js/noPosixInRegularExpression"},
		);
	},
);
