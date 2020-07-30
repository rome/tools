/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {
	AnyJSObjectPropertyKey,
	AnyNode,
	JSAssignmentObjectPatternProperty,
	JSBindingObjectPatternProperty,
	JSObjectProperty,
} from "@internal/ast";

function isShorthand(key: AnyJSObjectPropertyKey, value: AnyNode): boolean {
	return (
		key.type === "JSStaticPropertyKey" &&
		key.value.type === "JSIdentifier" &&
		(value.type === "JSReferenceIdentifier" ||
		value.type === "JSBindingIdentifier" ||
		value.type === "JSAssignmentIdentifier") &&
		value.name === key.value.name
	);
}

export default function JSObjectProperty(
	builder: Builder,
	node:
		| JSObjectProperty
		| JSAssignmentObjectPatternProperty
		| JSBindingObjectPatternProperty,
): Token {
	const tokens = [builder.tokenize(node.key, node)];

	if (
		(node.value.type === "JSBindingAssignmentPattern" ||
		node.value.type === "JSAssignmentAssignmentPattern") &&
		isShorthand(node.key, node.value.left)
	) {
		return concat([
			concat(tokens),
			space,
			"=",
			space,
			builder.tokenize(node.value.right, node.value),
		]);
	} else if (isShorthand(node.key, node.value)) {
		return concat(tokens);
	} else {
		return concat([
			concat(tokens),
			":",
			space,
			builder.tokenize(node.value, node),
		]);
	}
}
