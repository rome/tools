/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {PartialExtensionHandler} from "./types";
import {parseJS} from "@romefrontend/js-parser";

export const textHandler: PartialExtensionHandler = {
	sourceTypeJS: "module",
	canLint: false,
	canFormat: false,

	async parse({path, file, worker}) {
		const src = await worker.readFile(file.real);
		const serial = JSON.stringify(src);
		const sourceText = `export default ${serial};`;

		return {
			// Shouldn't error
			ast: parseJS({input: sourceText, sourceType: "module", path}),
			sourceText,
			astModifiedFromSource: true,
		};
	},
};
