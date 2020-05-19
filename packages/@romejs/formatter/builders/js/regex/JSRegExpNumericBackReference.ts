/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romejs/formatter";

import {JSRegExpNumericBackReference} from "@romejs/ast";

export default function JSRegExpNumericBackReference(
	builder: Builder,
	node: JSRegExpNumericBackReference,
): Token {
	return `\\${node.value}`;
}
