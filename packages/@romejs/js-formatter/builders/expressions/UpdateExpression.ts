/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token, concat} from "../../tokens";
import {UpdateExpression} from "@romejs/js-ast";

export default function UpdateExpression(
	builder: Builder,
	node: UpdateExpression,
): Token {
	if (node.prefix === true) {
		return concat([node.operator, builder.tokenize(node.argument, node)]);
	} else {
		return concat([builder.tokenize(node.argument, node), node.operator]);
	}
}
