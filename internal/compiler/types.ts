/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnalyzeDependencyResult, FileReference} from "@internal/core";
import {Path} from "@internal/compiler";
import {AnyRoot} from "@internal/ast";
import {ProjectConfig} from "@internal/project";
import {EnterSignal, ExitSignal} from "./signals";
import CompilerContext from "./lib/CompilerContext";
import {AbsoluteFilePath} from "@internal/path";
import {SourceMap} from "@internal/codec-source-map";
import {Dict, UnknownObject} from "@internal/typescript-helpers";
import {DiagnosticCategory} from "@internal/diagnostics";
import {VisitorStateEnter, VisitorStateExit} from "./lib/VisitorState";

//
export type TransformStageName = "pre" | "compile" | "compileForBundle";

export type TransformStageFactory = (
	projectConfig: ProjectConfig,
	options: Object,
) => Transforms;

export type TransformStageFactories = {
	[key in TransformStageName]: TransformStageFactory
};

//
export type Transform =
	| AnyVisitor
	| ((context: CompilerContext) => Visitor<UnknownObject>);

export type Transforms = Array<Transform>;

export interface Visitor<State extends UnknownObject> {
	name: string;
	enter?: (path: Path, state: VisitorStateEnter<State>) => EnterSignal;
	exit?: (path: Path, state: VisitorStateExit<State>) => ExitSignal;
}

// rome-ignore lint/ts/noExplicitAny
export type AnyVisitor = Visitor<any>;

export type AnyVisitors = Array<AnyVisitor>;

export type CompileRequest = TransformRequest & {
	inputSourceMap?: SourceMap;
};

export type LintRequest = TransformRequest & {
	applySafeFixes: boolean;
};

export type TransformProjectDefinition = {
	config: ProjectConfig;
	directory: undefined | AbsoluteFilePath;
};

export type TransformRequest = {
	ref?: FileReference;
	sourceText: string;
	ast: AnyRoot;
	project: TransformProjectDefinition;
	options: CompilerOptions;
	stage?: TransformStageName;
};

export type BundleCompileResolvedImports = {
	[key: string]: {
		id: string;
		name: string;
	};
};

export type BundleCompileOptions = {
	moduleAll: boolean;
	moduleId: string;
	analyze: AnalyzeDependencyResult;
	relativeSourcesToModuleId: Dict<string>;
	resolvedImports: BundleCompileResolvedImports;
	assetPath: undefined | string;
};

export type LintCompilerOptions = {
	hasDecisions?: boolean;
	globalDecisions?: LintCompilerOptionsDecisions;
	decisionsByPosition?: Dict<LintCompilerOptionsDecisions>;
};

export type LintCompilerOptionsDecisions = Array<LintCompilerOptionsDecision>;

export type LintCompilerOptionsDecisionAction = "suppress" | "fix" | "ignore";

export type LintCompilerOptionsDecision = {
	action: LintCompilerOptionsDecisionAction;
	category: DiagnosticCategory;
	id?: number;
};

export type CompilerOptions = {
	bundle?: BundleCompileOptions;
	lint?: LintCompilerOptions;
};
