/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticSuppressions, Diagnostics} from "@internal/diagnostics";
import {AnyVisitor, LintRequest} from "../types";
import {Cache, CompilerContext} from "@internal/compiler";
import {formatAST} from "@internal/formatter";
import {addSuppressions} from "./suppressions";
import {lintTransforms} from "./rules/index";
import {ProjectConfig} from "@internal/project";

export type LintResult = {
	diagnostics: Diagnostics;
	suppressions: DiagnosticSuppressions;
	src: string;
};

const ruleVisitorCache: WeakMap<ProjectConfig, Array<AnyVisitor>> = new WeakMap();

const allVisitors = Array.from(lintTransforms.values());

function getVisitors(config: ProjectConfig): Array<AnyVisitor> {
	const {disabledRules} = config.lint;

	// Fast path
	if (disabledRules.length === 0) {
		return allVisitors;
	}

	const cached = ruleVisitorCache.get(config);
	if (cached !== undefined) {
		return cached;
	}

	const visitors: Array<AnyVisitor> = [];
	ruleVisitorCache.set(config, visitors);

	for (const [ruleName, visitor] of lintTransforms) {
		if (!disabledRules.includes(ruleName)) {
			visitors.push(visitor);
		}
	}

	return visitors;
}

const lintCache: Cache<LintResult> = new Cache();

export default async function lint(req: LintRequest): Promise<LintResult> {
	const {ast, project, applySafeFixes, options, suppressionExplanation} = req;

	const query = Cache.buildQuery(req, {applySafeFixes, suppressionExplanation});
	const cached = lintCache.get(query);
	if (cached) {
		return cached;
	}

	const visitors = getVisitors(project.config);

	// Perform fixes
	let formatAst = ast;
	if (project.config.format.enabled && applySafeFixes) {
		const formatContext = new CompilerContext({
			ref: req.ref,
			options,
			ast,
			project,
			frozen: false,
			origin: {
				category: "check",
			},
		});

		formatAst = formatContext.reduceRoot(visitors);
		formatAst = addSuppressions(
			formatContext,
			formatAst,
			suppressionExplanation,
		);
	}
	const formattedCode = formatAST(formatAst).code;

	// Run lints (could be with the autofixed AST)
	const context = new CompilerContext({
		ref: req.ref,
		ast,
		project,
		options,
		origin: {
			category: "check",
		},
		frozen: true,
	});
	const newAst = context.reduceRoot(visitors);
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
