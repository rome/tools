/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {
	JSExportExternalSpecifier,
	JSExportLocalSpecifier,
	JSImportSpecifier,
} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";
import {naturalCompare} from "@internal/string-utils";

function compareImportSpecifiers(
	a: JSImportSpecifier,
	b: JSImportSpecifier,
): number {
	const order = naturalCompare(a.local.name.name, b.local.name.name, false);
	if (order === 0) {
		return naturalCompare(a.imported.name, b.imported.name, false);
	} else {
		return order;
	}
}

function compareExportSpecifiers<T extends
	| JSExportExternalSpecifier
	| JSExportLocalSpecifier>(a: T, b: T): number {
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

export default createVisitor({
	name: "js/useSortedSpecifiers",
	enter(path) {
		const {node} = path;

		if (node.type === "JSImportDeclaration") {
			if (node.namedSpecifiers.length > 1) {
				const specifiers = node.namedSpecifiers;
				const sortedSpecifiers = specifiers.slice().sort(
					compareImportSpecifiers,
				);
				if (shouldReorder(specifiers, sortedSpecifiers)) {
					return path.addFixableDiagnostic(
						{
							fixed: signals.replace({
								...node,
								namedSpecifiers: sortedSpecifiers,
							}),
						},
						descriptions.LINT.JS_SORT_IMPORT_SPECIFIERS,
					);
				}
			}
		} else if (node.type === "JSExportExternalDeclaration") {
			if (node.namedSpecifiers.length > 1) {
				const specifiers = node.namedSpecifiers;
				const sortedSpecifiers = specifiers.slice().sort(
					compareExportSpecifiers,
				);
				if (shouldReorder(specifiers, sortedSpecifiers)) {
					return path.addFixableDiagnostic(
						{
							fixed: signals.replace({
								...node,
								namedSpecifiers: sortedSpecifiers,
							}),
						},
						descriptions.LINT.JS_SORT_EXPORT_SPECIFIERS,
					);
				}
			}
		} else if (node.type === "JSExportLocalDeclaration") {
			if (node.specifiers !== undefined && node.specifiers.length > 1) {
				const specifiers = node.specifiers;
				const sortedSpecifiers = specifiers.slice().sort(
					compareExportSpecifiers,
				);
				if (shouldReorder(specifiers, sortedSpecifiers)) {
					return path.addFixableDiagnostic(
						{
							fixed: signals.replace({...node, specifiers: sortedSpecifiers}),
						},
						descriptions.LINT.JS_SORT_EXPORT_SPECIFIERS,
					);
				}
			}
		}

		return signals.retain;
	},
});
