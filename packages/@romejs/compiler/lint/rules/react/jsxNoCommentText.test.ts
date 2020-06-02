/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"react jsx no comment text",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"const a = <div>// comment</div>",
					"const a = <div>/* comment */</div>",
					"const a = <div>/** comment */</div>",
				],
				valid: [
					"const a = <div>{/* comment */}</div>",
					"const a = <div>{/** comment */}</div>",
					'const a = <div className={"cls" /* comment */}></div>',
				],
			},
			{category: "lint/react/jsxNoCommentText"},
		);
	},
);
