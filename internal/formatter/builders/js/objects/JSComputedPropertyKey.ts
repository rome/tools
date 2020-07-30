/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@internal/formatter";

import {JSComputedPropertyKey} from "@internal/ast";

export default function JSComputedPropertyKey(
	builder: Builder,
	node: JSComputedPropertyKey,
): Token {
	return concat(["[", builder.tokenize(node.value, node), "]"]);
}
