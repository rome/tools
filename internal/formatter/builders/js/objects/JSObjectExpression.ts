/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	JSAssignmentObjectPattern,
	JSBindingObjectPattern,
	JSObjectExpression,
} from "@internal/ast";
import {
	Builder,
	Token,
	concat,
	group,
	ifBreak,
	indent,
	join,
	lineOrSpace,
	softline,
} from "@internal/formatter";
import {hasInnerComments} from "../../comments";

export default function JSObjectExpression(
	builder: Builder,
	node: JSObjectExpression | JSAssignmentObjectPattern | JSBindingObjectPattern,
): Token {
	if (hasInnerComments(node)) {
		return group(
			concat(["{", builder.tokenizeInnerComments(node, true), softline, "}"]),
		);
	}

	const tokens: Array<Token> = [];
	const props: Array<AnyNode> = node.properties;

	tokens.push(
		join(
			concat([",", lineOrSpace]),
			props.map((prop, index) => {
				const printed = builder.tokenize(prop, node);
				if (index > 0 && builder.getLinesBetween(props[index - 1], prop) > 1) {
					return concat([softline, printed]);
				} else {
					return printed;
				}
			}),
		),
	);

	if (
		(node.type === "JSBindingObjectPattern" ||
		node.type === "JSAssignmentObjectPattern") &&
		node.rest !== undefined
	) {
		if (props.length > 0) {
			tokens.push(",", lineOrSpace);
			if (builder.getLinesBetween(props[props.length - 1], node.rest) > 1) {
				tokens.push(softline);
			}
		}

		tokens.push("...", builder.tokenize(node.rest, node));
	} else if (props.length > 0) {
		// Add trailing comma
		tokens.push(ifBreak(","));
	}

	// If the first property is not one the same line as the opening brace,
	// the object is printed on multiple lines.
	const shouldBreak =
		node.loc !== undefined &&
		props.length > 0 &&
		props[0].loc !== undefined &&
		props[0].loc.start.line !== node.loc.start.line;

	return group(
		concat(["{", indent(concat([softline, concat(tokens)])), softline, "}"]),
		shouldBreak,
	);
}
