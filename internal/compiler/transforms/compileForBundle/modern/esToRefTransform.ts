/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	FunctionBinding,
	ImportBinding,
	TypeBinding,
	createVisitor,
	signals,
} from "@internal/compiler";
import {
	getModuleId,
	getOptions,
	getPrefixedName,
	getPrefixedNamespace,
	getPrivateName,
} from "../_utils";
import {
	getBindingIdentifiers,
	getImportSpecifiers,
	renameBindings,
	template,
} from "@internal/js-ast-utils";
import {
	AnyNode,
	JSObjectProperties,
	jsBindingIdentifier,
	jsBlockStatement,
	jsFunctionHead,
	jsIdentifier,
	jsObjectExpression,
	jsObjectMethod,
	jsObjectProperty,
	jsReferenceIdentifier,
	jsReturnStatement,
	jsRoot,
	jsStaticPropertyKey,
	jsVariableDeclaration,
	jsVariableDeclarationStatement,
	jsVariableDeclarator,
} from "@internal/ast";
import {pretty} from "@internal/pretty-format";

export default createVisitor({
	name: "esToRefTransform",
	enter(path) {
		const {node, scope, context} = path;

		const opts = getOptions(context);

		if (node.type === "JSRoot") {
			const mappings = new Map();

			// make all variables private
			for (const [name] of path.scope.getOwnBindings()) {
				mappings.set(name, getPrivateName(name, opts.moduleId));
			}

			// map exports and imports and correctly
			for (const child of node.body) {
				if (
					child.type === "JSImportDeclaration" &&
					child.importKind !== "type" &&
					child.importKind !== "typeof"
				) {
					const moduleId = getModuleId(child.source.value, opts);
					if (moduleId === undefined) {
						continue;
					}

					for (const specifier of getImportSpecifiers(child)) {
						if (specifier.type === "JSImportSpecifier") {
							mappings.set(
								specifier.local.name.name,
								getPrefixedName(specifier.imported.name, moduleId, opts),
							);
						} else if (specifier.type === "JSImportNamespaceSpecifier") {
							mappings.set(
								specifier.local.name.name,
								getPrefixedNamespace(moduleId),
							);
						} else if (specifier.type === "JSImportDefaultSpecifier") {
							mappings.set(
								specifier.local.name.name,
								getPrefixedName("default", moduleId, opts),
							);
						} else {
							throw new Error("unexpected");
						}
					}
				}

				if (child.type === "JSExportLocalDeclaration") {
					// export const foo = '';
					// export function foo() {}
					for (const {name} of getBindingIdentifiers(child)) {
						mappings.set(name, getPrefixedName(name, opts.moduleId, opts));
					}

					// export {foo};
					if (child.specifiers !== undefined) {
						for (const specifier of child.specifiers) {
							const local = specifier.local.name;
							const binding = scope.getBinding(local);
							if (binding === undefined || binding instanceof ImportBinding) {
								continue;
							}

							mappings.set(
								local,
								getPrefixedName(specifier.exported.name, opts.moduleId, opts),
							);
						}
					}
				}

				if (child.type === "JSExportDefaultDeclaration") {
					const {declaration: decl} = child;
					if (
						(decl.type === "JSFunctionDeclaration" ||
						decl.type === "JSClassDeclaration") &&
						decl.id !== undefined
					) {
						mappings.set(
							decl.id.name,
							getPrefixedName("default", opts.moduleId, opts),
						);
					}
				}
			}

			const newProgram = jsRoot.assert(renameBindings(path, mappings));

			// Get new scope with updated bindings. TODO Maybe `renameBindings` should return the path?
			const newScope = scope.getRootScope().enterEvaluate(newProgram);

			if (opts.moduleAll) {
				// Get all the export names
				const exportNames: Map<string, string> = new Map();
				for (const child of newProgram.body) {
					if (child.type === "JSExportDefaultDeclaration") {
						exportNames.set(
							"default",
							getPrefixedName("default", opts.moduleId, opts),
						);
					}

					if (child.type === "JSExportExternalDeclaration") {
						// TODO defaultSpecifier and namespaceSpecifier
						const {source} = child;

						for (const specifier of child.namedSpecifiers) {
							// If this is an external export then use the correct name
							const moduleId = getModuleId(source.value, opts);
							if (moduleId === undefined) {
								continue;
							}

							const local = getPrefixedName(
								specifier.local.name,
								moduleId,
								opts,
							);

							exportNames.set(specifier.exported.name, local);
						}
					}

					if (child.type === "JSExportLocalDeclaration") {
						if (child.declaration !== undefined) {
							throw new Error(
								pretty`No export declarations should be here as they have been removed by renameBindings. Node: ${child}`,
							);
						}

						const {specifiers} = child;
						if (specifiers !== undefined) {
							for (const specifier of specifiers) {
								// The local binding has already been rewritten by renameBindings if it existed
								exportNames.set(specifier.exported.name, specifier.local.name);
							}
						}
					}
				}

				const exportObjProps: JSObjectProperties = [];

				for (const [exported, local] of exportNames) {
					const binding = newScope.getBinding(local);
					if (binding !== undefined) {
						if (binding instanceof TypeBinding) {
							continue;
						}

						if (binding instanceof FunctionBinding) {
							exportObjProps.push(
								jsObjectProperty.create({
									key: jsStaticPropertyKey.quick(jsIdentifier.quick(exported)),
									value: jsReferenceIdentifier.quick(local),
								}),
							);
							continue;
						}
					}

					exportObjProps.push(
						jsObjectMethod.create({
							kind: "get",
							key: jsStaticPropertyKey.quick(jsIdentifier.quick(exported)),
							head: jsFunctionHead.quick([]),
							body: jsBlockStatement.create({
								body: [
									jsReturnStatement.create({
										argument: jsReferenceIdentifier.create({
											name: local,
										}),
									}),
								],
							}),
						}),
					);
				}

				const exportObj = jsObjectExpression.create({properties: exportObjProps});

				return signals.replace({
					...newProgram,
					type: "JSRoot",
					body: [
						jsVariableDeclarationStatement.quick(
							jsVariableDeclaration.create({
								kind: "const",
								declarations: [
									jsVariableDeclarator.create({
										id: jsBindingIdentifier.create({
											name: getPrefixedNamespace(opts.moduleId),
										}),
										init: exportObj,
									}),
								],
							}),
						),
						...newProgram.body,
					],
				});
			} else {
				return signals.replace(newProgram);
			}
		}

		if (node.type === "JSImportDeclaration") {
			// should have already been handled with the JSRoot branch
			return signals.remove;
		}

		if (node.type === "JSExportDefaultDeclaration") {
			const {declaration} = node;
			if (
				declaration.type === "JSFunctionDeclaration" ||
				declaration.type === "JSClassDeclaration"
			) {
				if (declaration.id === undefined) {
					return signals.replace({
						// give it the correct name
						...node,
						declaration: {
							...declaration,
							id: jsBindingIdentifier.create({
								name: getPrefixedName("default", opts.moduleId, opts),
							}),
						},
					});
				} else {
					// if the export was named then we'll have already given it the correct name
					return signals.replace(declaration);
				}
			} else {
				return signals.replace(
					template.statement`const ${getPrefixedName(
						"default",
						opts.moduleId,
						opts,
					)} = ${declaration};`,
				);
			}
		}

		if (node.type === "JSExportExternalDeclaration") {
			// Remove external exports with a source as they will be resolved correctly and never point here
			return signals.remove;
		}

		if (node.type === "JSExportLocalDeclaration") {
			const {declaration, specifiers} = node;

			if (specifiers === undefined) {
				if (declaration === undefined) {
					throw new Error(
						"No specifiers or declaration existed, if there's no specifiers then there should be a declaration",
					);
				}
				return signals.replace(declaration);
			} else {
				// Check if any of the specifiers reference a global or import
				// If so, we need to insert declarations for them
				const nodes: Array<AnyNode> = [];

				for (const specifier of specifiers) {
					if (specifier.type === "JSExportLocalSpecifier") {
						const binding = path.scope.getBinding(specifier.local.name);

						// TODO we only really need this declaration for global bindings, `analyze()` could detect the exported import and resolvedImports would just work
						if (binding === undefined || binding instanceof ImportBinding) {
							nodes.push(
								jsVariableDeclarationStatement.quick(
									jsVariableDeclaration.create({
										kind: "const",
										declarations: [
											jsVariableDeclarator.create({
												id: jsBindingIdentifier.create({
													name: getPrefixedName(
														specifier.exported.name,
														opts.moduleId,
														opts,
													),
												}),
												init: jsReferenceIdentifier.quick(specifier.local.name),
											}),
										],
									}),
								),
							);
						}
					} else {
						// TODO ???
					}
				}

				if (nodes.length === 0) {
					return signals.remove;
				} else {
					return signals.replace(nodes);
				}
			}
		}

		if (node.type === "JSExportAllDeclaration" && opts.moduleAll) {
			const moduleId = getModuleId(node.source.value, opts);
			if (moduleId === undefined) {
				return signals.retain;
			}

			const theirNamespace = getPrefixedNamespace(moduleId);
			const ourNamespace = getPrefixedNamespace(opts.moduleId);
			return signals.replace(
				template.statement`
        Object.keys(${theirNamespace}).forEach(function (key) {
          if (key === 'default') return undefined;
          Object.defineProperty(${ourNamespace}, key, {
            enumerable: true,
            configurable: true,
            get: function get() {
              return ${theirNamespace}[key];
            }
          });
        });
      `,
			);
		}

		if (node.type === "JSExportAllDeclaration" && !opts.moduleAll) {
			// We can remove these, this signature has already been flagged by analyze() and we'll automatically forward it
			return signals.remove;
		}

		return signals.retain;
	},
});
