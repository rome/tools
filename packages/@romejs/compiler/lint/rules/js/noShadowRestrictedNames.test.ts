/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"no shadow restricted names",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"function NaN() {}",
					"let Set;",
					"try {  } catch(Object) {}",
					"!function Array() {}",
					"function test(JSON) {console.log(JSON)}",
				],
			},
			{category: "lint/js/noShadowRestrictedNames"},
		);
	},
);
