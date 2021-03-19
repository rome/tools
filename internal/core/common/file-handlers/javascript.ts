/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ConstJSProgramSyntax, ConstJSSourceType} from "@internal/ast";
import {PartialExtensionHandler} from "./types";
import {parseJS, tokenizeJS} from "@internal/js-parser";

// These are extensions that be implicitly tried when a file is referenced
// This is mostly for compatibility with Node.js projects. This list should not
// be extended. Explicit extensions are required in the browser for as modules and
// should be required everywhere.
// TypeScript is unfortunately included here as it produces an error if you use an
// import source with ".ts"
export const IMPLICIT_JS_EXTENSIONS = ["js", "ts", "tsx", "json"];

// Used when filtering files
export const JS_EXTENSIONS: string[] = [];

function buildJSHandler(
	ext: string,
	syntax: ConstJSProgramSyntax[],
	sourceTypeJS?: ConstJSSourceType,
): PartialExtensionHandler {
	JS_EXTENSIONS.push(ext);

	return {
		sourceTypeJS,
		language: "js",
		mime: "application/javascript",
		hasTabs: true,
		capabilities: {
			lint: true,
			format: true,
		},

		async parse(
			{integrity, sourceTypeJS, manifestPath, path, file, project, worker},
		) {
			let fileSyntax = syntax;
			if (project.config.parser.jsxEverywhere) {
				fileSyntax = [...fileSyntax, "jsx"];
			}

			const sourceText = await worker.readFileText(file);
			const ast = parseJS({
				input: sourceText,
				integrity,
				manifestPath,
				path,
				sourceType: sourceTypeJS,
				syntax: fileSyntax,
				allowReturnOutsideFunction: sourceTypeJS === "script",
			});
			return {
				sourceText,
				ast,
				astModifiedFromSource: false,
			};
		},

		async tokenize({integrity, path, file, worker}) {
			const sourceText = await worker.readFileText(file);
			const tokens = tokenizeJS({input: sourceText, integrity, path});
			return {
				sourceText,
				tokens,
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
