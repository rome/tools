/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  ConstExportModuleKind,
  AnyNode,
  ReferenceIdentifier,
} from '@romejs/js-ast';
import {SourceLocation} from '@romejs/parser-core';
import {
  AnalyzeDependency,
  AnyAnalyzeExport,
  AnalyzeDependencyImportUsageItem,
} from '@romejs/core';
import {Record} from '@romejs/js-compiler';

export class ImportRecord extends Record {
  constructor(data: AnalyzeDependency) {
    super();
    this.data = data;
  }

  data: AnalyzeDependency;
}

export class ExportRecord extends Record {
  constructor(data: AnyAnalyzeExport) {
    super();
    this.data = data;
  }

  data: AnyAnalyzeExport;
}

// Whenever we encounter a reference to CJS module or exports
export class EscapedCJSRefRecord extends Record {
  constructor(node: AnyNode) {
    super();
    this.node = node;
  }

  node: AnyNode;
}

// Whenever we encounter a exports or module.exports assignment
export class CJSExportRecord extends Record {
  constructor(node: AnyNode) {
    super();
    this.node = node;
  }

  node: AnyNode;
}

export class CJSVarRefRecord extends Record {
  constructor(node: ReferenceIdentifier) {
    super();
    this.node = node;
  }

  node: ReferenceIdentifier;
}

export class ESExportRecord extends Record {
  constructor(kind: ConstExportModuleKind, node: AnyNode) {
    super();
    this.node = node;
    this.kind = kind;
  }

  node: AnyNode;
  kind: ConstExportModuleKind;
}

// Whenever we encounter a top level await
export class TopLevelAwaitRecord extends Record {
  constructor(loc: SourceLocation) {
    super();
    this.loc = loc;
  }

  loc: SourceLocation;
}

// Whenever we encounter the first reference to an import
export class ImportUsageRecord extends Record {
  constructor(isTop: boolean, data: AnalyzeDependencyImportUsageItem) {
    super();
    this.isTop = isTop;
    this.data = data;
  }

  isTop: boolean;
  data: AnalyzeDependencyImportUsageItem;
}
