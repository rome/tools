/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ConstJSExportModuleKind, ConstJSImportModuleKind} from "@internal/ast";
import {ImportBinding, createVisitor, signals} from "@internal/compiler";
import {AnalyzeDependencyName} from "@internal/core";
import {
	getBindingIdentifiers,
	getImportSpecifiers,
	isFunctionNode,
	isInTypeAnnotation,
} from "@internal/js-ast-utils";
import {
	ESExportRecord,
	ExportRecord,
	ImportRecord,
	ImportUsageRecord,
	TopLevelAwaitRecord,
} from "../records";
import {
	getAnalyzeExportValueType,
	getDeclarationLoc,
	getExportKind,
	getImportKind,
	getKindWithSpecifiers,
	isOptional,
	maybeTypeBinding,
} from "../utils";

export default createVisitor({
	name: "analyzeDependenciesES",
	enter(path) {
		const {node, scope, context} = path;

		// import('./bar');
		if (node.type === "JSImportCall" && node.argument.type === "JSStringLiteral") {
			context.record(
				new ImportRecord({
					type: "es",
					async: true,
					kind: "value",
					names: [],
					loc: node.argument.loc,
					source: node.argument.value,
					optional: isOptional(path),
					all: true,
				}),
			);
		}

		// Local bindings exports:
		// export const foo
		// export function foo() {}
		// export {};
		if (node.type === "JSExportLocalDeclaration") {
			const valueType = getAnalyzeExportValueType(scope, node.declaration);
			for (const id of getBindingIdentifiers(node)) {
				const kind = maybeTypeBinding(getExportKind(node.exportKind), scope, id);
				context.record(
					new ExportRecord({
						type: "local",
						valueType,
						kind,
						loc: getDeclarationLoc(scope, id),
						name: id.name,
					}),
				);
			}

			const {specifiers} = node;
			if (specifiers !== undefined) {
				for (const specifier of specifiers) {
					const kind: ConstJSExportModuleKind = maybeTypeBinding(
						getExportKind(specifier.exportKind || node.exportKind),
						scope,
						specifier.local,
					);

					context.record(
						new ExportRecord({
							type: "local",
							loc: getDeclarationLoc(scope, specifier.local),
							valueType: getAnalyzeExportValueType(scope, specifier.local),
							kind,
							name: specifier.exported.name,
						}),
					);
				}
			}
		}

		// export default
		if (node.type === "JSExportDefaultDeclaration") {
			context.record(
				new ExportRecord({
					type: "local",
					loc: getDeclarationLoc(scope, node.declaration),
					valueType: getAnalyzeExportValueType(scope, node.declaration),
					kind: "value",
					name: "default",
				}),
			);
		}

		// External binding exports:
		// export {} from '';
		if (node.type === "JSExportExternalDeclaration") {
			const {source} = node;
			const specifiersKinds: Array<ConstJSImportModuleKind> = [];
			const exportedNames: Array<AnalyzeDependencyName> = [];

			const {namedSpecifiers, defaultSpecifier, namespaceSpecifier} = node;

			if (defaultSpecifier !== undefined) {
				context.record(
					new ExportRecord({
						type: "external",
						kind: "value",
						loc: defaultSpecifier.loc,
						imported: "default",
						exported: defaultSpecifier.exported.name,
						source: source.value,
					}),
				);
			}

			if (namespaceSpecifier !== undefined) {
				context.record(
					new ExportRecord({
						type: "externalNamespace",
						kind: "value",
						loc: namespaceSpecifier.loc,
						exported: namespaceSpecifier.exported.name,
						source: source.value,
					}),
				);
			}

			for (const specifier of namedSpecifiers) {
				const kind = getImportKind(specifier.exportKind || node.exportKind);
				specifiersKinds.push(kind);

				exportedNames.push({
					name: specifier.local.name,
					kind,
					loc: specifier.loc,
				});

				context.record(
					new ExportRecord({
						type: "external",
						kind,
						loc: specifier.loc,
						imported: specifier.local.name,
						exported: specifier.exported.name,
						source: source.value,
					}),
				);
			}

			context.record(
				new ImportRecord({
					type: "es",
					async: false,
					kind: getKindWithSpecifiers(node.exportKind, specifiersKinds),
					names: exportedNames,
					loc: source.loc,
					source: source.value,
					optional: isOptional(path),
					all: false,
				}),
			);
		}

		// TS: import A = require('B');
		if (
			node.type === "TSImportEqualsDeclaration" &&
			node.moduleReference.type === "TSExternalModuleReference"
		) {
			context.record(
				new ImportRecord({
					type: "cjs",
					kind: "value",
					optional: isOptional(path),
					loc: node.loc,
					source: node.moduleReference.expression.value,
					names: [],
					all: true,
					async: false,
				}),
			);
		}

		// export * from '';
		if (node.type === "JSExportAllDeclaration") {
			context.record(
				new ImportRecord({
					type: "es",
					async: false,
					kind: getExportKind(node.exportKind),
					optional: isOptional(path),
					loc: node.source.loc,
					names: [],
					source: node.source.value,
					all: true,
				}),
			);

			context.record(
				new ExportRecord({
					type: "externalAll",
					loc: node.loc,
					kind: getExportKind(node.exportKind),
					source: node.source.value,
				}),
			);
		}

		if (
			node.type === "JSExportAllDeclaration" ||
			node.type === "JSExportDefaultDeclaration" ||
			node.type === "JSExportLocalDeclaration"
		) {
			context.record(new ESExportRecord(getExportKind(node.exportKind), node));
		}

		// import {} from '';
		// import * as foo from '';
		if (node.type === "JSImportDeclaration") {
			let hasNamespaceSpecifier = false;
			const specifierKinds: Array<ConstJSImportModuleKind> = [];
			const names: Array<AnalyzeDependencyName> = [];

			for (const specifier of getImportSpecifiers(node)) {
				if (specifier.type === "JSImportNamespaceSpecifier") {
					hasNamespaceSpecifier = true;
					break;
				}

				const kind: ConstJSImportModuleKind = getImportKind(node.importKind);
				specifierKinds.push(kind);

				if (specifier.type === "JSImportDefaultSpecifier") {
					names.push({
						kind,
						loc: specifier.loc,
						name: "default",
					});
				}

				if (specifier.type === "JSImportSpecifier") {
					names.push({
						kind,
						loc: specifier.loc,
						name: specifier.imported.name,
					});
				}
			}

			context.record(
				new ImportRecord({
					type: "es",
					async: false,
					kind: getKindWithSpecifiers(node.importKind, specifierKinds),
					loc: node.source.loc,
					optional: isOptional(path),
					source: node.source.value,
					all: hasNamespaceSpecifier,
					names,
				}),
			);
		}

		// Detect top level await
		if (
			node.type === "JSAwaitExpression" &&
			path.findAncestry((path) => isFunctionNode(path.node)) === undefined
		) {
			const {loc} = node;
			if (loc === undefined) {
				throw new Error("loc is undefined on JSAwaitExpression we want to mark");
			}
			context.record(new TopLevelAwaitRecord(loc));
		}

		if (node.type === "JSReferenceIdentifier") {
			const binding = path.scope.getBinding(node.name);

			// Mark references to imports outside of functions
			if (binding !== undefined && binding instanceof ImportBinding) {
				const {meta} = binding;

				// We can skip this if it's referencing a namespace
				if (meta.type !== "name") {
					return signals.retain;
				}

				// These are nodes that will defer the execution of code outside the init path
				// (They could still be triggered with an actual function call but this is just for some basic analysis)
				const deferredExecution = path.findAncestry((path) =>
					isFunctionNode(path.node) || path.node.type === "JSClassProperty"
				);
				const isTop = deferredExecution === undefined;

				let kind: ConstJSImportModuleKind = getImportKind(meta.kind);
				if (isInTypeAnnotation(path)) {
					kind = "type";
				}

				context.record(
					new ImportUsageRecord(
						isTop,
						{
							kind,
							loc: node.loc,
							local: node.name,
							imported: meta.imported,
							source: meta.source,
						},
					),
				);
			}
		}

		return signals.retain;
	},
});
