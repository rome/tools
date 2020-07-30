/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, JSConditionalExpression} from "@internal/ast";
import {
	Builder,
	Token,
	concat,
	group,
	indent,
	lineOrSpace,
	space,
} from "@internal/formatter";

export default function JSConditionalExpression(
	builder: Builder,
	node: JSConditionalExpression,
	parent: AnyNode,
): Token {
	return printConditionalExpression(
		builder.tokenize(node.test, node),
		builder.tokenize(node.consequent, node),
		builder.tokenize(node.alternate, node),
		parent,
		node.consequent,
		node.alternate,
	);
}

function isConditionalExpression(node: AnyNode): boolean {
	return (
		node.type === "JSConditionalExpression" || node.type === "TSConditionalType"
	);
}

export function printConditionalExpression(
	test: Token,
	consequent: Token,
	alternate: Token,
	parentNode: AnyNode,
	consequentNode: AnyNode,
	alternateNode: AnyNode,
): Token {
	const printed = concat([
		test,
		indent(
			concat([
				lineOrSpace,
				"?",
				space,
				isConditionalExpression(consequentNode)
					? consequent
					: indent(consequent),
			]),
		),
		indent(
			concat([
				lineOrSpace,
				":",
				space,
				isConditionalExpression(alternateNode) ? alternate : indent(alternate),
			]),
		),
	]);

	// Do not group nested conditional expressions. By doing so, if a conditional
	// expression breaks, the hole chain breaks.
	return isConditionalExpression(parentNode) ? printed : group(printed);
}
