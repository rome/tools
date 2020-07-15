/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"react no find dom node",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"findDOMNode(this).scrollIntoView()",
					"ReactDOM.findDOMNode(this).scrollIntoView()",
				],
				valid: ["this.node.scrollIntoView()"],
				filename: "file.tsx",
				category: "lint/react/noFindDOMNode",
			},
		);
	},
);
