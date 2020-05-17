/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token} from "../../tokens";
import {AnyNode, TSStringLiteralTypeAnnotation} from "@romejs/js-ast";
import StringLiteral from "../literals/StringLiteral";

export default function TSStringLiteralTypeAnnotation(
	builder: Builder,
	node: TSStringLiteralTypeAnnotation,
	parent: AnyNode,
): Token {
	return StringLiteral(builder, node, parent);
}
