/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {TSAssignmentTypeAssertion} from "@internal/ast";

export default function TSAssignmentTypeAssertion(
	builder: Builder,
	node: TSAssignmentTypeAssertion,
): Token {
	return concat([
		"<",
		builder.tokenize(node.typeAnnotation, node),
		">",
		space,
		builder.tokenize(node.expression, node),
	]);
}
