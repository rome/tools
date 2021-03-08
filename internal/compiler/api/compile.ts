/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Mapping, SourceMapConsumer} from "@internal/codec-source-map";
import {Diagnostic, DiagnosticSuppression} from "@internal/diagnostics";
import {Cache} from "@internal/compiler";
import {formatAST} from "@internal/formatter";
import {CompileRequest} from "../types";
import transform from "../methods/transform";
import {Path} from "@internal/path";

export type CompileResult = {
	mappings: Mapping[];
	diagnostics: Diagnostic[];
	suppressions: DiagnosticSuppression[];
	cacheDependencies: Path[];
	compiledCode: string;
	sourceText: string;
};

const compileCache: Cache<CompileResult> = new Cache();

export default async function compile(
	req: CompileRequest,
): Promise<CompileResult> {
	const {sourceText, ast, project} = req;

	const query = Cache.buildQuery(req);
	const cached: undefined | CompileResult = compileCache.get(query);
	if (cached) {
		return cached;
	}

	const {
		ast: transformedAst,
		diagnostics,
		suppressions,
		cacheDependencies,
	} = await transform(req);

	const formatted = formatAST(
		transformedAst,
		{
			projectConfig: project?.config,
			typeAnnotations: false,
			indent: req.stage === "compileForBundle" ? 1 : 0,
			sourceMaps: true,
			allowInterpreterDirective: false,
		},
	);

	if (req.inputSourceMap !== undefined) {
		const inputSourceMap = SourceMapConsumer.fromJSON(req.inputSourceMap);
		const mappings: Mapping[] = [];

		for (const mapping of formatted.mappings) {
			const actual = inputSourceMap.exactOriginalPositionFor(
				mapping.original.line,
				mapping.original.column,
			);

			if (actual !== undefined) {
				if (
					mapping.original.line !== actual.line ||
					mapping.original.column !== actual.column
				) {
					mappings.push({
						...mapping,
						original: {
							line: actual.line,
							column: actual.column,
						},
					});
				} else {
					mappings.push(mapping);
				}
			}
		}

		formatted.mappings = mappings;
	}

	const res: CompileResult = {
		compiledCode: formatted.code,
		mappings: formatted.mappings,
		diagnostics: [...ast.diagnostics, ...diagnostics],
		cacheDependencies,
		suppressions,
		sourceText,
	};

	compileCache.set(query, res);
	return res;
}
