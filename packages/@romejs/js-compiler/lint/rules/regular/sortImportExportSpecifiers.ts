/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from "@romejs/js-compiler";
import {
	ExportExternalSpecifier,
	ExportLocalSpecifier,
	ImportSpecifier,
} from "@romejs/js-ast";
import {descriptions} from "@romejs/diagnostics";
import {naturalCompare} from "@romejs/string-utils";

function compareImportSpecifiers(a: ImportSpecifier, b: ImportSpecifier): number {
	const order = naturalCompare(a.local.name.name, b.local.name.name, false);
	if (order === 0) {
		return naturalCompare(a.imported.name, b.imported.name, false);
	} else {
		return order;
	}
}

function compareExportSpecifiers<T extends
	| ExportExternalSpecifier
	| ExportLocalSpecifier>(a: T, b: T): number {
	const order = naturalCompare(a.local.name, b.local.name, false);
	if (order === 0) {
		return naturalCompare(a.exported.name, b.exported.name, false);
	} else {
		return order;
	}
}

function shouldReorder<T>(a: Array<T>, b: Array<T>) {
	for (let i = 0; i < a.length && i < b.length; i++) {
		if (a[i] !== b[i]) {
			return true;
		}
	}

	return false;
}

export default {
	name: "sortImportExportSpecifiers",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (node.type === "ImportDeclaration") {
			if (node.namedSpecifiers.length > 1) {
				const specifiers = node.namedSpecifiers;
				const sortedSpecifiers = specifiers.slice().sort(compareImportSpecifiers);
				if (shouldReorder(specifiers, sortedSpecifiers)) {
					return context.addFixableDiagnostic(
						{
							old: node,
							fixed: {...node, namedSpecifiers: sortedSpecifiers},
						},
						descriptions.LINT.SORT_IMPORT_SPECIFIERS,
					);
				}
			}
		} else if (node.type === "ExportExternalDeclaration") {
			if (node.namedSpecifiers.length > 1) {
				const specifiers = node.namedSpecifiers;
				const sortedSpecifiers = specifiers.slice().sort(compareExportSpecifiers);
				if (shouldReorder(specifiers, sortedSpecifiers)) {
					return context.addFixableDiagnostic(
						{
							old: node,
							fixed: {...node, namedSpecifiers: sortedSpecifiers},
						},
						descriptions.LINT.SORT_EXPORT_SPECIFIERS,
					);
				}
			}
		} else if (node.type === "ExportLocalDeclaration") {
			if (node.specifiers !== undefined && node.specifiers.length > 1) {
				const specifiers = node.specifiers;
				const sortedSpecifiers = specifiers.slice().sort(compareExportSpecifiers);
				if (shouldReorder(specifiers, sortedSpecifiers)) {
					return context.addFixableDiagnostic(
						{
							old: node,
							fixed: {...node, specifiers: sortedSpecifiers},
						},
						descriptions.LINT.SORT_EXPORT_SPECIFIERS,
					);
				}
			}
		}

		return node;
	},
};
