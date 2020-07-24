/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ExtensionHandler} from "./types";
import {parseHTML} from "@romefrontend/html-parser";

// These are extensions that be implicitly tried when a file is referenced
// This is mostly for compatibility with Node.js projects. This list should not
// be extended. Explicit extensions are required in the browser for as modules and
// should be required everywhere.
// TypeScript is unfortunately included here as it produces an error if you use an
// import source with ".ts"
export const IMPLICIT_JS_EXTENSIONS = ["js", "ts", "tsx", "json"];

// Used when filtering files
export const JS_EXTENSIONS: Array<string> = [];

export const htmlHandler: ExtensionHandler = {
	ext: "html",
	canLint: true,
	canFormat: true,
	language: "html",
	hasTabs: true,

	async parse({mtime, path, file, worker}) {
		const sourceText = await worker.readFile(file.real);
		const ast = parseHTML({
			input: sourceText,
			mtime,
			path,
		});
		return {
			sourceText,
			ast,
			astModifiedFromSource: false,
		};
	},
};

export const htmHandler: ExtensionHandler = {
	...htmlHandler,
	ext: "htm",
};
