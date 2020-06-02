/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"react no danger",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					'<div dangerouslySetInnerHTML={{ __html: "child" }}></div>;',
					`React.createElement('div', {dangerouslySetInnerHTML: { __html: "child" }})`,
					`createElement('div', {dangerouslySetInnerHTML: { __html: "child" }})`,
				],
				valid: [
					"<div>Hello World</div>;",
					"React.createElement('div', 'child)",
					"createElement('div', 'child)",
				],
			},
			{category: "lint/react/noDanger"},
		);
	},
);
