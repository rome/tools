/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/js-compiler";
import {
	AnyNode,
	JSFunctionExpression,
	jsBlockStatement,
	jsFunctionExpression,
	jsStringLiteral,
} from "@romejs/ast";
import {template} from "@romejs/js-ast-utils";
import {getOptions} from "../_utils";

export default {
	name: "magicCJSTransform",
	enter(path: Path): AnyNode {
		const {node, scope, context} = path;
		const options = getOptions(context);

		// Update relative requires with their module id
		if (
			node.type === "JSCallExpression" &&
			node.callee.type === "JSReferenceIdentifier" &&
			node.callee.name === "require" &&
			scope.getBinding("require") === undefined
		) {
			const args = node.arguments;
			const arg = args[0];

			// Maybe error?
			if (args.length !== 1 || arg.type !== "JSStringLiteral") {
				return node;
			}

			const source = arg.value;

			if (
				Object.prototype.hasOwnProperty.call(
					options.relativeSourcesToModuleId,
					source,
				)
			) {
				const resolved = options.relativeSourcesToModuleId[source];
				const sourceNode = jsStringLiteral.create({
					value: resolved,
				});
				return template.expression`Rome.requireNamespace(${sourceNode})`;
			}
		}

		if (
			node.type === "JSReferenceIdentifier" &&
			node.name === "require" &&
			scope.getBinding("require") === undefined
		) {
			return template.expression`Rome.requireNamespace`;
		}

		return node;
	},
	exit(path: Path): AnyNode {
		const {node, context} = path;
		const options = getOptions(context);

		// Add module wrapper
		if (node.type === "JSProgram") {
			const source = jsStringLiteral.create({
				value: options.moduleId,
			});

			// Build factory
			const factoryBody = jsBlockStatement.create({
				directives: node.directives,
				body: node.body,
			});

			const factory: JSFunctionExpression = {
				...jsFunctionExpression.assert(
					template.expression`(function(module, exports) {})`,
				),
				body: factoryBody,
			};

			// Build call
			const declare =
				options.analyze.moduleType === "es"
					? template.expression`Rome.declareES`
					: template.expression`Rome.declareCJS`;
			const wrapper = template.statement`${declare}(${source}, ${factory})`;

			return {
				...node,
				directives: [],
				body: [wrapper],
			};
		}

		return node;
	},
};
