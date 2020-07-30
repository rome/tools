/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@internal/formatter";

import {JSFunctionDeclaration} from "@internal/ast";
import JSFunctionExpression from "../expressions/JSFunctionExpression";

export default function JSFunctionDeclaration(
	builder: Builder,
	node: JSFunctionDeclaration,
): Token {
	return JSFunctionExpression(builder, node);
}
