/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticSuppressions, Diagnostics} from "@romejs/diagnostics";
import {LintRequest} from "../types";
import {Cache, CompilerContext} from "@romejs/compiler";
import {formatJS} from "@romejs/formatter";
import {addSuppressions} from "./suppressions";
import {lintTransforms} from "./rules/index";

export type LintResult = {
	diagnostics: Diagnostics;
	suppressions: DiagnosticSuppressions;
	src: string;
};

const lintCache: Cache<LintResult> = new Cache();

export default async function lint(req: LintRequest): Promise<LintResult> {
	const {ast, sourceText, project, applyFixes, options} = req;

	const query = Cache.buildQuery(req, {applyFixes});
	const cached = lintCache.get(query);
	if (cached) {
		return cached;
	}

	// Perform autofixes
	const formatContext = new CompilerContext({
		ref: req.ref,
		sourceText: req.sourceText,
		options,
		ast,
		project,
		frozen: false,
		origin: {
			category: "lint",
		},
	});

	let formatAst = ast;
	if (applyFixes) {
		formatAst = formatContext.reduceRoot(ast, lintTransforms);
		formatAst = addSuppressions(formatContext, formatAst);
	}
	const formattedCode = formatJS(
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
			category: "lint",
		},
		frozen: true,
	});
	context.reduceRoot(ast, lintTransforms);

	const diagnostics = context.diagnostics.getDiagnostics();
	const result: LintResult = {
		suppressions: context.suppressions,
		diagnostics: [...ast.diagnostics, ...diagnostics],
		src: formattedCode,
	};
	lintCache.set(query, result);
	return result;
}
