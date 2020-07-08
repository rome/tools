/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@romefrontend/formatter";

import {JSRegExpNamedBackReference} from "@romefrontend/ast";

export default function JSRegExpNamedBackReference(
	builder: Builder,
	node: JSRegExpNamedBackReference,
): Token {
	return concat(["\\k", "<", node.name, ">"]);
}
