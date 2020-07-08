/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"disallow var",
	async (t) => {
		await testLint(
			t,
			{invalid: ["var foobar;\nfoobar"]},
			{category: "lint/js/noVar"},
		);
	},
);
