/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';

import {testLintMultiple} from '../testHelpers';

test(
	'do not allow findDOMNode',
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				'findDOMNode(this).scrollIntoView()',
				'ReactDOM.findDOMNode(this).scrollIntoView()',
				// VALID
				'this.node.scrollIntoView()',
			],
			{category: 'lint/noFindDOMNode'},
		);
	},
);
