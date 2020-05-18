/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romejs/formatter";

import {JSAssignmentArrayPattern} from "@romejs/ast";
import JSArrayExpression from "../expressions/JSArrayExpression";

export default function JSAssignmentArrayPattern(
	builder: Builder,
	node: JSAssignmentArrayPattern,
): Token {
	return JSArrayExpression(builder, node);
}
