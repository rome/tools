/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token} from "../../tokens";
import {AssignmentObjectPattern} from "@romejs/js-ast";
import ObjectExpression from "../objects/ObjectExpression";

export default function AssignmentObjectPattern(
	builder: Builder,
	node: AssignmentObjectPattern,
): Token {
	return ObjectExpression(builder, node);
}
