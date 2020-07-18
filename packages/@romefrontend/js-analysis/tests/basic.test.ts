/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostics} from "@romefrontend/diagnostics";
import {createDefaultProjectConfig} from "@romefrontend/project";
import {test} from "rome";
import {check} from "@romefrontend/js-analysis";
import {parseJS} from "@romefrontend/js-parser";
import {createUnknownFilePath} from "@romefrontend/path";

async function testCheck(code: string): Promise<Diagnostics> {
	const ast = parseJS({
		input: code,
		sourceType: "module",
		path: createUnknownFilePath("unknown"),
	});

	return check({
		ast,
		project: {
			directory: undefined,
			config: createDefaultProjectConfig(),
		},
		provider: {
			getExportTypes() {
				return Promise.reject("unsupported");
			},
		},
	});
}

test(
	"discovers require('module') call",
	async () => {
		testCheck;

		/*const diagnostics = await testCheck(`
    const a: number = '';
  `);

  console.log(diagnostics);*/
	},
);
