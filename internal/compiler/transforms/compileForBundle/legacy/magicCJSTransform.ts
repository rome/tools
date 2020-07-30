/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {
	JSFunctionExpression,
	jsBlockStatement,
	jsFunctionExpression,
	jsStringLiteral,
} from "@internal/ast";
import {template} from "@internal/js-ast-utils";
import {getOptions} from "../_utils";

export default createVisitor({
	name: "magicCJSTransform",
	enter(path) {
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
				return signals.retain;
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
				return signals.replace(
					template.expression`Rome.requireNamespace(${sourceNode})`,
				);
			}
		}

		if (
			node.type === "JSReferenceIdentifier" &&
			node.name === "require" &&
			scope.getBinding("require") === undefined
		) {
			return signals.replace(template.expression`Rome.requireNamespace`);
		}

		return signals.retain;
	},
	exit(path) {
		const {node, context} = path;
		const options = getOptions(context);

		// Add module wrapper
		if (node.type === "JSRoot") {
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

			return signals.replace({
				...node,
				directives: [],
				body: [wrapper],
			});
		}

		return signals.retain;
	},
});
