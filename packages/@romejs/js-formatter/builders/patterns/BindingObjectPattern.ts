/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token, concat} from "../../tokens";
import {BindingObjectPattern} from "@romejs/js-ast";
import ObjectExpression from "../objects/ObjectExpression";
import {printPatternMeta} from "../utils";

export default function BindingObjectPattern(
	builder: Builder,
	node: BindingObjectPattern,
): Token {
	return concat([
		ObjectExpression(builder, node),
		printPatternMeta(builder, node, node.meta),
	]);
}
