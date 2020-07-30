/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Binding, Path, signals} from "@internal/compiler";
import {inheritLoc} from "./inheritLoc";
import {
	AnyJSVariableIdentifier,
	AnyNode,
	jsExportLocalDeclaration,
	jsExportLocalSpecifier,
	jsIdentifier,
	jsReferenceIdentifier,
} from "@internal/ast";
import {getBindingIdentifiers} from "./getBindingIdentifiers";
import {isVariableIdentifier} from "./isVariableIdentifier";
import {assertSingleOrMultipleNodes} from "./assertSingleOrMultipleNodes";

// This methods allows either passing in Bindings that could be present within deep scopes,
// or local names for the scope in the passed Path
export function renameBindings(
	path: Path,
	oldToNewMapping: Map<Binding | string, string>,
): AnyNode | Array<AnyNode> {
	if (oldToNewMapping.size === 0) {
		return path.node;
	}

	const oldBindingToNewName: Map<Binding, string> = new Map();

	// get a list of the current bindings for this scope
	const oldNameToBinding: Map<string, undefined | Binding> = new Map();
	for (const [oldName, newName] of oldToNewMapping) {
		if (typeof oldName === "string") {
			const binding = path.scope.getBinding(oldName);
			oldNameToBinding.set(oldName, binding);
		} else {
			oldBindingToNewName.set(oldName, newName);
		}
	}

	// discover nodes to replace first without manipulating the AST as that will change the scope and binding objects
	const replaceNodesWithName: Map<AnyJSVariableIdentifier, string> = new Map();
	path.traverse(
		"renameBindingsCollector",
		(path) => {
			const {node, scope} = path;
			if (!isVariableIdentifier(node)) {
				return;
			}

			const binding = scope.getBinding(node.name);

			// oldName -> newName
			if (
				oldToNewMapping.has(node.name) &&
				binding === oldNameToBinding.get(node.name)
			) {
				const newName = oldToNewMapping.get(node.name);
				if (newName === undefined) {
					throw new Error("Should exist");
				}
				replaceNodesWithName.set(node, newName);
			}

			// Binding -> newName
			if (binding !== undefined && oldBindingToNewName.has(binding)) {
				const newName = oldBindingToNewName.get(binding);
				if (newName === undefined) {
					throw new Error("Should exist");
				}
				replaceNodesWithName.set(node, newName);
			}
		},
	);
	if (replaceNodesWithName.size === 0) {
		return path.node;
	}

	//
	const replaced: Set<AnyNode> = new Set();

	// replace the nodes
	const renamedNode = path.reduceNode(
		{
			name: "renameBindings",
			enter(path) {
				const {node} = path;

				// Retain the correct exported name for `export function` and `export class`
				if (
					node.type === "JSExportLocalDeclaration" &&
					node.declaration !== undefined &&
					(node.declaration.type === "JSFunctionDeclaration" ||
					node.declaration.type === "JSClassDeclaration")
				) {
					const id = node.declaration.id;
					const oldName = id.name;
					const newName = replaceNodesWithName.get(id);

					if (newName !== undefined) {
						replaced.add(id);

						return signals.replace([
							node.declaration,
							jsExportLocalDeclaration.create({
								specifiers: [
									jsExportLocalSpecifier.create({
										loc: id.loc,
										local: jsReferenceIdentifier.quick(newName),
										exported: jsIdentifier.quick(oldName),
									}),
								],
							}),
						]);
					}
				}

				// Retain the correct exported names for `export const`
				if (
					node.type === "JSExportLocalDeclaration" &&
					node.declaration !== undefined
				) {
					const bindings = getBindingIdentifiers(node.declaration);
					let includesAny = false;
					for (const node of bindings) {
						if (replaceNodesWithName.has(node)) {
							includesAny = true;
							break;
						}
					}

					if (includesAny) {
						return signals.replace([
							node.declaration,
							jsExportLocalDeclaration.create({
								specifiers: bindings.map((node) => {
									let local: string = node.name;

									const newName = replaceNodesWithName.get(node);
									if (newName !== undefined) {
										local = newName;
										replaced.add(node);
									}

									return jsExportLocalSpecifier.create({
										loc: node.loc,
										local: jsReferenceIdentifier.quick(local),
										exported: jsIdentifier.quick(node.name),
									});
								}),
							}),
						]);
					}
				}

				if (isVariableIdentifier(node)) {
					const newName = replaceNodesWithName.get(node);
					if (newName !== undefined) {
						replaced.add(node);
						return signals.replace({
							...node,
							name: newName,
							loc: inheritLoc(node, node.name),
						});
					}
				}

				return signals.retain;
			},
		},
		{
			noScopeCreation: true,
		},
	);

	//
	if (replaced.size !== replaceNodesWithName.size) {
		throw new Error("Missed some bindings");
	}

	return assertSingleOrMultipleNodes(renamedNode);
}
