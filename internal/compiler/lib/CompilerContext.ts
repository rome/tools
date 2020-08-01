/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyComment,
	AnyNode,
	AnyNodes,
	AnyRoot,
	ConstJSSourceType,
} from "@internal/ast";
import {
	SourceLocation,
	extractSourceLocationRangeFromNodes,
} from "@internal/parser-core";
import {
	AnyVisitors,
	CompilerOptions,
	PathOptions,
	Transforms,
} from "@internal/compiler";
import {
	Diagnostic,
	DiagnosticCategory,
	DiagnosticDescription,
	DiagnosticLanguage,
	DiagnosticLocation,
	DiagnosticOrigin,
	DiagnosticSuppressions,
	DiagnosticsProcessor,
	descriptions,
} from "@internal/diagnostics";
import Record from "./Record";
import {RootScope} from "../scope/Scope";
import {reduceNode} from "../methods/reduce";
import {UnknownFilePath, createUnknownFilePath} from "@internal/path";
import {
	AnyVisitor,
	LintCompilerOptionsDecision,
	TransformProjectDefinition,
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
import {Markup, markup} from "@internal/markup";
import cleanTransform from "../transforms/cleanTransform";
import {assertSingleNode} from "@internal/js-ast-utils";
import VisitorState, {AnyVisitorState} from "./VisitorState";
import {UnknownObject} from "@internal/typescript-helpers";

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
export type ContextDiagnostic = Omit<Diagnostic, "location" | "description"> & {
	marker?: Markup;
};

type DiagnosticTarget =
	| undefined
	| {
			loc?: SourceLocation;
		}
	| Array<{
			loc?: SourceLocation;
		}>;

export default class CompilerContext {
	constructor(arg: ContextArg) {
		const {
			ast,
			origin,
			ref,
			frozen = false,
			options = {},
			project = {
				directory: undefined,
				config: createDefaultProjectConfig(),
			},
			suppressions = [],
		} = arg;

		this.records = [];

		this.ast = ast;
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

		this.reducedRoot = false;
		this.suppressions = suppressions;
		this.visitSuppressions = arg.suppressions === undefined;

		this.visitorStates = new Map();
	}

	visitorStates: Map<AnyVisitor, AnyVisitorState>;
	displayFilename: string;
	filename: string;
	mtime: undefined | number;
	path: UnknownFilePath;
	project: TransformProjectDefinition;
	language: DiagnosticLanguage;
	sourceTypeJS: undefined | ConstJSSourceType;
	reducedRoot: boolean;
	rootScope: RootScope;
	ast: AnyRoot;

	comments: CommentsConsumer;
	cacheDependencies: Set<string>;
	records: Array<Record>;

	diagnostics: DiagnosticsProcessor;
	suppressions: DiagnosticSuppressions;
	visitSuppressions: boolean;

	frozen: boolean;
	origin: undefined | DiagnosticOrigin;
	options: CompilerOptions;

	getVisitorState<State extends UnknownObject>(
		visitor: Visitor<State>,
	): VisitorState<State> {
		let state = this.visitorStates.get(visitor);
		if (state === undefined) {
			state = new VisitorState();
			this.visitorStates.set(visitor, state);
		}
		return (state as VisitorState<State>);
	}

	async normalizeTransforms(transforms: Transforms): Promise<AnyVisitors> {
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

	checkOverlappingSuppressions() {
		// Check for overlapping suppressions
		const nonOverlapSuppressions = new Map();
		for (const suppression of this.suppressions) {
			if (nonOverlapSuppressions.has(suppression.category)) {
				const previousSuppression = nonOverlapSuppressions.get(
					suppression.category,
				);
				const currentSuppression = suppression;
				if (
					currentSuppression.startLine > previousSuppression.startLine &&
					currentSuppression.endLine <= previousSuppression.endLine
				) {
					this.diagnostics.addDiagnostic({
						description: descriptions.SUPPRESSIONS.OVERLAP(suppression.category),
						location: suppression.commentLocation,
					});
				} else {
					// Replace suppression to compare to later suppressions
					nonOverlapSuppressions.set(suppression.category, suppression);
				}
			} else {
				nonOverlapSuppressions.set(suppression.category, suppression);
			}
		}
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
		visitors: AnyVisitor | AnyVisitors,
		pathOpts?: PathOptions,
	): AnyRoot {
		if (this.reducedRoot) {
			throw new Error("reduceRoot has already been called");
		}

		const node = assertSingleNode(
			reduceNode(
				this.ast,
				[
					createSuppressionsVisitor(),
					...helperVisitors,
					cleanTransform,
					...(Array.isArray(visitors) ? visitors : [visitors]),
				],
				this,
				pathOpts,
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

	reduce(
		ast: AnyNode,
		visitors: AnyVisitor | AnyVisitors,
		pathOpts?: PathOptions,
	): AnyNodes {
		return reduceNode(ast, visitors, this, pathOpts);
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
					noun: markup`Add suppression comment`,
					shortcut: "s",
					instruction: markup`To suppress this error run`,
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
					noun: markup`Add suppression comments for ALL files with this category`,
					instruction: markup`To add suppression comments for ALL files with this category run`,
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
