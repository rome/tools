/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {BundlerMode, AnalyzeDependencyResult} from '@romejs/core';
import {Path, REDUCE_REMOVE} from '@romejs/js-compiler';
import {AnyNode, Program} from '@romejs/js-ast';
import {ProjectConfig} from '@romejs/project';
import {REDUCE_SKIP_SUBTREE} from './constants';
import Context from './lib/Context';
import {AbsoluteFilePath} from '@romejs/path';
import {SourceMap} from '@romejs/codec-source-map';
import {Dict} from '@romejs/typescript-helpers';

//

export type TransformStageName = 'pre' | 'compile' | 'compileForBundle';

export type TransformStageFactory = (
  projectConfig: ProjectConfig,
  options: Object,
) => Transforms;

export type TransformStageFactories = {
  [key in TransformStageName]: TransformStageFactory;
};

//

export type Transform =
  | TransformVisitor
  | ((context: Context) => TransformVisitor);

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

//

export type CompileRequest = TransformRequest & {
  inputSourceMap?: SourceMap;
};

export type TransformProjectDefinition = {
  config: ProjectConfig;
  folder: undefined | AbsoluteFilePath;
};

export type TransformRequest = {
  ast: Program;
  sourceText: string;
  project: TransformProjectDefinition;
  options: CompilerOptions;
  stage?: TransformStageName;
};

export type BundleCompileResolvedImports = {
  [key: string]: {id: string; name: string};
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

export type CompilerOptions = {
  bundle?: BundleCompileOptions;
};
