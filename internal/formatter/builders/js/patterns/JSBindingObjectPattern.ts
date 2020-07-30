/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {JSBindingObjectPattern} from "@internal/ast";
import JSObjectExpression from "../objects/JSObjectExpression";
import {printPatternMeta} from "../utils";

export default function JSBindingObjectPattern(
	builder: Builder,
	node: JSBindingObjectPattern,
): Token {
	return concat([
		JSObjectExpression(builder, node),
		printPatternMeta(builder, node, node.meta),
	]);
}
