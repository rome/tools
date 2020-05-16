/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLintMultiple} from '../testHelpers';

test(
	'no async promise executor',
	async (t) => {
		await testLintMultiple(
			t,
			[
				// VALID
				'new Promise(() => {})',
				'new Promise(() => {}, async function unrelated() {})',
				'class Foo {} new Foo(async () => {})',
				// INVALID
				'new Promise(async function foo() {})',
				'new Promise(async () => {})',
				'new Promise(((((async () => {})))))',
			],
			{category: 'lint/noAsyncPromiseExecutor'},
		);
	},
);
