/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@internal/formatter";

import {AnyNode, TSStringLiteralTypeAnnotation} from "@internal/ast";
import JSStringLiteral from "../literals/JSStringLiteral";

export default function TSStringLiteralTypeAnnotation(
	builder: Builder,
	node: TSStringLiteralTypeAnnotation,
	parent: AnyNode,
): Token {
	return JSStringLiteral(builder, node, parent);
}
