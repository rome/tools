/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLintMultiple} from '../testHelpers';

test(
	'enforce single var declarator',
	async (t) => {
		await testLintMultiple(
			t,
			[
				// import statements
				`import {b, a, c, D} from "mod";`,
				`import {b as A, a as C, B} from "mod";`,
				`import {c, b as b2, b as b1, b} from "mod";`,
				// export external statements
				`export {b, a, c, D} from "mod";`,
				`export {b as A, a as C, B} from "mod";`,
				`export {c, b as b2, b as b1, b} from "mod";`,
				// export local statements
				`export {b, a, c, D};`,
				`export {b as A, a as C, B};`,
				`export {c, b as b2, b as b1, b};`,
			],
			{category: 'lint/sortImportExportSpecifiers'},
		);
	},
);
