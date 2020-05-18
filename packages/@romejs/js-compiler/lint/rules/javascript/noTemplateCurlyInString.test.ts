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
	"no template curly in string",
	async (t) => {
		await testLint(
			t,
			dedent`
        const user = "Faustina";
        const helloUser = "Hello, \${user}!";
      `,
			{category: "lint/javascript/noTemplateCurlyInString"},
		);
	},
);
