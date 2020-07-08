/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romefrontend/compiler";
import {AnyJSStatement, AnyNode, JSImportDeclaration} from "@romefrontend/ast";
import {SourceLocation} from "@romefrontend/parser-core";
import {descriptions} from "@romefrontend/diagnostics";

export default {
	name: "duplicateImport",
	enter(path: Path): AnyNode {
		const {node} = path;

		if (node.type === "JSRoot") {
			const skipImports: Set<JSImportDeclaration> = new Set();
			const seenSources: Map<string, undefined | SourceLocation> = new Map();
			let shouldFix = false;

			for (const bodyNode of node.body) {
				if (bodyNode.type === "JSImportDeclaration") {
					const source = bodyNode.source.value;

					// Allow duplicate sources if the `importKind` is different
					const sourceKey =
						bodyNode.importKind === undefined
							? source
							: `${bodyNode.importKind}:${source}`;

					const seenLoc = seenSources.get(sourceKey);
					if (seenLoc === undefined) {
						seenSources.set(sourceKey, bodyNode.loc);
					} else {
						const {suppressed} = path.context.addNodeDiagnostic(
							bodyNode,
							descriptions.LINT.JS_DUPLICATE_IMPORT_SOURCE(seenLoc),
							{fixable: true},
						);

						if (suppressed) {
							skipImports.add(bodyNode);
						} else {
							shouldFix = true;
						}
					}
				}
			}

			// Defer fixing unless it's totally necessary since there's additional overhead
			if (shouldFix) {
				const newBody: Array<AnyJSStatement> = [];

				for (let i = 0; i < node.body.length; i++) {
					const bodyNode = node.body[i];

					if (bodyNode.type === "JSImportDeclaration") {
						// Skip import if it's already been consumed
						if (skipImports.has(bodyNode)) {
							continue;
						}

						let {
							namedSpecifiers,
							defaultSpecifier,
							namespaceSpecifier,
						} = bodyNode;

						// Find and concat all duplicate imports
						for (let x = i + 1; x < node.body.length; x++) {
							const possibleDuplicateNode = node.body[x];

							if (
								possibleDuplicateNode.type === "JSImportDeclaration" &&
								bodyNode.source.value === possibleDuplicateNode.source.value &&
								bodyNode.importKind === possibleDuplicateNode.importKind &&
								!skipImports.has(possibleDuplicateNode)
							) {
								skipImports.add(possibleDuplicateNode);
								namedSpecifiers = [
									...namedSpecifiers,
									...possibleDuplicateNode.namedSpecifiers,
								];

								// We do not currently handle renaming duplicate namespace and default bindings
								if (defaultSpecifier === undefined) {
									defaultSpecifier = possibleDuplicateNode.defaultSpecifier;
								}
								if (namespaceSpecifier === undefined) {
									namespaceSpecifier = possibleDuplicateNode.namespaceSpecifier;
								}
							}
						}

						newBody.push({
							...bodyNode,
							defaultSpecifier,
							namespaceSpecifier,
							namedSpecifiers,
						});
					} else {
						newBody.push(bodyNode);
					}
				}

				return {
					...node,
					body: newBody,
				};
			}
		}
		return node;
	},
};
