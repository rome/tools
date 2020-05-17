/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token, concat, space} from "../../tokens";
import {NewExpression} from "@romejs/js-ast";
import CallExpression from "./CallExpression";

export default function NewExpression(
	builder: Builder,
	node: NewExpression,
): Token {
	return concat(["new", space, CallExpression(builder, node)]);
}
