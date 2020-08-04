/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {
	doesNodeMatchPattern,
	getNodeReferenceParts,
	getRequireSource,
} from "@internal/js-ast-utils";
import {
	CJSExportRecord,
	CJSVarRefRecord,
	EscapedCJSRefRecord,
	ExportRecord,
	ImportRecord,
} from "../records";
import {
	getAnalyzeExportValueType,
	getDeclarationLoc,
	isOptional,
} from "../utils";

export default createVisitor({
	name: "analyzeDependenciesCJS",
	enter(path) {
		const {node, parent, scope, context} = path;

		// Handle require()
		if (node.type === "JSCallExpression") {
			const {callee, arguments: args} = node;

			const isRequire: boolean =
				callee.type === "JSReferenceIdentifier" &&
				callee.name === "require" &&
				!path.scope.hasBinding("require");
			const sourceArg = args[0];

			if (isRequire && args.length === 1 && sourceArg.type === "JSStringLiteral") {
				context.record(
					new ImportRecord({
						type: "cjs",
						kind: "value",
						optional: isOptional(path),
						loc: node.loc,
						source: sourceArg.value,
						names: [],
						all: true,
						async: false,
					}),
				);
			}
		}

		// Detect assignments to exports and module.exports as definitely being an CJS module
		if (node.type === "JSAssignmentExpression") {
			const isModuleExports =
				path.scope.getBinding("module") === undefined &&
				(doesNodeMatchPattern(node.left, "module.exports") ||
				doesNodeMatchPattern(node.left, "module.exports.**"));
			const isExports =
				path.scope.getBinding("exports") === undefined &&
				(doesNodeMatchPattern(node.left, "exports") ||
				doesNodeMatchPattern(node.left, "exports.**"));

			if (isModuleExports || isExports) {
				context.record(new CJSExportRecord(node));
			}

			if (isModuleExports) {
				const {right} = node;

				if (right.type === "JSObjectExpression") {
					context.record(
						new ExportRecord({
							type: "local",
							loc: getDeclarationLoc(scope, node.right),
							valueType: getAnalyzeExportValueType(scope, node.right),
							kind: "value",
							name: "default",
						}),
					);

					for (const prop of right.properties) {
						// Don't allow spread, unknown, or computed properties
						if (
							prop.type === "JSSpreadProperty" ||
							(prop.key.type === "JSComputedPropertyKey" &&
							prop.key.value.type !== "JSStringLiteral")
						) {
							context.record(new EscapedCJSRefRecord(prop));
							continue;
						}

						const key = prop.key.value;
						let name: string;
						if (key.type === "JSIdentifier") {
							name = key.name;
						} else if (key.type === "JSStringLiteral") {
							name = key.value;
						} else {
							// Unknown key literal
							context.record(new EscapedCJSRefRecord(key));
							continue;
						}

						let target = prop.type === "JSObjectMethod" ? prop : prop.value;

						context.record(
							new ExportRecord({
								type: "local",
								loc: getDeclarationLoc(scope, target),
								valueType: getAnalyzeExportValueType(scope, target),
								kind: "value",
								name,
							}),
						);
					}
				} else {
					const source = getRequireSource(node.right, scope);
					if (source === undefined) {
						context.record(
							new ExportRecord({
								type: "local",
								loc: getDeclarationLoc(scope, node.right),
								valueType: getAnalyzeExportValueType(scope, node.right),
								kind: "value",
								name: "default",
							}),
						);
					} else {
						context.record(
							new ExportRecord({
								type: "externalAll",
								loc: getDeclarationLoc(scope, node.right),
								kind: "value",
								source,
							}),
						);

						context.record(
							new ExportRecord({
								type: "external",
								kind: "value",
								loc: getDeclarationLoc(scope, node.right),
								imported: "default",
								exported: "default",
								source,
							}),
						);
					}
				}
			}

			if (isExports) {
				const {parts} = getNodeReferenceParts(node.left);

				if (parts.length >= 2) {
					// parts[0] is exports
					const name = parts[1].value;

					context.record(
						new ExportRecord({
							type: "local",
							loc: getDeclarationLoc(scope, node.right),
							valueType: getAnalyzeExportValueType(scope, node.right),
							kind: "value",
							name,
						}),
					);
				}
			}
		}

		if (node.type === "JSReferenceIdentifier") {
			const binding = path.scope.getBinding(node.name);

			// Detect references to exports and module
			if (binding === undefined) {
				if (
					node.name === "__filename" ||
					node.name === "__dirname" ||
					node.name === "require" ||
					node.name === "module" ||
					node.name === "exports"
				) {
					context.record(new CJSVarRefRecord(node));
				}

				if (node.name === "module" || node.name === "exports") {
					const inMemberExpression =
						parent.type === "JSMemberExpression" && parent.object === node;
					if (!inMemberExpression) {
						context.record(new EscapedCJSRefRecord(node));
					}
				}
			}
		}

		return signals.retain;
	},
});
