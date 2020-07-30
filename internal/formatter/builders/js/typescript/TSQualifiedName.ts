/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSQualifiedName} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function TSQualifiedName(
	builder: Builder,
	node: TSQualifiedName,
): Token {
	return concat([
		builder.tokenize(node.left, node),
		".",
		builder.tokenize(node.right, node),
	]);
}
