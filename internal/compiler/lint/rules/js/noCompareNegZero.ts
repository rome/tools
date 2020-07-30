/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
import {AnyNode} from "@internal/ast";
import {createVisitor, signals} from "@internal/compiler";
import {template} from "@internal/js-ast-utils";
import {descriptions} from "@internal/diagnostics";

const OPERATORS_TO_CHECK = [">", ">=", "<", "<=", "==", "===", "!=", "!=="];

function isNegZero(node: AnyNode): boolean {
	return (
		node.type === "JSUnaryExpression" &&
		node.operator === "-" &&
		node.argument.type === "JSNumericLiteral" &&
		node.argument.value === 0
	);
}

export default createVisitor({
	name: "js/noCompareNegZero",
	enter(path) {
		const {node} = path;

		if (
			node.type === "JSBinaryExpression" &&
			OPERATORS_TO_CHECK.includes(node.operator) &&
			(isNegZero(node.left) || isNegZero(node.right))
		) {
			if (node.operator === "===") {
				return path.addFixableDiagnostic(
					{
						fixed: signals.replace(
							template.expression`Object.is(${node.left}, ${node.right})`,
						),
					},
					descriptions.LINT.JS_NO_COMPARE_NEG_ZERO(node.operator),
				);
			} else {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JS_NO_COMPARE_NEG_ZERO(node.operator),
				);
			}
		}

		return signals.retain;
	},
});
