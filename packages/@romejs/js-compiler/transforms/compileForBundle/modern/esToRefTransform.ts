/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	FunctionBinding,
	ImportBinding,
	Path,
	REDUCE_REMOVE,
	TransformExitResult,
	TypeBinding,
} from "@romejs/js-compiler";
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
} from "@romejs/js-ast-utils";
import {
	AnyNode,
	ObjectProperties,
	bindingIdentifier,
	blockStatement,
	functionHead,
	identifier,
	objectExpression,
	objectMethod,
	objectProperty,
	program,
	referenceIdentifier,
	returnStatement,
	staticPropertyKey,
	variableDeclaration,
	variableDeclarationStatement,
	variableDeclarator,
} from "@romejs/js-ast";

export default {
	name: "esToRefTransform",
	enter(path: Path): TransformExitResult {
		const {node, scope, context} = path;

		const opts = getOptions(context);

		if (program.is(node)) {
			const mappings = new Map();

			// make all variables private
			for (const [name] of path.scope.bindings) {
				mappings.set(name, getPrivateName(name, opts.moduleId));
			}

			// map exports and imports and correctly
			for (const child of node.body) {
				if (
					child.type === "ImportDeclaration" &&
					child.importKind !== "type" &&
					child.importKind !== "typeof"
				) {
					const moduleId = getModuleId(child.source.value, opts);
					if (moduleId === undefined) {
						continue;
					}

					for (const specifier of getImportSpecifiers(child)) {
						if (specifier.type === "ImportSpecifier") {
							mappings.set(
								specifier.local.name.name,
								getPrefixedName(specifier.imported.name, moduleId, opts),
							);
						} else if (specifier.type === "ImportNamespaceSpecifier") {
							mappings.set(specifier.local.name.name, getPrefixedNamespace(moduleId));
						} else if (specifier.type === "ImportDefaultSpecifier") {
							mappings.set(
								specifier.local.name.name,
								getPrefixedName("default", moduleId, opts),
							);
						} else {
							throw new Error("unexpected");
						}
					}
				}

				if (child.type === "ExportLocalDeclaration") {
					// export const foo = '';
					// export function foo() {}
					for (const {name} of getBindingIdentifiers(child)) {
						mappings.set(name, getPrefixedName(name, opts.moduleId, opts));
					}

					// export {foo};
					if (child.specifiers !== undefined) {
						for (const specifier of child.specifiers) {
							const local = specifier.local.name;
							if (scope.getBindingAssert(local) instanceof ImportBinding) {
								continue;
							}

							mappings.set(
								local,
								getPrefixedName(specifier.exported.name, opts.moduleId, opts),
							);
						}
					}
				}

				if (child.type === "ExportDefaultDeclaration") {
					const {declaration: decl} = child;
					if (
						(decl.type === "FunctionDeclaration" || decl.type === "ClassDeclaration") &&
						decl.id !== undefined
					) {
						mappings.set(
							decl.id.name,
							getPrefixedName("default", opts.moduleId, opts),
						);
					}
				}
			}

			const newProgram = program.assert(renameBindings(path, mappings));

			// Get new scope with updated bindings. TODO Maybe `renameBindings` should return the path?
			const newScope = scope.getRootScope().evaluate(newProgram, undefined, true);

			if (opts.moduleAll === true) {
				// Get all the export names
				const exportNames: Map<string, string> = new Map();
				for (const child of newProgram.body) {
					if (child.type === "ExportDefaultDeclaration") {
						exportNames.set(
							"default",
							getPrefixedName("default", opts.moduleId, opts),
						);
					}

					if (child.type === "ExportExternalDeclaration") {
						// TODO defaultSpecifier and namespaceSpecifier
						const {source} = child;

						for (const specifier of child.namedSpecifiers) {
							// If this is an external export then use the correct name
							const moduleId = getModuleId(source.value, opts);
							if (moduleId === undefined) {
								continue;
							}

							const local = getPrefixedName(specifier.local.name, moduleId, opts);

							exportNames.set(specifier.exported.name, local);
						}
					}

					if (child.type === "ExportLocalDeclaration") {
						if (child.declaration !== undefined) {
							throw new Error(
								"No export declarations should be here as they have been removed by renameBindings",
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

				const exportObjProps: ObjectProperties = [];

				for (const [exported, local] of exportNames) {
					const binding = newScope.getBinding(local);
					if (binding !== undefined) {
						if (binding instanceof TypeBinding) {
							continue;
						}

						if (binding instanceof FunctionBinding) {
							exportObjProps.push(
								objectProperty.create({
									key: staticPropertyKey.quick(identifier.quick(exported)),
									value: referenceIdentifier.quick(local),
								}),
							);
							continue;
						}
					}

					exportObjProps.push(
						objectMethod.create({
							kind: "get",
							key: staticPropertyKey.quick(identifier.quick(exported)),
							head: functionHead.quick([]),
							body: blockStatement.create({
								body: [
									returnStatement.create({
										argument: referenceIdentifier.create({
											name: local,
										}),
									}),
								],
							}),
						}),
					);
				}

				const exportObj = objectExpression.create({properties: exportObjProps});

				return {
					...newProgram,
					type: "Program",
					body: [
						variableDeclarationStatement.quick(
							variableDeclaration.create({
								kind: "const",
								declarations: [
									variableDeclarator.create({
										id: bindingIdentifier.create({
											name: getPrefixedNamespace(opts.moduleId),
										}),
										init: exportObj,
									}),
								],
							}),
						),
						...newProgram.body,
					],
				};
			} else {
				return newProgram;
			}
		}

		if (node.type === "ImportDeclaration") {
			// should have already been handled with the Program branch
			return REDUCE_REMOVE;
		}

		if (node.type === "ExportDefaultDeclaration") {
			const {declaration} = node;
			if (
				declaration.type === "FunctionDeclaration" ||
				declaration.type === "ClassDeclaration"
			) {
				if (declaration.id === undefined) {
					return {
						// give it the correct name
						...node,
						declaration: {
							...declaration,
							id: bindingIdentifier.create({
								name: getPrefixedName("default", opts.moduleId, opts),
							}),
						},
					};
				} else {
					// if the export was named then we'll have already given it the correct name
					return declaration;
				}
			} else {
				return template.statement`const ${getPrefixedName(
					"default",
					opts.moduleId,
					opts,
				)} = ${declaration};`;
			}
		}

		if (node.type === "ExportExternalDeclaration") {
			// Remove external exports with a source as they will be resolved correctly and never point here
			return REDUCE_REMOVE;
		}

		if (node.type === "ExportLocalDeclaration") {
			const {declaration, specifiers} = node;

			if (specifiers === undefined) {
				if (declaration === undefined) {
					throw new Error(
						"No specifiers or declaration existed, if there's no specifiers then there should be a declaration",
					);
				}
				return declaration;
			} else {
				// check if any of the specifiers reference a global or import
				// if so, we need to insert declarations for them
				const nodes: Array<AnyNode> = [];

				for (const specifier of specifiers) {
					if (specifier.type === "ExportLocalSpecifier") {
						const binding = path.scope.getBinding(specifier.local.name);

						// TODO we only really need this declaration for global bindings, `analyze()` could detect the exported import and resolvedImports would just work
						if (binding === undefined || binding instanceof ImportBinding) {
							nodes.push(
								variableDeclaration.create({
									kind: "const",
									declarations: [
										variableDeclarator.create({
											id: bindingIdentifier.create({
												name: getPrefixedName(specifier.exported.name, opts.moduleId, opts),
											}),
											init: referenceIdentifier.quick(specifier.local.name),
										}),
									],
								}),
							);
						}
					} else {
						// TODO ???
					}
				}

				if (nodes.length === 0) {
					return REDUCE_REMOVE;
				} else {
					return nodes;
				}
			}
		}

		if (node.type === "ExportAllDeclaration" && opts.moduleAll === true) {
			const moduleId = getModuleId(node.source.value, opts);
			if (moduleId === undefined) {
				return node;
			}

			const theirNamespace = getPrefixedNamespace(moduleId);
			const ourNamespace = getPrefixedNamespace(opts.moduleId);
			return template.statement`
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
      `;
		}

		if (node.type === "ExportAllDeclaration" && opts.moduleAll !== true) {
			// We can remove these, this signature has already been flagged by analyze() and we'll automatically forward it
			return REDUCE_REMOVE;
		}

		return node;
	},
};
