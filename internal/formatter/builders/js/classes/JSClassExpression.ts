/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@internal/formatter";

import {JSClassExpression} from "@internal/ast";
import JSClassDeclaration from "./JSClassDeclaration";

export default function JSClassExpression(
	builder: Builder,
	node: JSClassExpression,
): Token {
	return JSClassDeclaration(builder, node);
}
