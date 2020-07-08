/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romefrontend/formatter";

import {JSXIdentifier} from "@romefrontend/ast";

export default function JSXIdentifier(
	builder: Builder,
	node: JSXIdentifier,
): Token {
	return node.name;
}
