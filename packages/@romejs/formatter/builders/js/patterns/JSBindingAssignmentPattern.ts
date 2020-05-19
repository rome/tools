/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSAssignmentAssignmentPattern,
	JSBindingAssignmentPattern,
} from "@romejs/ast";
import {Builder, Token, concat, space} from "@romejs/formatter";

export default function JSBindingAssignmentPattern(
	builder: Builder,
	node: JSAssignmentAssignmentPattern | JSBindingAssignmentPattern,
): Token {
	return concat([
		builder.tokenize(node.left, node),
		space,
		"=",
		space,
		builder.tokenize(node.right, node),
	]);
}
