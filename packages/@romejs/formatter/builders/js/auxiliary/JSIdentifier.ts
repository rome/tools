/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romejs/formatter";

import {
	JSAssignmentIdentifier,
	JSBindingIdentifier,
	JSIdentifier,
	JSReferenceIdentifier,
} from "@romejs/ast";

export default function JSIdentifier(
	builder: Builder,
	node:
		| JSAssignmentIdentifier
		| JSBindingIdentifier
		| JSIdentifier
		| JSReferenceIdentifier,
): Token {
	return node.name;
}
