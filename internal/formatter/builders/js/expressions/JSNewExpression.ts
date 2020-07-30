/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {JSNewExpression} from "@internal/ast";
import JSCallExpression from "./JSCallExpression";

export default function JSNewExpression(
	builder: Builder,
	node: JSNewExpression,
): Token {
	return concat(["new", space, JSCallExpression(builder, node)]);
}
