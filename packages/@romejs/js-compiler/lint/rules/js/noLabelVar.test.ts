/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";
import {dedent} from "@romejs/string-utils";

test(
	"no label var",
	async (t) => {
		await testLint(
			t,
			dedent`
        const x = 'test';
        x: const y = 'test';
      `,
			{category: "lint/js/noLabelVar"},
		);

		await testLint(
			t,
			dedent`
        const x = 'test';
        z: const y = 'test';
      `,
			{category: "lint/js/noLabelVar"},
		);
	},
);
