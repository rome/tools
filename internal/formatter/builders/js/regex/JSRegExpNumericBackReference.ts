/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romefrontend/formatter";

import {JSRegExpNumericBackReference} from "@romefrontend/ast";

export default function JSRegExpNumericBackReference(
	builder: Builder,
	node: JSRegExpNumericBackReference,
): Token {
	return `\\${node.value}`;
}
