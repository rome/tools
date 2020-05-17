/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token, concat} from "../../tokens";
import {BindingArrayPattern} from "@romejs/js-ast";
import ArrayExpression from "../expressions/ArrayExpression";
import {printPatternMeta} from "../utils";

export default function BindingArrayPattern(
	builder: Builder,
	node: BindingArrayPattern,
): Token {
	return concat([
		ArrayExpression(builder, node),
		printPatternMeta(builder, node, node.meta),
	]);
}
