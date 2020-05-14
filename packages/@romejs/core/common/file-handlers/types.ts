/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {FileReference} from '@romejs/core';
import {WorkerLintOptions, WorkerParseOptions} from '../bridges/WorkerBridge';
import Worker from '../../worker/Worker';
import {DiagnosticSuppressions, Diagnostics} from '@romejs/diagnostics';
import * as compiler from '@romejs/js-compiler';
import {ConstProgramSyntax, ConstSourceType} from '@romejs/js-ast';
import {AnalyzeDependencyResult} from '../types/analyzeDependencies';

export type ExtensionLintInfo = ExtensionHandlerMethodInfo & {
  options: WorkerLintOptions;
};

export type ExtensionLintResult = {
  sourceText: string;
  diagnostics: Diagnostics;
  formatted: string;
  suppressions: DiagnosticSuppressions;
};

export type ExtensionHandlerMethodInfo = {
  parseOptions: WorkerParseOptions;
  file: FileReference;
  project: compiler.TransformProjectDefinition;
  worker: Worker;
};

export type PartialExtensionHandler = {
  sourceType?: ConstSourceType;
  syntax?: Array<ConstProgramSyntax>;
  isAsset?: boolean;
  canHaveScale?: boolean;
  lint?: (info: ExtensionLintInfo) => Promise<ExtensionLintResult>;
  format?: (info: ExtensionHandlerMethodInfo) => Promise<ExtensionLintResult>;
  toJavaScript?: (
    opts: ExtensionHandlerMethodInfo,
  ) => Promise<{
    generated: boolean;
    sourceText: string;
  }>;
  analyzeDependencies?: (
    opts: ExtensionHandlerMethodInfo,
  ) => Promise<AnalyzeDependencyResult>;
};

export type ExtensionHandler = PartialExtensionHandler & {
  ext: string;
};
