/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import * as compiler from "@romejs/compiler";
import {check as typeCheck} from "@romejs/js-analysis";
import {ConstProgramSyntax, ConstSourceType} from "@romejs/ast";
import {formatJS} from "@romejs/formatter";
import {
	ExtensionHandler,
	ExtensionHandlerMethodInfo,
	ExtensionLintInfo,
	ExtensionLintResult,
} from "./types";
import {ParseJSResult} from "@romejs/core/worker/Worker";

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
	syntax: Array<ConstProgramSyntax>,
	sourceType?: ConstSourceType,
): ExtensionHandler {
	JS_EXTENSIONS.push(ext);

	return {
		ext,
		syntax,
		sourceType,

		async analyzeDependencies({file, worker, parseOptions}) {
			const {ast, sourceText, project, generated} = await worker.parseJS(
				file,
				parseOptions,
			);
			worker.logger.info(`Analyzing:`, file.real);

			return worker.api.interceptAndAddGeneratedToDiagnostics(
				await compiler.analyzeDependencies({
					ref: file,
					ast,
					sourceText,
					project,
					options: {},
				}),
				generated,
			);
		},

		async toJavaScript({file, worker}) {
			return {
				sourceText: await worker.readFile(file.real),
				generated: false,
			};
		},

		async format(info: ExtensionHandlerMethodInfo): Promise<ExtensionLintResult> {
			const {file: ref, parseOptions, worker} = info;

			const {ast, sourceText, generated}: ParseJSResult = await worker.parseJS(
				ref,
				parseOptions,
			);

			const out = formatJS(
				ast,
				{
					sourceText,
				},
			);

			return worker.api.interceptAndAddGeneratedToDiagnostics(
				{
					formatted: out.code,
					sourceText,
					suppressions: [],
					diagnostics: ast.diagnostics,
				},
				generated,
			);
		},

		async lint(info: ExtensionLintInfo): Promise<ExtensionLintResult> {
			const {file: ref, project, parseOptions, options, worker} = info;

			const {ast, sourceText, generated}: ParseJSResult = await worker.parseJS(
				ref,
				parseOptions,
			);

			worker.logger.info(`Linting: `, ref.real);

			// Run the compiler in lint-mode which is where all the rules are actually ran
			const res = await compiler.lint({
				applyFixes: options.applyFixes,
				ref,
				options: {
					lint: options.compilerOptions,
				},
				ast,
				project,
				sourceText,
			});

			// Extract lint diagnostics
			let {diagnostics} = res;

			// Only enable typechecking if enabled in .romeconfig
			let typeCheckingEnabled = project.config.typeCheck.enabled === true;
			if (project.config.typeCheck.libs.has(ref.real)) {
				// don't typecheck lib files
				typeCheckingEnabled = false;
			}

			// Run type checking if necessary
			if (typeCheckingEnabled) {
				const typeCheckProvider = await worker.getTypeCheckProvider(
					ref.project,
					options.prefetchedModuleSignatures,
					parseOptions,
				);
				const typeDiagnostics = await typeCheck({
					ast,
					provider: typeCheckProvider,
					project,
				});
				diagnostics = [...diagnostics, ...typeDiagnostics];
			}

			return worker.api.interceptAndAddGeneratedToDiagnostics(
				{
					suppressions: res.suppressions,
					diagnostics,
					sourceText,
					formatted: res.src,
				},
				generated,
			);
		},
	};
}

export const jsHandler = buildJSHandler("js", []);
export const jsxHandler = buildJSHandler("jsx", ["jsx"]);
export const cjsHandler = buildJSHandler("cjs", [], "script");
export const mjsHandler = buildJSHandler("mjs", [], "module");
export const tsHandler = buildJSHandler("ts", ["ts"], "module");
export const tsxHandler = buildJSHandler("tsx", ["ts", "jsx"], "module");
