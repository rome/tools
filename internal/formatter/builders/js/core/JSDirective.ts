/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {AnyNode, JSDirective} from "@internal/ast";
import JSStringLiteral from "../literals/JSStringLiteral";

export default function JSDirective(
	builder: Builder,
	node: JSDirective,
	parent: AnyNode,
): Token {
	return concat([JSStringLiteral(builder, node, parent), ";"]);
}
