/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {JSXNamespacedName} from "@internal/ast";

export default function JSXNamespacedName(
	builder: Builder,
	node: JSXNamespacedName,
): Token {
	return concat([
		builder.tokenize(node.namespace, node),
		":",
		builder.tokenize(node.name, node),
	]);
}
