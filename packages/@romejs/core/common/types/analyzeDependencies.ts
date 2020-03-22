/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {PartialDiagnostics} from '@romejs/diagnostics';
import {
  ConstExportModuleKind,
  ConstImportModuleKind,
  ConstProgramSyntax,
} from '@romejs/js-ast';
import {SourceLocation} from '@romejs/parser-core';

export type AnalyzeModuleType = 'es' | 'cjs' | 'unknown';

export type AnalyzeDependencyName = {
  name: string;
  kind: ConstImportModuleKind;
  loc?: SourceLocation;
};

export type AnalyzeExportValueType = 'class' | 'function' | 'other';

export type AnalyzeExportLocal = {
  type: 'local';
  loc?: SourceLocation;
  kind: ConstExportModuleKind;
  valueType: AnalyzeExportValueType;
  name: string;
};

export type AnyAnalyzeExport =
  | AnalyzeExportLocal
  | {
    type: 'external';
    kind: ConstImportModuleKind;
    loc?: SourceLocation;
    imported: string;
    exported: string;
    source: string;
  }
  | {
    type: 'externalAll';
    loc?: SourceLocation;
    kind: ConstImportModuleKind;
    source: string;
  };

export type AnalyzeDependency = {
  names: Array<AnalyzeDependencyName>;
  async: boolean;
  kind: ConstImportModuleKind;
  type: AnalyzeModuleType;
  loc?: SourceLocation;
  all: boolean;
  optional: boolean;
  source: string;
};

export type AnalyzeDependencyImportUsageItem = {
  imported: string;
  local: string;
  source: string;
  loc?: SourceLocation;
  kind: ConstImportModuleKind;
};

export type AnalyzeDependencyImportFirstUsage = Array<
  AnalyzeDependencyImportUsageItem
>;

export type AnalyzeDependencyResult = {
  moduleType: AnalyzeModuleType;
  syntax: Array<ConstProgramSyntax>;
  diagnostics: PartialDiagnostics;
  firstTopAwaitLocation: undefined | SourceLocation;
  importFirstUsage: AnalyzeDependencyImportFirstUsage;
  exports: Array<AnyAnalyzeExport>;
  dependencies: Array<AnalyzeDependency>;
};

export const UNKNOWN_ANALYZE_DEPENDENCIES_RESULT: AnalyzeDependencyResult = {
  moduleType: 'unknown',
  syntax: [],
  diagnostics: [],
  firstTopAwaitLocation: undefined,
  importFirstUsage: [],
  exports: [],
  dependencies: [],
};
