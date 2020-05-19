/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@romejs/formatter";

import {JSBindingArrayPattern} from "@romejs/ast";
import JSArrayExpression from "../expressions/JSArrayExpression";
import {printPatternMeta} from "../utils";

export default function JSBindingArrayPattern(
	builder: Builder,
	node: JSBindingArrayPattern,
): Token {
	return concat([
		JSArrayExpression(builder, node),
		printPatternMeta(builder, node, node.meta),
	]);
}
