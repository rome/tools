/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, AnyNodes, AnyRoot, ConstJSSourceType} from "@internal/ast";
import {
	SourceLocation,
	extractSourceLocationRangeFromNodes,
} from "@internal/parser-core";
import {
	CompilerOptions,
	CompilerPathOptions,
	Transform,
} from "@internal/compiler";
import {
	Diagnostic,
	DiagnosticCategory,
	DiagnosticDescription,
	DiagnosticIntegrity,
	DiagnosticLanguage,
	DiagnosticLocation,
	DiagnosticOrigin,
	DiagnosticSuppression,
	DiagnosticsProcessor,
	descriptions,
	equalCategoryNames,
	formatCategoryDescription,
} from "@internal/diagnostics";
import Record from "./Record";
import {RootScope} from "../scope/Scope";
import {reduceNode} from "../methods/reduce";
import {
	AbsoluteFilePath,
	AbsoluteFilePathSet,
	Path,
	equalPaths,
} from "@internal/path";
import {
	AnyVisitor,
	CompilerProject,
	LintCompilerOptionsDecision,
	Visitor,
} from "../types";
import {createSuppressionsVisitor, matchesSuppression} from "../suppressions";
import {CommentsConsumer} from "@internal/js-parser";
import {helperVisitors} from "../transforms";
import {FileReference} from "@internal/core";
import {createDefaultProjectConfig} from "@internal/project";
import {
	buildLintDecisionAdviceAction,
	buildLintDecisionGlobalString,
	buildLintDecisionString,
	deriveDecisionPositionKey,
} from "../lint/decisions";
import {isRoot} from "@internal/ast-utils";
import {inferDiagnosticLanguageFromRootAST} from "@internal/cli-diagnostics/utils";
import {StaticMarkup, markup} from "@internal/markup";
import cleanTransform from "../transforms/cleanTransform";
import {assertSingleNode} from "@internal/js-ast-utils";
import VisitorState, {AnyVisitorState} from "./VisitorState";
import {UnknownObject} from "@internal/typescript-helpers";
import {ExtendedMap} from "@internal/collections";
import {promiseAllFrom} from "@internal/async";

export type ContextArg = {
	ast: AnyRoot;
	suppressions?: DiagnosticSuppression[];
	ref?: FileReference;
	project?: CompilerProject;
	frozen?: boolean;
	options?: CompilerOptions;
	origin?: DiagnosticOrigin;
};

type AddDiagnosticResult = {
	loc: undefined | DiagnosticLocation;
	suppressed: boolean;
};

// We only want a Context to create diagnostics that belong to itself
export type ContextDiagnostic = Omit<Diagnostic, "location" | "description"> & {
	marker?: StaticMarkup;
};

type DiagnosticTarget =
	| undefined
	| {
			loc?: SourceLocation;
		}
	| ({
			loc?: SourceLocation;
		}[]);

export default class CompilerContext {
	constructor(arg: ContextArg) {
		const {
			ast,
			origin,
			ref,
			frozen = false,
			options = {},
			project,
			suppressions = [],
		} = arg;

		const compilerProject = CompilerContext.normalizeProject(project);

		this.ast = ast;
		this.path = ast.path;

		if (ref === undefined) {
			this.displayPath = ast.path;
		} else if (compilerProject.directory === undefined) {
			this.displayPath = ref.uid;
		} else {
			this.displayPath = compilerProject.directory.relative(ref.real);
		}

		this.frozen = frozen;
		this.integrity = ast.integrity;
		this.project = compilerProject;
		this.options = options;
		this.origin = origin;
		this.cacheDependencies = new AbsoluteFilePathSet();
		this.language = inferDiagnosticLanguageFromRootAST(ast);
		this.sourceTypeJS = ast.type === "JSRoot" ? ast.sourceType : undefined;
		this.rootScope = new RootScope(this, ast);

		this.comments = new CommentsConsumer(ast.comments);
		this.diagnostics = new DiagnosticsProcessor();
		this.records = [];

		this.reducedRoot = false;
		this.suppressions = suppressions;
		this.visitSuppressions = arg.suppressions === undefined;

		this.visitorStates = new ExtendedMap(
			"visitorStates",
			() => new VisitorState(),
		);
	}

	private visitorStates: ExtendedMap<AnyVisitor, AnyVisitorState>;
	private integrity: undefined | DiagnosticIntegrity;
	public displayPath: Path;
	public path: Path;
	public project: CompilerProject;
	public language: DiagnosticLanguage;
	private sourceTypeJS: undefined | ConstJSSourceType;
	private reducedRoot: boolean;
	public rootScope: RootScope;
	private ast: AnyRoot;

	public comments: CommentsConsumer;
	private cacheDependencies: AbsoluteFilePathSet;
	public records: Record[];

	public diagnostics: DiagnosticsProcessor;
	public suppressions: DiagnosticSuppression[];
	private visitSuppressions: boolean;

	public frozen: boolean;
	private origin: undefined | DiagnosticOrigin;
	public options: CompilerOptions;

	public static normalizeProject(
		project: undefined | CompilerProject,
	): CompilerProject {
		if (project === undefined) {
			return {
				config: createDefaultProjectConfig(),
			};
		} else {
			return project;
		}
	}

	public getVisitorState<State extends UnknownObject>(
		visitor: Visitor<State>,
	): VisitorState<State> {
		const state = this.visitorStates.assert(visitor);
		return state as VisitorState<State>;
	}

	public async normalizeTransforms(
		transforms: Transform[],
	): Promise<AnyVisitor[]> {
		return promiseAllFrom(
			transforms,
			async (visitor) => {
				if (typeof visitor === "function") {
					return await visitor(this);
				} else {
					return visitor;
				}
			},
		);
	}

	private checkOverlappingSuppressions() {
		const nonOverlapSuppressions = new Map();

		for (const suppression of this.suppressions) {
			const key = formatCategoryDescription(suppression);

			if (!nonOverlapSuppressions.has(key)) {
				nonOverlapSuppressions.set(key, suppression);
				continue;
			}

			const previousSuppression = nonOverlapSuppressions.get(key);
			const currentSuppression = suppression;
			if (
				currentSuppression.startLine > previousSuppression.startLine &&
				currentSuppression.endLine <= previousSuppression.endLine
			) {
				this.diagnostics.addDiagnostic({
					description: descriptions.SUPPRESSIONS.OVERLAP(key),
					location: suppression.loc,
				});
				continue;
			}

			// Replace suppression to compare to later suppressions
			nonOverlapSuppressions.set(key, suppression);
		}
	}

	public hasLocSuppression(
		loc: undefined | DiagnosticLocation,
		category: DiagnosticCategory,
		categoryValue: undefined | string,
	): boolean {
		if (loc === undefined) {
			return false;
		}

		for (const suppression of this.suppressions) {
			if (matchesSuppression(category, categoryValue, loc, suppression)) {
				return true;
			}
		}

		return false;
	}

	public getCacheDependencies(): AbsoluteFilePath[] {
		return Array.from(this.cacheDependencies);
	}

	public addCacheDependency(path: AbsoluteFilePath) {
		this.cacheDependencies.add(path);
	}

	public reduceRoot(
		visitors: AnyVisitor | AnyVisitor[],
		ast: AnyRoot = this.ast,
	): AnyRoot {
		if (this.reducedRoot) {
			throw new Error("reduceRoot has already been called");
		}

		const node = assertSingleNode(
			reduceNode(
				ast,
				[
					createSuppressionsVisitor(),
					...helperVisitors,
					cleanTransform,
					...(Array.isArray(visitors) ? visitors : [visitors]),
				],
				this,
			),
		);
		if (!isRoot(node)) {
			throw new Error("Expected root to be returned from reduce");
		}

		if (this.visitSuppressions) {
			this.checkOverlappingSuppressions();
		}

		return node;
	}

	public reduce(
		ast: AnyNode,
		visitors: AnyVisitor | AnyVisitor[],
		pathOpts?: CompilerPathOptions,
	): AnyNodes {
		return reduceNode(ast, visitors, this, pathOpts);
	}

	public record(record: Record) {
		this.records.push(record);
	}

	public hasLintDecisions(): boolean {
		const {lint} = this.options;
		return lint?.hasDecisions === true;
	}

	public getLintDecisions(
		key: undefined | string,
		allowGlobalDecisions: boolean = true,
	): LintCompilerOptionsDecision[] {
		const {lint} = this.options;
		if (lint === undefined) {
			return [];
		}

		const {globalDecisions = [], decisionsByPosition} = lint;

		if (key !== undefined && decisionsByPosition !== undefined) {
			const keyDecisions = decisionsByPosition[key];
			if (keyDecisions !== undefined) {
				if (allowGlobalDecisions) {
					return [...globalDecisions, ...keyDecisions];
				} else {
					return keyDecisions;
				}
			}
		}

		if (allowGlobalDecisions) {
			return globalDecisions;
		} else {
			return [];
		}
	}

	public addLocDiagnostic(
		loc: undefined | DiagnosticLocation,
		description: DiagnosticDescription,
		contextDiag: ContextDiagnostic = {},
	): AddDiagnosticResult {
		let origins: DiagnosticOrigin[] = [];
		if (this.origin !== undefined) {
			origins.push(this.origin);
		}
		if (contextDiag.origins !== undefined) {
			origins = origins.concat(contextDiag.origins);
		}

		if (loc !== undefined && !equalPaths(loc.path, this.path)) {
			throw new Error(
				`Trying to add a location from ${loc.path} on a Context from ${this.path}`,
			);
		}

		const {category, categoryValue} = description;
		const verboseAdvice = [...(description.verboseAdvice ?? [])];
		if (loc?.start !== undefined) {
			verboseAdvice.push(
				buildLintDecisionAdviceAction({
					description: markup`Add suppression comment`,
					shortcut: "s",
					path: this.displayPath,
					decision: buildLintDecisionString({
						path: this.displayPath,
						action: "suppress",
						category,
						categoryValue,
						start: loc.start,
					}),
				}),
			);

			verboseAdvice.push(
				buildLintDecisionAdviceAction({
					secondary: true,
					description: markup`Add suppression comments for ALL files with this category`,
					decision: buildLintDecisionGlobalString(
						"suppress",
						category,
						categoryValue,
					),
				}),
			);
		}

		let {marker, tags, ...diag} = contextDiag;

		// Only set `fixable` if formatting is enabled
		if (tags?.fixable) {
			tags = {
				...tags,
				fixable: this.project.config.format.enabled,
			};
		}

		this.diagnostics.addDiagnostic({
			...diag,
			tags,
			description: {
				...description,
				verboseAdvice,
			},
			location: {
				marker,
				integrity: this.integrity,
				path: this.path,
				start: loc === undefined ? undefined : loc.start,
				end: loc === undefined ? undefined : loc.end,
				sourceTypeJS: this.sourceTypeJS,
				language: this.language,
			},
			origins,
		});

		let suppressed = this.hasLocSuppression(loc, category, categoryValue);

		// If we've been passed lint decisions then consider it suppressed unless we have been specifically told to fix it
		const diagCategory = category;
		const diagCategoryValue = categoryValue;
		if (this.hasLintDecisions()) {
			suppressed = true;

			const decisions = this.getLintDecisions(
				deriveDecisionPositionKey("fix", loc),
			);
			for (const {category, categoryValue, action} of decisions) {
				if (
					equalCategoryNames(category, diagCategory) &&
					action === "fix" &&
					categoryValue === diagCategoryValue
				) {
					suppressed = false;
				}
			}
		}

		return {
			loc,
			suppressed,
		};
	}

	public getLoc(node: DiagnosticTarget): undefined | SourceLocation {
		if (node === undefined) {
			return undefined;
		}

		if (Array.isArray(node)) {
			return extractSourceLocationRangeFromNodes(node);
		} else {
			return node.loc;
		}
	}

	public addNodeDiagnostic(
		node: DiagnosticTarget,
		description: DiagnosticDescription,
		diag: ContextDiagnostic = {},
	): AddDiagnosticResult {
		return this.addLocDiagnostic(this.getLoc(node), description, diag);
	}
}
