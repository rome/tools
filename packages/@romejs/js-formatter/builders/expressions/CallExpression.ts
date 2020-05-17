/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	CallExpression,
	NewExpression,
	OptionalCallExpression,
} from "@romejs/js-ast";
import {isFunctionNode} from "@romejs/js-ast-utils";
import Builder from "../../Builder";
import {
	Token,
	concat,
	group,
	hardline,
	ifBreak,
	indent,
	softline,
} from "../../tokens";
import {printCommaList} from "../utils";
import {hasInnerComments} from "../comments";

type AnyCallableExpression =
	| CallExpression
	| OptionalCallExpression
	| NewExpression;

export default function CallExpression(
	builder: Builder,
	node: AnyCallableExpression,
): Token {
	const tokens: Array<Token> = [builder.tokenize(node.callee, node)];

	if (node.type === "OptionalCallExpression") {
		tokens.push("?.");
	}

	if (node.typeArguments) {
		tokens.push(builder.tokenize(node.typeArguments, node));
	}

	tokens.push(printArguments(builder, node));

	return concat(tokens);
}

function printArguments(builder: Builder, node: AnyCallableExpression): Token {
	if (node.arguments.length === 0) {
		if (hasInnerComments(node)) {
			return concat([
				"(",
				builder.tokenizeInnerComments(node, true),
				hardline,
				")",
			]);
		} else {
			return "()";
		}
	}

	if (node.arguments.length === 1) {
		const argument = node.arguments[0];
		if (
			argument.type === "ArrayExpression" ||
			argument.type === "ObjectExpression" ||
			isFunctionNode(argument)
		) {
			return concat(["(", builder.tokenize(argument, node), ")"]);
		}
	}

	return group(
		concat([
			"(",
			indent(concat([softline, printCommaList(builder, node.arguments, node)])),
			ifBreak(","),
			softline,
			")",
		]),
	);
}
