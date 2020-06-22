/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"react style prop object",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<div style={true} />",
					"<div style={1} />",
					"<div style={undefined} />",
					`<div style="color: 'red'" />`,
					`<div style={"color: 'red'"} />`,
				],
				valid: ["<div style={{color: 'red'}} />"],
			},
			{category: "lint/react/stylePropObject"},
		);
	},
);
