/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {JSMetaProperty} from "@internal/ast";

export default function JSMetaProperty(
	builder: Builder,
	node: JSMetaProperty,
): Token {
	return concat([
		builder.tokenize(node.meta, node),
		".",
		builder.tokenize(node.property, node),
	]);
}
