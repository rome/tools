/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from "../../Builder";
import {Token} from "../../tokens";
import {
	AssignmentIdentifier,
	BindingIdentifier,
	Identifier,
	ReferenceIdentifier,
} from "@romejs/js-ast";

export default function Identifier(
	builder: Builder,
	node:
		 | AssignmentIdentifier
		| BindingIdentifier
		| Identifier
		| ReferenceIdentifier,
): Token {
	return node.name;
}
