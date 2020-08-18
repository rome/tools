/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Binding, createVisitor, signals} from "@internal/compiler";
import {
	AnyNode,
	JSImportDeclaration,
	JSImportSpecifier,
	jsBindingIdentifier,
	jsIdentifier,
	jsImportDeclaration,
	jsImportSpecifier,
	jsImportSpecifierLocal,
	jsReferenceIdentifier,
	jsxIdentifier,
} from "@internal/ast";
import {isIdentifierish} from "@internal/js-ast-utils";
import {ExtendedMap} from "@internal/collections";

// TODO: Remove this. This contains React for the following reason:
//   A user may write: import * as React from 'react';
//   We will remove the namespace and have only the used specifiers
//   But the JSX plugin inserts `React.createElement`. Oh no.
const IGNORED_NAMES = ["React", "react"];

function getName(node: AnyNode): undefined | string {
	if (node.type !== "JSMemberExpression" && node.type !== "JSXMemberExpression") {
		return undefined;
	}

	const {property} = node;

	if (property.type === "JSComputedMemberProperty") {
		if (property.value.type === "JSStringLiteral") {
			return property.value.value;
		}
	} else {
		if (isIdentifierish(property)) {
			return property.name;
		}
	}

	return undefined;
}

export default createVisitor({
	name: "optimizeImports",
	enter(path) {
		const {node} = path;

		if (node.type !== "JSRoot") {
			return signals.retain;
		}

		// Check if we have any wildcard imports
		const wildcardImports: Map<
			string,
			{
				binding: Binding;
				names: Set<string>;
				mappings: ExtendedMap<string, string>;
				references: Set<AnyNode>;
			}
		> = new Map();
		const wildcardImportNodeToLocal: ExtendedMap<JSImportDeclaration, string> = new ExtendedMap(
			"wildcardImportNodeToLocal",
		);
		for (const child of node.body) {
			if (
				child.type === "JSImportDeclaration" &&
				!IGNORED_NAMES.includes(child.source.value) &&
				child.namespaceSpecifier !== undefined
			) {
				const specifier = child.namespaceSpecifier;
				wildcardImports.set(
					specifier.local.name.name,
					{
						binding: path.scope.getBindingAssert(specifier.local.name.name),
						names: new Set(),
						mappings: new ExtendedMap("wildcard import mappings"),
						references: new Set(),
					},
				);
				wildcardImportNodeToLocal.set(child, specifier.local.name.name);
			}
		}
		if (wildcardImports.size === 0) {
			return signals.retain;
		}

		// - Find all imported names from this namespace
		// - Remove the namespaces that have computed property access
		path.traverse(
			"optimizeImportsWildcardCollector",
			(path) => {
				const {node, parent} = path;
				if (node.type !== "JSReferenceIdentifier") {
					return;
				}

				// Ensure we're referencing a wildcard import
				const wildcardInfo = wildcardImports.get(node.name);
				if (wildcardInfo === undefined) {
					return;
				}

				// Ensure that the binding hasn't been shadowed
				if (path.scope.getBinding(node.name) !== wildcardInfo.binding) {
					return;
				}

				const isComputed =
					parent.type === "JSMemberExpression" &&
					parent.object === node &&
					getName(parent) === undefined;
				const isUnboxed =
					parent.type !== "JSMemberExpression" &&
					parent.type !== "JSXMemberExpression";

				if (isComputed || isUnboxed) {
					// Deopt as we can't follow this
					wildcardImports.delete(node.name);
				} else {
					const name = getName(parent);
					if (name === undefined) {
						throw new Error("Expected name");
					}
					wildcardInfo.names.add(name);
					wildcardInfo.references.add(parent);
				}
			},
		);
		if (wildcardImports.size === 0) {
			return signals.retain;
		}

		// Populate the `mappings` field with a uid
		for (const info of wildcardImports.values()) {
			for (const name of info.names) {
				info.mappings.set(name, path.scope.generateUid(name));
			}
		}

		return path.reduceSignal({
			name: "optimizeImportWilcards",
			enter(path) {
				const {node} = path;

				// Replace all member expressions with their uids
				if (
					(node.type === "JSMemberExpression" ||
					node.type === "JSXMemberExpression") &&
					isIdentifierish(node.object)
				) {
					const wildcardInfo = wildcardImports.get(node.object.name);
					if (wildcardInfo !== undefined && wildcardInfo.references.has(node)) {
						const name = getName(node);
						if (name === undefined) {
							throw new Error("Expected name");
						}

						const newName = wildcardInfo.mappings.assert(name);
						if (node.type === "JSXMemberExpression") {
							return signals.replace(jsxIdentifier.quick(newName));
						} else {
							return signals.replace(jsReferenceIdentifier.quick(newName));
						}
					}
				}

				// Add new specifiers to wildcard import declarations
				if (
					node.type === "JSImportDeclaration" &&
					wildcardImportNodeToLocal.has(node)
				) {
					const local = wildcardImportNodeToLocal.assert(node);

					const wildcardInfo = wildcardImports.get(local);
					if (wildcardInfo === undefined) {
						// We would have deopted earlier
						return signals.retain;
					}

					// Remove wildcard specifier
					let namedSpecifiers: Array<JSImportSpecifier> = [
						...(node.namedSpecifiers || []),
					];

					// Add on our new mappings
					for (const [imported, local] of wildcardInfo.mappings) {
						namedSpecifiers.push(
							jsImportSpecifier.create({
								imported: jsIdentifier.quick(imported),
								local: jsImportSpecifierLocal.quick(
									jsBindingIdentifier.quick(local),
								),
							}),
						);
					}

					return signals.replace(
						jsImportDeclaration.create({
							...node,
							namespaceSpecifier: undefined,
							namedSpecifiers,
						}),
					);
				}

				return signals.retain;
			},
		});
	},
});
