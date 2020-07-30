/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romefrontend/formatter";

import {JSOptionalCallExpression} from "@romefrontend/ast";
import JSCallExpression from "./JSCallExpression";

export default function JSOptionalCallExpression(
	builder: Builder,
	node: JSOptionalCallExpression,
): Token {
	return JSCallExpression(builder, node);
}
