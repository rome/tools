/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostic, DiagnosticSuppression} from "@internal/diagnostics";
import {AnyVisitor, LintRequest} from "../types";
import {Cache, CompilerContext, LintRuleName} from "@internal/compiler";
import {formatAST} from "@internal/formatter";
import {addSuppressions} from "./suppressions";
import {lintTransforms} from "./rules/index";
import {ProjectConfig} from "@internal/project";

export type LintResult = {
	diagnostics: Diagnostic[];
	suppressions: DiagnosticSuppression[];
	formatted: string;
};

const ruleVisitorCache: WeakMap<ProjectConfig, AnyVisitor[]> = new WeakMap();

const allVisitors = Array.from(lintTransforms.values());

function getVisitors(
	config: ProjectConfig,
	applyLintCategories?: LintRuleName[],
): AnyVisitor[] {
	const {disabledRules} = config.lint;

	if (applyLintCategories && applyLintCategories.length > 0) {
		return applyLintCategories.map((ruleName) => {
			return lintTransforms.get(ruleName);
		}).filter(Boolean) as AnyVisitor[];
	}

	// Fast path
	if (disabledRules.length === 0) {
		return allVisitors;
	}

	const cached = ruleVisitorCache.get(config);
	if (cached !== undefined) {
		return cached;
	}

	const visitors: AnyVisitor[] = [];
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
	const {
		ast,
		applySafeFixes,
		options,
		suppressionExplanation,
		applyLintCategories,
	} = req;
	const project = CompilerContext.normalizeProject(req.project);

	const query = Cache.buildQuery(req, {applySafeFixes, suppressionExplanation});
	const cached = lintCache.get(query);
	if (cached) {
		return cached;
	}

	const shouldLint = project.config.lint.enabled;
	const shouldFormat = project.config.format.enabled;
	const visitors = getVisitors(project.config, applyLintCategories);

	// Perform fixes
	let formatAst = ast;
	if (shouldFormat && applySafeFixes && shouldLint) {
		const formatContext = new CompilerContext({
			ref: req.ref,
			options,
			ast,
			project,
			frozen: false,
			origin: {
				entity: "compiler.lint",
			},
		});

		formatAst = formatContext.reduceRoot(visitors);
		formatAst = addSuppressions(
			formatContext,
			formatAst,
			suppressionExplanation,
		);
	}

	let formattedCode = req.sourceText;

	if (shouldFormat) {
		formattedCode = formatAST(
			formatAst,
			{
				projectConfig: project.config,
			},
		).code;
	}

	const context = new CompilerContext({
		ref: req.ref,
		ast,
		project,
		options,
		origin: {
			entity: "compiler.lint",
		},
		frozen: true,
	});

	if (shouldLint) {
		// Run lints (could be with the autofixed AST)
		const newAst = context.reduceRoot(visitors);
		if (ast !== newAst) {
			throw new Error("Expected the same ast. `frozen: true` is broken");
		}
	}

	const result: LintResult = {
		suppressions: context.suppressions,
		diagnostics: [...ast.diagnostics, ...context.diagnostics.getDiagnostics()],
		formatted: formattedCode,
	};
	lintCache.set(query, result);
	return result;
}
