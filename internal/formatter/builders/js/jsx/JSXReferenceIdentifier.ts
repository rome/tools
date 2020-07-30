/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@internal/formatter";

import {JSXReferenceIdentifier} from "@internal/ast";

export default function JSXReferenceIdentifier(
	builder: Builder,
	node: JSXReferenceIdentifier,
): Token {
	return node.name;
}
