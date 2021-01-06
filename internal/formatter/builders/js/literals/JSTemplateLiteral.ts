/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@internal/formatter";
import {AnyNode, JSTemplateLiteral} from "@internal/ast";
import {printTemplateLiteral} from "../utils";

export default function JSTemplateLiteral(
	builder: Builder,
	node: JSTemplateLiteral,
	parent: AnyNode,
): Token {
	return printTemplateLiteral(builder, node, parent);
}
