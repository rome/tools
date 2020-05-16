/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLint} from '../testHelpers';

test(
	'unsafe negation',
	async (t) => {
		await testLint(t, `!1 in [1,2]`, {category: 'lint/unsafeNegation'});
	},
);
