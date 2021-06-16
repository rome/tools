/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnalyzeDependencyResult, FileReference} from "@internal/core";
import {CompilerPath, LintRuleName} from "@internal/compiler";
import {AnyRoot} from "@internal/ast";
import {ProjectConfig} from "@internal/project";
import {EnterSignal, ExitSignal} from "./signals";
import CompilerContext from "./lib/CompilerContext";
import {
	AbsoluteFilePath,
	Path,
	RelativePath,
	UIDPath,
	UIDPathMap,
} from "@internal/path";
import {SourceMap} from "@internal/codec-source-map";
import {Dict, UnknownObject} from "@internal/typescript-helpers";
import {DiagnosticCategory} from "@internal/diagnostics";
import {VisitorStateEnter, VisitorStateExit} from "./lib/VisitorState";

export type CompilerProject = {
	config: ProjectConfig;
	directory?: undefined | AbsoluteFilePath;
};

//
export type TransformStageName = "pre" | "compile" | "compileForBundle";

export type TransformStageFactory = (
	projectConfig: ProjectConfig,
	options: Object,
) => Transform[];

export type TransformStageFactories = {
	[key in TransformStageName]: TransformStageFactory
};

//
export type Transform =
	| AnyVisitor
	| ((context: CompilerContext) => Visitor<UnknownObject>);

export interface Visitor<State extends UnknownObject> {
	name: string;
	enter?: (path: CompilerPath, state: VisitorStateEnter<State>) => EnterSignal;
	exit?: (path: CompilerPath, state: VisitorStateExit<State>) => ExitSignal;
}

export interface TypedVisitor<
	State extends UnknownObject,
	PathType extends CompilerPath
> {
	name: string;
	enter?: (path: PathType, state: VisitorStateEnter<State>) => EnterSignal;
	exit?: (path: PathType, state: VisitorStateExit<State>) => ExitSignal;
}

// rome-ignore lint/ts/noExplicitAny: future cleanup
export interface LintVisitor<State extends UnknownObject = any> {
	name: LintRuleName;
	enter?: (path: CompilerPath, state: VisitorStateEnter<State>) => EnterSignal;
	exit?: (path: CompilerPath, state: VisitorStateExit<State>) => ExitSignal;
}

// rome-ignore lint/ts/noExplicitAny: future cleanup
export type AnyVisitor = Visitor<any>;

export type CompileRequest = TransformRequest & {
	inputSourceMap?: SourceMap;
};

export type LintRequest = TransformRequest & {
	applySafeFixes: boolean;
	suppressionExplanation?: string;
	/**
	 * An array of specific lint name rules to apply
	 */
	applyLintCategories?: LintRuleName[];
};

export type TransformRequest = {
	sourceText: string;
	ast: AnyRoot;
	options: CompilerOptions;
	ref?: FileReference;
	project?: CompilerProject;
	stage?: TransformStageName;
};

export type BundleCompileResolvedImports = UIDPathMap<Map<
	string,
	{
		id: UIDPath;
		name: string;
	}
>>;

export type BundleCompileOptions = {
	moduleAll: boolean;
	moduleId: UIDPath;
	analyze: AnalyzeDependencyResult;
	relativeSourcesToModuleId: Map<string, UIDPath>;
	resolvedImports: BundleCompileResolvedImports;
	assetPath: undefined | Path;
	__filename: RelativePath;
};

export type LintCompilerOptions = {
	hasDecisions?: boolean;
	globalDecisions?: LintCompilerOptionsDecisions;
	decisionsByPosition?: Dict<LintCompilerOptionsDecisions>;
};

export type LintCompilerOptionsDecisions = LintCompilerOptionsDecision[];

export type LintCompilerOptionsDecisionAction = "suppress" | "fix" | "ignore";

export type LintCompilerOptionsDecision = {
	action: LintCompilerOptionsDecisionAction;
	category: DiagnosticCategory;
	categoryValue: undefined | string;
	id?: number;
};

export type CompilerOptions = {
	bundle?: BundleCompileOptions;
	lint?: LintCompilerOptions;
	target?: string;
};
