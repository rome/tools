/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, JSSequenceExpression} from "@internal/ast";
import {
	Builder,
	Token,
	concat,
	group,
	indent,
	join,
	lineOrSpace,
} from "@internal/formatter";

export default function JSSequenceExpression(
	builder: Builder,
	node: JSSequenceExpression,
	parent: AnyNode,
): Token {
	if (
		parent.type === "JSExpressionStatement" ||
		parent.type === "JSForStatement" ||
		parent.type === "JSSequenceExpression"
	) {
		// Indent expressions after the first to improve the readability
		return group(
			concat(
				node.expressions.map((expr, i) =>
					i === 0
						? builder.tokenize(expr, node)
						: concat([
								",",
								indent(concat([lineOrSpace, builder.tokenize(expr, node)])),
							])
				),
			),
		);
	} else {
		return group(
			join(
				concat([",", lineOrSpace]),
				node.expressions.map((expr) => builder.tokenize(expr, node)),
			),
		);
	}
}
