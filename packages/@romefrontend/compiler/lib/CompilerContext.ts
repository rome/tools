/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyComment,
	AnyNode,
	AnyRoot,
	ConstJSSourceType,
} from "@romefrontend/ast";
import {
	SourceLocation,
	extractSourceLocationRangeFromNodes,
} from "@romefrontend/parser-core";
import {
	CompilerOptions,
	PathOptions,
	TransformExitResult,
	TransformVisitors,
	Transforms,
} from "@romefrontend/compiler";
import {
	Diagnostic,
	DiagnosticCategory,
	DiagnosticDescription,
	DiagnosticLanguage,
	DiagnosticLocation,
	DiagnosticOrigin,
	DiagnosticSuppressions,
	DiagnosticsProcessor,
} from "@romefrontend/diagnostics";
import Record from "./Record";
import {RootScope} from "../scope/Scope";
import reduce from "../methods/reduce";
import {UnknownFilePath, createUnknownFilePath} from "@romefrontend/path";
import {
	LintCompilerOptionsDecision,
	TransformProjectDefinition,
	TransformVisitor,
} from "../types";
import {
	extractSuppressionsFromProgram,
	matchesSuppression,
} from "../suppressions";
import CommentsConsumer from "@romefrontend/js-parser/CommentsConsumer";
import {hookVisitors} from "../transforms";
import stringDiff from "@romefrontend/string-diff";
import {formatAST} from "@romefrontend/formatter";
import {REDUCE_REMOVE} from "../constants";
import {FileReference} from "@romefrontend/core";
import {createDefaultProjectConfig} from "@romefrontend/project";
import {
	buildLintDecisionAdviceAction,
	buildLintDecisionGlobalString,
	buildLintDecisionString,
	deriveDecisionPositionKey,
} from "../lint/decisions";
import {isRoot} from "@romefrontend/ast-utils";
import {inferDiagnosticLanguageFromRootAST} from "@romefrontend/cli-diagnostics/utils";

export type ContextArg = {
	ast: AnyRoot;
	suppressions?: DiagnosticSuppressions;
	ref?: FileReference;
	project?: TransformProjectDefinition;
	frozen?: boolean;
	options?: CompilerOptions;
	origin?: DiagnosticOrigin;
};

type AddDiagnosticResult = {
	loc: undefined | DiagnosticLocation;
	diagnostic: undefined | Diagnostic;
	suppressed: boolean;
};

// We only want a Context to create diagnostics that belong to itself
type ContextDiagnostic = Omit<Diagnostic, "location" | "description"> & {
	marker?: string;
};

type DiagnosticTarget =
	| undefined
	| {
			loc?: SourceLocation;
		}
	| Array<{
			loc?: SourceLocation;
		}>;

function getFormattedCodeFromExitResult(result: TransformExitResult): string {
	if (Array.isArray(result)) {
		return result.map((node) => {
			return formatAST(node).code;
		}).filter((str) => str !== "").join("\n");
	} else if (result === REDUCE_REMOVE) {
		return "";
	} else {
		return formatAST(result).code;
	}
}

export default class CompilerContext {
	constructor(arg: ContextArg) {
		const {
			ast,
			origin,
			ref,
			frozen = false,
			options = {},
			project = {
				folder: undefined,
				config: createDefaultProjectConfig(),
			},
			suppressions,
		} = arg;

		this.records = [];

		this.path = createUnknownFilePath(ast.filename);
		this.filename = ast.filename;
		this.displayFilename =
			ref === undefined ? ast.filename : ref.relative.join();
		this.frozen = frozen;
		this.mtime = ast.mtime;
		this.project = project;
		this.options = options;
		this.origin = origin;
		this.cacheDependencies = new Set();
		this.language = inferDiagnosticLanguageFromRootAST(ast);
		this.sourceTypeJS = ast.type === "JSRoot" ? ast.sourceType : undefined;
		this.rootScope = new RootScope(this, ast);

		this.comments = new CommentsConsumer(ast.comments);
		this.diagnostics = new DiagnosticsProcessor();

		if (suppressions === undefined) {
			const {suppressions, diagnostics} = extractSuppressionsFromProgram(
				this,
				ast,
			);
			this.suppressions = suppressions;
			this.diagnostics.addDiagnostics(diagnostics);
		} else {
			this.suppressions = suppressions;
		}
	}

	displayFilename: string;
	filename: string;
	path: UnknownFilePath;
	project: TransformProjectDefinition;

	language: DiagnosticLanguage;
	sourceTypeJS: undefined | ConstJSSourceType;

	comments: CommentsConsumer;
	cacheDependencies: Set<string>;
	records: Array<Record>;
	diagnostics: DiagnosticsProcessor;
	suppressions: DiagnosticSuppressions;
	frozen: boolean;
	rootScope: RootScope;
	mtime: undefined | number;
	origin: undefined | DiagnosticOrigin;
	options: CompilerOptions;

	async normalizeTransforms(transforms: Transforms): Promise<TransformVisitors> {
		return Promise.all(
			transforms.map(async (visitor) => {
				if (typeof visitor === "function") {
					return await visitor(this);
				} else {
					return visitor;
				}
			}),
		);
	}

	getComments(ids: undefined | Array<string>): Array<AnyComment> {
		return this.comments.getCommentsFromIds(ids);
	}

	hasLocSuppression(
		loc: undefined | DiagnosticLocation,
		category: DiagnosticCategory,
	): boolean {
		if (loc === undefined) {
			return false;
		}

		for (const suppression of this.suppressions) {
			if (
				suppression.category === category &&
				matchesSuppression(loc, suppression)
			) {
				return true;
			}
		}

		return false;
	}

	getCacheDependencies(): Array<string> {
		return Array.from(this.cacheDependencies);
	}

	addCacheDependency(filename: string) {
		this.cacheDependencies.add(filename);
	}

	reduceRoot(
		ast: AnyRoot,
		visitors: TransformVisitor | TransformVisitors,
		pathOpts?: PathOptions,
	): AnyRoot {
		const node = reduce(
			ast,
			[...hookVisitors, ...(Array.isArray(visitors) ? visitors : [visitors])],
			this,
			pathOpts,
		);
		if (!isRoot(node)) {
			throw new Error("Expected root to be returned from reduce");
		}
		return node;
	}

	reduce(
		ast: AnyNode,
		visitors: TransformVisitor | TransformVisitors,
		pathOpts?: PathOptions,
	): TransformExitResult {
		return reduce(
			ast,
			Array.isArray(visitors) ? visitors : [visitors],
			this,
			pathOpts,
		);
	}

	record(record: Record) {
		this.records.push(record);
	}

	hasLintDecisions(): boolean {
		const {lint} = this.options;
		return lint !== undefined && lint.hasDecisions === true;
	}

	getLintDecisions(key: undefined | string): Array<LintCompilerOptionsDecision> {
		const {lint} = this.options;
		if (lint === undefined) {
			return [];
		}

		const {globalDecisions = []} = lint;

		if (key === undefined) {
			return globalDecisions;
		}

		const {decisionsByPosition} = lint;
		if (decisionsByPosition === undefined) {
			return globalDecisions;
		}

		return [...globalDecisions, ...(decisionsByPosition[key] || [])];
	}

	addFixableDiagnostic<Old extends AnyNode, New extends TransformExitResult>(
		nodes: {
			target?: AnyNode | Array<AnyNode>;
			old: Old;
			fixed?: New;
			suggestions?: Array<{
				description: string;
				title: string;
				fixed: New;
			}>;
		},
		description: DiagnosticDescription,
		diag: ContextDiagnostic = {},
	): TransformExitResult {
		const {old, fixed: defaultFixed, suggestions} = nodes;
		const target = nodes.target === undefined ? nodes.old : nodes.target;

		const {category} = description;
		const advice = [...description.advice];
		const loc = this.getLoc(target);

		let fixed: undefined | New = defaultFixed;

		// Add recommended fix
		if (defaultFixed !== undefined) {
			advice.push({
				type: "log",
				category: "info",
				text: "Recommended fix",
			});

			advice.push({
				type: "diff",
				language: this.language,
				diff: stringDiff(
					getFormattedCodeFromExitResult(old),
					getFormattedCodeFromExitResult(defaultFixed),
				),
			});

			if (loc === undefined) {
				advice.push({
					type: "log",
					category: "error",
					text: "Unable to find target location",
				});
			} else {
				advice.push(
					buildLintDecisionAdviceAction({
						filename: this.displayFilename,
						decision: buildLintDecisionString({
							action: "fix",
							filename: this.displayFilename,
							category,
							start: loc.start,
						}),
						shortcut: "f",
						noun: "Apply fix",
						instruction: "To apply this fix run",
					}),
				);

				advice.push(
					buildLintDecisionAdviceAction({
						extra: true,
						noun: "Apply fix for ALL files with this category",
						instruction: "To apply fix for ALL files with this category run",
						decision: buildLintDecisionGlobalString("fix", category),
					}),
				);
			}
		}

		if (suggestions !== undefined) {
			// If we have lint decisions then find the fix that corresponds with this suggestion
			if (this.hasLintDecisions()) {
				const decisions = this.getLintDecisions(
					deriveDecisionPositionKey("fix", loc),
				);
				for (const decision of decisions) {
					if (
						decision.category === category &&
						decision.action === "fix" &&
						decision.id !== undefined
					) {
						const suggestion = suggestions[decision.id];
						if (suggestion !== undefined) {
							fixed = suggestion.fixed;
						}
					}
				}
			}

			// Add advice suggestions
			let index = 0;
			for (const suggestion of suggestions) {
				const num = index + 1;

				const titlePrefix =
					suggestions.length === 1 ? "Suggested fix" : `Suggested fix #${num}`;
				advice.push({
					type: "log",
					category: "none",
					text: `<emphasis>${titlePrefix}:</emphasis> ${suggestion.title}`,
				});

				advice.push({
					type: "diff",
					language: this.language,
					diff: stringDiff(
						getFormattedCodeFromExitResult(old),
						getFormattedCodeFromExitResult(suggestion.fixed),
					),
				});

				advice.push({
					type: "log",
					category: "info",
					text: suggestion.description,
				});

				if (loc === undefined) {
					advice.push({
						type: "log",
						category: "error",
						text: "Unable to find target location",
					});
				} else {
					advice.push(
						buildLintDecisionAdviceAction({
							noun: suggestions.length === 1
								? "Apply suggested fix"
								: `Apply suggested fix "${suggestion.title}"`,
							shortcut: String(num),
							instruction: "To apply this fix run",
							filename: this.displayFilename,
							decision: buildLintDecisionString({
								filename: this.displayFilename,
								action: "fix",
								category,
								start: loc.start,
								id: index,
							}),
						}),
					);
				}

				index++;
			}
		}

		const {suppressed} = this.addLocDiagnostic(
			loc,
			{
				...description,
				advice,
			},
			{
				...diag,
				fixable: true,
			},
		);

		if (suppressed || fixed === undefined) {
			return old;
		}

		return fixed;
	}

	addLocDiagnostic(
		loc: undefined | DiagnosticLocation,
		description: DiagnosticDescription,
		contextDiag: ContextDiagnostic = {},
	): AddDiagnosticResult {
		let origins: Array<DiagnosticOrigin> = [];
		if (this.origin !== undefined) {
			origins.push(this.origin);
		}
		if (contextDiag.origins !== undefined) {
			origins = origins.concat(contextDiag.origins);
		}

		if (loc !== undefined && loc.filename !== this.filename) {
			throw new Error(
				`Trying to add a location from ${loc.filename} on a Context from ${this.path}`,
			);
		}

		const {category, advice = []} = description;
		if (loc !== undefined && loc.start !== undefined) {
			advice.push(
				buildLintDecisionAdviceAction({
					noun: "Add suppression comment",
					shortcut: "s",
					instruction: "To suppress this error run",
					filename: this.displayFilename,
					decision: buildLintDecisionString({
						filename: this.displayFilename,
						action: "suppress",
						category,
						start: loc.start,
					}),
				}),
			);

			advice.push(
				buildLintDecisionAdviceAction({
					extra: true,
					noun: "Add suppression comments for ALL files with this category",
					instruction: "To add suppression comments for ALL files with this category run",
					decision: buildLintDecisionGlobalString("suppress", category),
				}),
			);
		}

		const {marker, ...diag} = contextDiag;
		const diagnostic = this.diagnostics.addDiagnostic({
			...diag,
			description: {
				...description,
				advice,
			},
			location: {
				marker,
				mtime: this.mtime,
				filename: this.filename,
				start: loc === undefined ? undefined : loc.start,
				end: loc === undefined ? undefined : loc.end,
				sourceTypeJS: this.sourceTypeJS,
				language: this.language,
			},
			origins,
		});

		let suppressed = this.hasLocSuppression(loc, description.category);

		// If we've been passed lint decisions then consider it suppressed unless we have been specifically told to fix it
		const diagCategory = description.category;
		if (this.hasLintDecisions()) {
			suppressed = true;

			const decisions = this.getLintDecisions(
				deriveDecisionPositionKey("fix", loc),
			);
			for (const {category, action} of decisions) {
				if (category === diagCategory && action === "fix") {
					suppressed = false;
				}
			}
		}

		return {
			loc,
			diagnostic,
			suppressed,
		};
	}

	getLoc(node: DiagnosticTarget): undefined | SourceLocation {
		if (node === undefined) {
			return undefined;
		}

		if (Array.isArray(node)) {
			return extractSourceLocationRangeFromNodes(node);
		} else {
			return node.loc;
		}
	}

	addNodeDiagnostic(
		node: DiagnosticTarget,
		description: DiagnosticDescription,
		diag: ContextDiagnostic = {},
	): AddDiagnosticResult {
		return this.addLocDiagnostic(this.getLoc(node), description, diag);
	}
}
