/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romejs/formatter";

import {AnyNode, JSLogicalExpression} from "@romejs/ast";
import JSBinaryExpression from "./JSBinaryExpression";

export default function JSLogicalExpression(
	builder: Builder,
	node: JSLogicalExpression,
	parent: AnyNode,
): Token {
	return JSBinaryExpression(builder, node, parent);
}
