/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"react no children props",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<MyComponent children={'foo'}></MyComponent>",
					"React.createElement('div', {children: 'foo'})",
					"createElement('div', {children: 'foo'})",
				],
				valid: [
					"<MyComponent><AnotherComponent /></MyComponent  >",
					"React.createElement('div', {}, 'children')",
					"React.createElement('div', child1, 'child2')",
				],
			},
			{category: "lint/react/noChildrenProp"},
		);
	},
);
