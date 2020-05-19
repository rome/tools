/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romejs/formatter";

import {JSStaticPropertyKey} from "@romejs/ast";

export default function JSStaticPropertyKey(
	builder: Builder,
	node: JSStaticPropertyKey,
): Token {
	return builder.tokenize(node.value, node);
}
