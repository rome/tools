/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSBigIntLiteral} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function JSBigIntLiteral(
	builder: Builder,
	node: JSBigIntLiteral,
): Token {
	return `${node.value}n`;
}
