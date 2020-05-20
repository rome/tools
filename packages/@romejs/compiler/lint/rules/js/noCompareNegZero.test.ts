/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"disallows comparing negative zero",
	async (t) => {
		await testLint(t, "(1 >= -0)", {category: "lint/js/noCompareNegZero"});
		await testLint(t, "(1 >= 0)", {category: "lint/js/noCompareNegZero"});
	},
);
