/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ConstJSProgramSyntax, ConstJSSourceType} from "@internal/ast";
import {ExtensionHandler} from "./types";
import {parseJS} from "@internal/js-parser";

// These are extensions that be implicitly tried when a file is referenced
// This is mostly for compatibility with Node.js projects. This list should not
// be extended. Explicit extensions are required in the browser for as modules and
// should be required everywhere.
// TypeScript is unfortunately included here as it produces an error if you use an
// import source with ".ts"
export const IMPLICIT_JS_EXTENSIONS = ["js", "ts", "tsx", "json"];

// Used when filtering files
export const JS_EXTENSIONS: Array<string> = [];

function buildJSHandler(
	ext: string,
	syntax: Array<ConstJSProgramSyntax>,
	sourceTypeJS?: ConstJSSourceType,
): ExtensionHandler {
	JS_EXTENSIONS.push(ext);

	return {
		ext,
		sourceTypeJS,
		language: "js",
		hasTabs: true,
		capabilities: {
			lint: true,
			format: true,
		},

		async parse({mtime, sourceTypeJS, manifestPath, path, file, worker}) {
			const sourceText = await worker.readFile(file.real);
			const ast = parseJS({
				input: sourceText,
				mtime,
				manifestPath,
				path,
				sourceType: sourceTypeJS,
				syntax,
				allowReturnOutsideFunction: sourceTypeJS === "script",
			});
			return {
				sourceText,
				ast,
				astModifiedFromSource: false,
			};
		},
	};
}

export const jsHandler = buildJSHandler("js", []);
export const jsxHandler = buildJSHandler("jsx", ["jsx"]);
export const cjsHandler = buildJSHandler("cjs", [], "script");
export const mjsHandler = buildJSHandler("mjs", [], "module");
export const tsHandler = buildJSHandler("ts", ["ts"], "module");
export const tsxHandler = buildJSHandler("tsx", ["ts", "jsx"], "module");
