/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostic, DiagnosticSuppression} from "@internal/diagnostics";
import {LintRequest, LintVisitor} from "../types";
import {Cache, CompilerContext, LintRuleName} from "@internal/compiler";
import {formatAST} from "@internal/formatter";
import {addSuppressions} from "./suppressions";
import {lintTransforms} from "./rules/index";
import {ProjectConfig} from "@internal/project";
import {
	LintCategories,
	RuleNames,
} from "@internal/compiler/lint/rules/categories";

export type LintResult = {
	diagnostics: Diagnostic[];
	suppressions: DiagnosticSuppression[];
	formatted: string;
};

const ruleVisitorCache: WeakMap<ProjectConfig, LintVisitor[]> = new WeakMap();

const allVisitors: LintVisitor[] = [];
const recommendedVisitors: LintVisitor[] = [];

for (const [, payload] of lintTransforms) {
	for (const visitors of payload.values()) {
		allVisitors.push(visitors.visitor);
		if (visitors.recommended) {
			recommendedVisitors.push(visitors.visitor);
		}
	}
}

export function getVisitors(
	config: ProjectConfig,
	applyLintRules?: LintRuleName[],
): LintVisitor[] {
	// the option has priority above all
	if (applyLintRules && applyLintRules.length > 0) {
		for (const ruleToApply of applyLintRules) {
			const [categoryToApply, ruleNameToApply] = ruleToApply.split("/");
			const visitorPayload = lintTransforms.get(
				categoryToApply as LintCategories,
			)?.get(ruleNameToApply as RuleNames);
			if (visitorPayload) {
				return [visitorPayload.visitor];
			}
		}
	}
	const {rules} = config.lint;

	// if no configuration was passed, let's just return all the rome visitors regardless
	if (!rules) {
		return allVisitors;
	}

	const cached = ruleVisitorCache.get(config);
	if (cached !== undefined) {
		return cached;
	}
	const computedVisitors: LintVisitor[] = [];
	ruleVisitorCache.set(config, computedVisitors);

	if (rules) {
		if (rules.recommended === true) {
			return recommendedVisitors;
		}

		for (const [category, currentRules] of lintTransforms) {
			// select the configuration of the current category, e.g. a11y, js, ts, etc.
			const categoryConfig = rules[category];
			if (categoryConfig && typeof categoryConfig !== "boolean") {
				// if it is recommended, let's push all the visitors that belong to that category
				if (categoryConfig.recommended) {
					computedVisitors.push(
						...Array.from(currentRules.values()).filter((c) => c.recommended).map((
							c,
						) => c.visitor),
					);
				} else {
					for (const [ruleName, active] of categoryConfig) {
						if (active) {
							const payload = currentRules.get(ruleName);
							if (payload) {
								computedVisitors.push(payload.visitor);
							}
						}
					}
				}
			} else if (categoryConfig === true) {
				computedVisitors.push(
					...Array.from(currentRules.values()).map((c) => c.visitor),
				);
			}
		}
	}

	return computedVisitors;
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
