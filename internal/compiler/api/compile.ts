/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Mappings, SourceMapConsumer} from "@internal/codec-source-map";
import {DiagnosticSuppressions, Diagnostics} from "@internal/diagnostics";
import {Cache} from "@internal/compiler";
import {formatAST} from "@internal/formatter";
import {CompileRequest} from "../types";
import transform from "../methods/transform";

export type CompileResult = {
	mappings: Mappings;
	diagnostics: Diagnostics;
	suppressions: DiagnosticSuppressions;
	cacheDependencies: Array<string>;
	compiledCode: string;
	sourceText: string;
};

const compileCache: Cache<CompileResult> = new Cache();

export default async function compile(
	req: CompileRequest,
): Promise<CompileResult> {
	const {sourceText, ast} = req;

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
			typeAnnotations: false,
			indent: req.stage === "compileForBundle" ? 1 : 0,
			sourceMaps: true,
			allowInterpreterDirective: false,
		},
	);

	if (req.inputSourceMap !== undefined) {
		const inputSourceMap = SourceMapConsumer.fromJSON(req.inputSourceMap);
		const mappings: Mappings = [];

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
