/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */ import {AnyNode, TSConditionalType} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

import {printConditionalExpression} from "../expressions/JSConditionalExpression";

export default function TSConditionalType(
	builder: Builder,
	node: TSConditionalType,
	parent: AnyNode,
): Token {
	return printConditionalExpression(
		concat([
			builder.tokenize(node.checkType, node),
			space,
			"extends",
			space,
			builder.tokenize(node.extendsType, node),
		]),
		builder.tokenize(node.trueType, node),
		builder.tokenize(node.falseType, node),
		parent,
		node.trueType,
		node.falseType,
	);
}
