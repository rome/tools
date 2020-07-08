/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";
import {dedent} from "@romefrontend/string-utils";

test(
	"disallow unnecessary boolean casts",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
					if (Boolean(foo)) {
						return foo;
					}
				`,
					dedent`
					while (!!foo) {
						return foo;
					}
				`,
					dedent`
					let x = 1;
					do {
						1 + 1;
					} while (Boolean(x));
				`,
					dedent`
					for (; !!foo; ) {
						return 1 + 1;
					}
				`,
				],
			},
			{category: "lint/js/noExtraBooleanCast"},
		);
	},
);
