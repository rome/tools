/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticSuppressions, Diagnostics} from "@romejs/diagnostics";
import {LintRequest} from "../types";
import {Cache, CompilerContext} from "@romejs/compiler";
import {formatAST} from "@romejs/formatter";
import {addSuppressions} from "./suppressions";
import {lintTransforms} from "./rules/index";

export type LintResult = {
	diagnostics: Diagnostics;
	suppressions: DiagnosticSuppressions;
	src: string;
};

const lintCache: Cache<LintResult> = new Cache();

export default async function lint(req: LintRequest): Promise<LintResult> {
	const {ast, sourceText, project, applyRecommendedFixes, options} = req;

	const query = Cache.buildQuery(req, {applyRecommendedFixes});
	const cached = lintCache.get(query);
	if (cached) {
		return cached;
	}

	// Perform autofixes
	let formatAst = ast;
	if (applyRecommendedFixes) {
		const formatContext = new CompilerContext({
			ref: req.ref,
			sourceText: req.sourceText,
			options,
			ast,
			project,
			frozen: false,
			origin: {
				category: "check",
			},
		});

		formatAst = formatContext.reduceRoot(ast, lintTransforms);
		formatAst = addSuppressions(formatContext, formatAst);
	}
	const formattedCode = formatAST(
		formatAst,
		{
			typeAnnotations: true,
			sourceMaps: true,
			format: "pretty",
			sourceText,
		},
	).code;

	// Run lints (could be with the autofixed AST)
	const context = new CompilerContext({
		ref: req.ref,
		sourceText: req.sourceText,
		ast,
		project,
		options,
		origin: {
			category: "check",
		},
		frozen: true,
	});
	const newAst = context.reduceRoot(ast, lintTransforms);
	if (ast !== newAst) {
		throw new Error("Expected the same ast. `frozen: true` is broken");
	}

	const diagnostics = context.diagnostics.getDiagnostics();
	const result: LintResult = {
		suppressions: context.suppressions,
		diagnostics: [...ast.diagnostics, ...diagnostics],
		src: formattedCode,
	};
	lintCache.set(query, result);
	return result;
}
