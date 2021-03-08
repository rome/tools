import { tokenizeHTML } from '@internal/html-parser';
/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {PartialExtensionHandler} from "./types";
import {parseHTML} from "@internal/html-parser";

// These are extensions that be implicitly tried when a file is referenced
// This is mostly for compatibility with Node.js projects. This list should not
// be extended. Explicit extensions are required in the browser for as modules and
// should be required everywhere.
// TypeScript is unfortunately included here as it produces an error if you use an
// import source with ".ts"
export const IMPLICIT_JS_EXTENSIONS = ["js", "ts", "tsx", "json"];

export const htmlHandler: PartialExtensionHandler = {
	language: "html",
	mime: "text/html",
	hasTabs: true,
	capabilities: {
		lint: true,
		format: true,
	},

	async parse({integrity, path, file, worker}) {
		const sourceText = await worker.readFileText(file);
		const ast = parseHTML({
			input: sourceText,
			integrity,
			path,
		});
		return {
			sourceText,
			ast,
			astModifiedFromSource: false,
		};
	},

	async tokenize({integrity, path, file, worker}) {
		const sourceText = await worker.readFileText(file);
		const tokens = tokenizeHTML({
			input: sourceText,
			integrity,
			path,
		});
		return {
			sourceText,
			tokens,
		};
	},
};
