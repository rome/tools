/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@internal/formatter";

import {AnyNode, TSTemplateLiteralTypeAnnotation} from "@internal/ast";
import {printTemplateLiteral} from "../utils";

export default function TSTemplateLiteralTypeAnnotation(
	builder: Builder,
	node: TSTemplateLiteralTypeAnnotation,
	parent: AnyNode,
): Token {
	return printTemplateLiteral(builder, node, parent);
}
