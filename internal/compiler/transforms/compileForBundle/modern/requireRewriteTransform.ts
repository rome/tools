/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	jsAssignmentExpression,
	jsAssignmentIdentifier,
	jsIdentifier,
} from "@internal/ast";
import {createVisitor, signals} from "@internal/compiler";
import {
	doesNodeMatchPattern,
	inheritLoc,
	template,
} from "@internal/js-ast-utils";
import {getOptions, getPrefixedNamespace} from "../_utils";

export default createVisitor({
	name: "requireRewriteTransform",
	enter(path) {
		const {node, context} = path;

		const {relativeSourcesToModuleId, moduleId} = getOptions(context);

		// Replace all references to module.exports to the correct version
		if (
			node.type === "JSMemberExpression" &&
			doesNodeMatchPattern(node, "module.exports")
		) {
			return signals.replace(
				jsIdentifier.create({
					name: getPrefixedNamespace(moduleId),
					loc: inheritLoc(node, "module.exports"),
				}),
			);
		}

		// Replace all assignments of module.exports to the correct version
		if (
			node.type === "JSAssignmentExpression" &&
			doesNodeMatchPattern(node.left, "module.exports")
		) {
			return signals.replace(
				jsAssignmentExpression.create({
					operator: node.operator,
					left: jsAssignmentIdentifier.create({
						name: getPrefixedNamespace(moduleId),
						loc: inheritLoc(node, "module.exports"),
					}),
					right: node.right,
				}),
			);
		}

		// Replace import foo = require('module');
		if (
			node.type === "TSImportEqualsDeclaration" &&
			node.moduleReference.type === "TSExternalModuleReference"
		) {
			return signals.replace(
				template.statement`const ${node.id} = require(${node.moduleReference.expression});`,
			);
		}

		// Now handle normal `require('module')`
		if (node.type !== "JSCallExpression") {
			return signals.retain;
		}

		const {callee} = node;
		if (callee.type !== "JSReferenceIdentifier" || callee.name !== "require") {
			return signals.retain;
		}

		const sourceArg = node.arguments[0];
		if (sourceArg.type !== "JSStringLiteral") {
			return signals.retain;
		}

		if (path.scope.hasBinding("require")) {
			return signals.retain;
		}

		const replacement = relativeSourcesToModuleId[sourceArg.value];
		if (typeof replacement === "string") {
			return signals.replace(
				jsIdentifier.create({
					name: getPrefixedNamespace(replacement),
				}),
			);
		}

		return signals.retain;
	},
});
