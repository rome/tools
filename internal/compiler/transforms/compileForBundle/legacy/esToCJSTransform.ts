/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSStatement,
	JSClassExpression,
	JSFunctionExpression,
	jsStringLiteral,
} from "@internal/ast";
import {
	getBindingIdentifiers,
	getImportSpecifiers,
	template,
} from "@internal/js-ast-utils";
import {getModuleId, getOptions} from "../_utils";
import {FunctionBinding, createVisitor, signals} from "@internal/compiler";

export default createVisitor({
	name: "esToCJSTransform",
	enter(path) {
		const {node} = path;

		if (node.type !== "JSRoot") {
			return signals.retain;
		}

		const options = getOptions(path.context);

		const topBody: Array<AnyJSStatement> = [];
		const bottomBody: Array<AnyJSStatement> = [];

		for (const bodyNode of node.body) {
			if (bodyNode.type === "JSImportDeclaration") {
				if (bodyNode.importKind === "type" || bodyNode.importKind === "typeof") {
					continue;
				}

				const moduleId = getModuleId(bodyNode.source.value, options);
				if (moduleId === undefined) {
					continue;
				}

				const source = jsStringLiteral.create({
					value: moduleId,
				});

				const specifiers = getImportSpecifiers(bodyNode);
				if (specifiers.length === 0) {
					topBody.push(template.statement`Rome.requireNamespace(${source});`);
				} else {
					for (const specifier of specifiers) {
						if (specifier.type === "JSImportSpecifier") {
							topBody.push(
								template.statement`const ${specifier.local.name} = Rome.requireNamespace(${source}).${specifier.imported};`,
							);
						} else if (specifier.type === "JSImportNamespaceSpecifier") {
							topBody.push(
								template.statement`const ${specifier.local.name} = Rome.requireNamespace(${source});`,
							);
						} else if (specifier.type === "JSImportDefaultSpecifier") {
							topBody.push(
								template.statement`const ${specifier.local.name} = Rome.requireDefault(${source});`,
							);
						}
					}
				}
				continue;
			}

			if (bodyNode.type === "JSExportAllDeclaration") {
				// TODO
				continue;
			}

			if (bodyNode.type === "JSExportExternalDeclaration") {
				if (bodyNode.exportKind === "type") {
					continue;
				}

				const {source} = bodyNode;

				// TODO defaultSpecifier and namespaceSpecifier
				for (const specifier of bodyNode.namedSpecifiers) {
					topBody.push(
						template.statement`Object.defineProperty(exports, ${jsStringLiteral.create({
							value: specifier.exported.name,
						})}, {
                get: function() {
                  return Rome.requireNamespace(${source}).${specifier.local};
                },
              })`,
					);
				}
			}

			if (bodyNode.type === "JSExportLocalDeclaration") {
				if (bodyNode.exportKind === "type") {
					continue;
				}

				const {declaration, specifiers} = bodyNode;

				if (declaration !== undefined) {
					// Hoist function declarations
					if (declaration.type === "JSFunctionDeclaration") {
						topBody.push(
							template.statement`exports.${declaration.id} = ${declaration.id}`,
						);
						bottomBody.push(declaration);
						continue;
					}

					// Handle type declarations (these have no runtime ordering implications)
					if (
						declaration.type === "TSModuleDeclaration" ||
						declaration.type === "TSEnumDeclaration" ||
						declaration.type === "TSTypeAlias" ||
						declaration.type === "TSInterfaceDeclaration" ||
						declaration.type === "TSDeclareFunction"
					) {
						bottomBody.push(declaration);
						continue;
					}

					// Handle variables and classes
					if (
						declaration.type === "JSVariableDeclarationStatement" ||
						declaration.type === "JSClassDeclaration"
					) {
						bottomBody.push(declaration);

						for (const id of getBindingIdentifiers(declaration)) {
							topBody.push(template.statement`exports.${id} = undefined;`);
							bottomBody.push(template.statement`exports.${id} = ${id};`);
						}
					}
				}

				if (specifiers !== undefined) {
					for (const specifier of specifiers) {
						const binding = path.scope.getBinding(specifier.local.name);

						if (binding instanceof FunctionBinding) {
							topBody.push(
								template.statement`exports.${specifier.exported} = ${specifier.local};`,
							);
						} else {
							topBody.push(
								template.statement`exports.${specifier.exported} = undefined;`,
							);
							bottomBody.push(
								template.statement`exports.${specifier.exported} = ${specifier.local};`,
							);
						}
					}
				}

				continue;
			}

			if (bodyNode.type === "JSExportDefaultDeclaration") {
				const {declaration} = bodyNode;

				// Hoist function declarations
				if (declaration.type === "JSFunctionDeclaration") {
					// If it has an id then there's no way that anything in the JSRoot can refer to it, so inline it as a function expression
					if (declaration.id === undefined) {
						const expr: JSFunctionExpression = {
							...declaration,
							type: "JSFunctionExpression",
						};
						topBody.push(template.statement`exports.default = ${expr};`);
					} else {
						topBody.push(declaration);
						topBody.push(
							template.statement`exports.default = ${declaration.id};`,
						);
					}
					continue;
				}

				// Handle classes
				if (declaration.type === "JSClassDeclaration") {
					// Technically we could hoist these if they have no super class, but we don't as it's not spec compliant
					topBody.push(template.statement`exports.default = undefined;`);
					if (declaration.id === undefined) {
						const expr: JSClassExpression = {
							...declaration,
							type: "JSClassExpression",
						};
						bottomBody.push(template.statement`exports.default = ${expr};`);
					} else {
						bottomBody.push(declaration);
						bottomBody.push(
							template.statement`exports.default = ${declaration.id};`,
						);
					}
					continue;
				}

				// Handle type declarations (these have no runtime ordering implications)
				if (
					declaration.type === "TSInterfaceDeclaration" ||
					declaration.type === "TSDeclareFunction"
				) {
					// Maybe we should keep them? Not sure what they would desugar to
					continue;
				}

				// Otherwise it's an expression
				bottomBody.push(template.statement`exports.default = ${declaration};`);

				// There are cases where we could omit this declaration at all if we the file has no imports, some other conditions etc
				topBody.push(template.statement`exports.default = undefined;`);

				continue;
			}

			bottomBody.push(bodyNode);
		}

		return signals.replace({
			...node,
			body: [...topBody, ...bottomBody],
		});
	},
});
