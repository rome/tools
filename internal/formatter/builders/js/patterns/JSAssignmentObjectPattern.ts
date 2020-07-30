/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romefrontend/formatter";

import {JSAssignmentObjectPattern} from "@romefrontend/ast";
import JSObjectExpression from "../objects/JSObjectExpression";

export default function JSAssignmentObjectPattern(
	builder: Builder,
	node: JSAssignmentObjectPattern,
): Token {
	return JSObjectExpression(builder, node);
}
