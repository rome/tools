/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {PartialExtensionHandler} from "./types";
import {parseJS} from "@internal/js-parser";

export const textHandler: PartialExtensionHandler = {
	sourceTypeJS: "module",
	language: "unknown",
	hasTabs: false,
	capabilities: {
		lint: false,
		format: false,
	},

	async parse({path, mtime, file, worker}) {
		const src = await worker.readFile(file.real);
		const serial = JSON.stringify(src);
		const sourceText = `export default ${serial};`;

		return {
			// Shouldn't error
			ast: parseJS({input: sourceText, mtime, sourceType: "module", path}),
			sourceText,
			astModifiedFromSource: true,
		};
	},
};
