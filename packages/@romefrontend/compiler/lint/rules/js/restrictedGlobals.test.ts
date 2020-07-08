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
	"restricted globals",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					// invalid, event is used as a global.
					"console.log(event);",
					"foo(event)",
				],
				valid: [
					// valid use of event into the function scope.
					dedent`
						function foo(event) {
							console.info(event);
						}
					`,
				],
			},
			{
				category: "lint/js/restrictedGlobals",
			},
		);
	},
);
