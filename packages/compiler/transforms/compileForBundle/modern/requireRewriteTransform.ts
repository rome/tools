/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	jsAssignmentExpression,
	jsAssignmentIdentifier,
	jsIdentifier,
} from "@romefrontend/ast";
import {Path} from "@romefrontend/compiler";
import {
	doesNodeMatchPattern,
	inheritLoc,
	template,
} from "@romefrontend/js-ast-utils";
import {getOptions, getPrefixedNamespace} from "../_utils";

export default {
	name: "requireRewriteTransform",
	enter(path: Path): AnyNode {
		const {node, context} = path;

		const {relativeSourcesToModuleId, moduleId} = getOptions(context);

		// Replace all references to module.exports to the correct version
		if (
			node.type === "JSMemberExpression" &&
			doesNodeMatchPattern(node, "module.exports")
		) {
			return jsIdentifier.create({
				name: getPrefixedNamespace(moduleId),
				loc: inheritLoc(node, "module.exports"),
			});
		}

		// Replace all assignments of module.exports to the correct version
		if (
			node.type === "JSAssignmentExpression" &&
			doesNodeMatchPattern(node.left, "module.exports")
		) {
			return jsAssignmentExpression.create({
				operator: node.operator,
				left: jsAssignmentIdentifier.create({
					name: getPrefixedNamespace(moduleId),
					loc: inheritLoc(node, "module.exports"),
				}),
				right: node.right,
			});
		}

		// Replace import foo = require('module');
		if (
			node.type === "TSImportEqualsDeclaration" &&
			node.moduleReference.type === "TSExternalModuleReference"
		) {
			return template.statement`const ${node.id} = require(${node.moduleReference.expression});`;
		}

		// Now handle normal `require('module')`
		if (node.type !== "JSCallExpression") {
			return node;
		}

		const {callee} = node;
		if (callee.type !== "JSReferenceIdentifier" || callee.name !== "require") {
			return node;
		}

		const sourceArg = node.arguments[0];
		if (sourceArg.type !== "JSStringLiteral") {
			return node;
		}

		if (path.scope.hasBinding("require")) {
			return node;
		}

		const replacement = relativeSourcesToModuleId[sourceArg.value];
		if (typeof replacement === "string") {
			return jsIdentifier.create({
				name: getPrefixedNamespace(replacement),
			});
		}

		return node;
	},
};
