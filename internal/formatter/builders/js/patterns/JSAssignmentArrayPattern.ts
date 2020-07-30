/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@internal/formatter";

import {JSAssignmentArrayPattern} from "@internal/ast";
import JSArrayExpression from "../expressions/JSArrayExpression";

export default function JSAssignmentArrayPattern(
	builder: Builder,
	node: JSAssignmentArrayPattern,
): Token {
	return JSArrayExpression(builder, node);
}
