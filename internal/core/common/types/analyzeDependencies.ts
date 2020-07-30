/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostics} from "@internal/diagnostics";
import {ConstJSExportModuleKind, ConstJSImportModuleKind} from "@internal/ast";
import {SourceLocation} from "@internal/parser-core";
import {Dict} from "@internal/typescript-helpers";

export type AnalyzeModuleType = "es" | "cjs" | "unknown";

export type AnalyzeDependencyName = {
	name: string;
	kind: ConstJSImportModuleKind;
	loc?: SourceLocation;
};

export type AnalyzeExportValueType = "class" | "function" | "other";

export type AnalyzeExportLocal = {
	type: "local";
	loc?: SourceLocation;
	kind: ConstJSExportModuleKind;
	valueType: AnalyzeExportValueType;
	name: string;
};

export type AnyAnalyzeExport =
	| AnalyzeExportLocal
	| {
			type: "externalNamespace";
			kind: ConstJSImportModuleKind;
			loc?: SourceLocation;
			exported: string;
			source: string;
		}
	| {
			type: "external";
			kind: ConstJSImportModuleKind;
			loc?: SourceLocation;
			imported: string;
			exported: string;
			source: string;
		}
	| {
			type: "externalAll";
			loc?: SourceLocation;
			kind: ConstJSImportModuleKind;
			source: string;
		};

export type AnalyzeDependency = {
	names: Array<AnalyzeDependencyName>;
	async: boolean;
	kind: ConstJSImportModuleKind;
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
	kind: ConstJSImportModuleKind;
};

export type AnalyzeDependencyImportFirstUsage = Array<AnalyzeDependencyImportUsageItem>;

export type AnalyzeDependencyTopLevelLocalBindings = Dict<
	undefined | SourceLocation
>;

export type AnalyzeDependencyResult = {
	moduleType: AnalyzeModuleType;
	diagnostics: Diagnostics;
	topLevelLocalBindings: AnalyzeDependencyTopLevelLocalBindings;
	firstTopAwaitLocation: undefined | SourceLocation;
	importFirstUsage: AnalyzeDependencyImportFirstUsage;
	exports: Array<AnyAnalyzeExport>;
	dependencies: Array<AnalyzeDependency>;
};

export const UNKNOWN_ANALYZE_DEPENDENCIES_RESULT: AnalyzeDependencyResult = {
	topLevelLocalBindings: {},
	moduleType: "unknown",
	diagnostics: [],
	firstTopAwaitLocation: undefined,
	importFirstUsage: [],
	exports: [],
	dependencies: [],
};
