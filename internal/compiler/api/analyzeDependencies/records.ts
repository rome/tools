/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	ConstJSExportModuleKind,
	JSReferenceIdentifier,
} from "@internal/ast";
import {SourceLocation} from "@internal/parser-core";
import {
	AnalyzeDependency,
	AnalyzeDependencyImportUsageItem,
	AnyAnalyzeExport,
} from "@internal/core";
import {Record} from "@internal/compiler";

export class ImportRecord extends Record {
	constructor(data: AnalyzeDependency) {
		super();
		this.data = data;
	}

	public data: AnalyzeDependency;
}

export class ExportRecord extends Record {
	constructor(data: AnyAnalyzeExport) {
		super();
		this.data = data;
	}

	public data: AnyAnalyzeExport;
}

// Whenever we encounter a reference to CJS module or exports
export class EscapedCJSRefRecord extends Record {
	constructor(node: AnyNode) {
		super();
		this.node = node;
	}

	public node: AnyNode;
}

// Whenever we encounter a exports or module.exports assignment
export class CJSExportRecord extends Record {
	constructor(node: AnyNode) {
		super();
		this.node = node;
	}

	public node: AnyNode;
}

export class CJSVarRefRecord extends Record {
	constructor(node: JSReferenceIdentifier) {
		super();
		this.node = node;
	}

	public node: JSReferenceIdentifier;
}

export class ESExportRecord extends Record {
	constructor(kind: ConstJSExportModuleKind, node: AnyNode) {
		super();
		this.node = node;
		this.kind = kind;
	}

	public node: AnyNode;
	public kind: ConstJSExportModuleKind;
}

// Whenever we encounter a top level await
export class TopLevelAwaitRecord extends Record {
	constructor(loc: SourceLocation) {
		super();
		this.loc = loc;
	}

	public loc: SourceLocation;
}

// Whenever we encounter the first reference to an import
export class ImportUsageRecord extends Record {
	constructor(isTop: boolean, data: AnalyzeDependencyImportUsageItem) {
		super();
		this.isTop = isTop;
		this.data = data;
	}

	public isTop: boolean;
	public data: AnalyzeDependencyImportUsageItem;
}
