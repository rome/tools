/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnalyzeDependencyResult,
	BundlerMode,
	FileReference,
} from '@romejs/core';
import {Path, REDUCE_REMOVE} from '@romejs/js-compiler';
import {AnyNode, Program} from '@romejs/js-ast';
import {ProjectConfig} from '@romejs/project';
import {REDUCE_SKIP_SUBTREE} from './constants';
import CompilerContext from './lib/CompilerContext';
import {AbsoluteFilePath} from '@romejs/path';
import {SourceMap} from '@romejs/codec-source-map';
import {Dict} from '@romejs/typescript-helpers';
import {DiagnosticCategory} from '@romejs/diagnostics';

//
export type TransformStageName = 'pre' | 'compile' | 'compileForBundle';

export type TransformStageFactory = (
	projectConfig: ProjectConfig,
	options: Object,
) => Transforms;

export type TransformStageFactories = {
	[key in TransformStageName]: TransformStageFactory
};

//
export type Transform =
	 | TransformVisitor
	| ((context: CompilerContext) => TransformVisitor);

export type Transforms = Array<Transform>;

export type TransformExitResult =
	 | Array<AnyNode>
	| AnyNode
	| typeof REDUCE_REMOVE;

export type TransformEnterResult =
	 | TransformExitResult
	| typeof REDUCE_SKIP_SUBTREE;

export type TransformVisitor = {
	name: string;
	enter?: (path: Path) => TransformEnterResult;
	exit?: (path: Path) => TransformExitResult;
};

export type TransformVisitors = Array<TransformVisitor>;

export type CompileRequest = TransformRequest & {
	inputSourceMap?: SourceMap;
};

export type LintRequest = TransformRequest & {
	applyFixes: boolean;
};

export type TransformProjectDefinition = {
	config: ProjectConfig;
	folder: undefined | AbsoluteFilePath;
};

export type TransformRequest = {
	ref?: FileReference;
	sourceText: string;
	ast: Program;
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
	mode: BundlerMode;
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

export type LintCompilerOptionsDecisionAction = 'suppress' | 'fix' | 'ignore';

export type LintCompilerOptionsDecision = {
	action: LintCompilerOptionsDecisionAction;
	category: DiagnosticCategory;
	id?: number;
};

export type CompilerOptions = {
	bundle?: BundleCompileOptions;
	lint?: LintCompilerOptions;
};
