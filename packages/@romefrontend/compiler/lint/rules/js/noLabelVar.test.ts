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
	"no label var",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						const x = 'test';
						x: const y = 'test';
					`,
				],
				valid: [
					dedent`
						const x = 'test';
						z: const y = 'test';
					`,
				],
			},
			{category: "lint/js/noLabelVar"},
		);
	},
);
