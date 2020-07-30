/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	ConstJSExportModuleKind,
	ConstJSImportModuleKind,
	JSBindingIdentifier,
	JSReferenceIdentifier,
} from "@internal/ast";
import {SourceLocation} from "@internal/parser-core";
import {
	ClassBinding,
	FunctionBinding,
	Path,
	Scope,
	TypeBinding,
} from "@internal/compiler";
import {
	AnalyzeDependency,
	AnalyzeDependencyName,
	AnalyzeDependencyResult,
	AnalyzeExportValueType,
	AnyAnalyzeExport,
} from "@internal/core";

export function isOptional(path: Path): boolean {
	for (const {node} of path.ancestryPaths) {
		if (node.type === "JSTryStatement") {
			return true;
		}
	}

	return false;
}

export function isTypeKind(kind: undefined | ConstJSImportModuleKind): boolean {
	return kind === "type" || kind === "typeof";
}

export function getImportKind(
	kind: undefined | ConstJSImportModuleKind,
): ConstJSImportModuleKind {
	return kind ?? "value";
}

export function getExportKind(
	kind: undefined | ConstJSExportModuleKind,
): ConstJSExportModuleKind {
	return kind ?? "value";
}

export function maybeTypeBinding(
	kind: ConstJSExportModuleKind,
	scope: Scope,
	id: JSBindingIdentifier | JSReferenceIdentifier,
): ConstJSExportModuleKind {
	const binding = scope.getBinding(id.name);
	if (kind === "value" && binding instanceof TypeBinding) {
		return "type";
	} else {
		return kind;
	}
}

export function getKindWithSpecifiers(
	rawKind: undefined | ConstJSImportModuleKind,
	specifierKinds: Array<ConstJSImportModuleKind>,
): ConstJSImportModuleKind {
	const kind: ConstJSImportModuleKind = getImportKind(rawKind);
	if (isTypeKind(kind) || specifierKinds.length === 0) {
		return kind;
	}

	for (const specifierKind of specifierKinds) {
		if (specifierKind === "value") {
			return "value";
		}
	}
	return "type";
}

// We use this to have an easy way to identify the actual runtime type of an import
// This is useful as we needs this as Flow allows you to `import type` classes which
// are considered values
export function getAnalyzeExportValueType(
	scope: Scope,
	node: undefined | AnyNode,
): AnalyzeExportValueType {
	if (node === undefined) {
		return "other";
	}

	if (node.type === "JSIdentifier") {
		const binding = scope.getBinding(node.name);

		if (binding instanceof FunctionBinding) {
			return "function";
		}

		if (binding instanceof ClassBinding) {
			return "class";
		}

		if (binding instanceof TypeBinding) {
			const {typeKind} = binding;
			switch (typeKind) {
				case "function":
				case "class":
					return typeKind;
			}
		}
	}

	if (node.type === "JSFunctionDeclaration") {
		return "function";
	}

	if (node.type === "JSClassDeclaration" || node.type === "JSClassExpression") {
		return "class";
	}

	return "other";
}

// Resolve a export declaration to it's binding node if one exists
export function getDeclarationLoc(
	scope: Scope,
	node: AnyNode,
): undefined | SourceLocation {
	if (node.type === "JSReferenceIdentifier") {
		const binding = scope.getBinding(node.name);
		if (binding !== undefined) {
			return binding.node.loc;
		}
	}

	return node.loc;
}

function arraySame<T>(
	a: Array<T>,
	b: Array<T>,
	callback: (a: T, b: T) => boolean,
): boolean {
	if (a.length !== b.length) {
		return false;
	}

	for (let i = 0; i < a.length; i++) {
		if (!callback(a[i], b[i])) {
			return false;
		}
	}

	return true;
}

function exportsSame(a: AnyAnalyzeExport, b: AnyAnalyzeExport): boolean {
	if (a.type !== b.type) {
		return false;
	}

	if (a.kind !== b.kind) {
		return false;
	}

	switch (a.type) {
		case "local":
			return b.type === "local" && a.name === b.name;

		case "external":
			return (
				b.type === "external" &&
				a.imported === b.imported &&
				a.exported === b.exported &&
				a.source === b.source
			);

		case "externalAll":
			return b.type === "externalAll" && a.source === b.source;

		case "externalNamespace":
			return (
				b.type === "externalNamespace" &&
				a.source === b.source &&
				a.exported === b.exported
			);
	}
}

function dependencyNameSame(
	a: AnalyzeDependencyName,
	b: AnalyzeDependencyName,
): boolean {
	return a.kind === b.kind && a.name === b.name;
}

function dependenciesSame(a: AnalyzeDependency, b: AnalyzeDependency): boolean {
	return (
		a.all === b.all &&
		a.async === b.async &&
		a.optional === b.optional &&
		a.source === b.source &&
		a.type === b.type &&
		arraySame(a.names, b.names, dependencyNameSame)
	);
}

// Check if the shape of two analyzeDependencyResults are equal. Ignoring location information
export function areAnalyzeDependencyResultsEqual(
	a: AnalyzeDependencyResult,
	b: AnalyzeDependencyResult,
): boolean {
	if (
		(a.firstTopAwaitLocation === undefined &&
		b.firstTopAwaitLocation !== undefined) ||
		(b.firstTopAwaitLocation === undefined &&
		a.firstTopAwaitLocation !== undefined)
	) {
		return false;
	}

	if (a.moduleType !== b.moduleType) {
		return false;
	}

	if (!arraySame(a.exports, b.exports, exportsSame)) {
		return false;
	}

	if (!arraySame(a.dependencies, b.dependencies, dependenciesSame)) {
		return false;
	}

	return true;
}
