/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";
import {dedent} from "@romejs/string-utils";

test(
	"duplicate import source",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					dedent`
						import	foo	from	'./testdummy.ts';
						import	{bar}	from	'./testdummy.ts';
						import	type	{fooType}	from	'./testdummy.ts';

						const	typedFoo:	fooType	=	{
							type:	'foo'
						};
					`,
				],
			},
			{category: "lint/js/duplicateImportSource"},
		);
	},
);
