/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {JSAssignmentExpression} from "@internal/ast";
import {printAssignment} from "../utils";

export default function JSAssignmentExpression(
	builder: Builder,
	node: JSAssignmentExpression,
): Token {
	return printAssignment(
		builder,
		node,
		node.left,
		concat([space, node.operator]),
		node.right,
	);
}
