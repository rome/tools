/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romejs/formatter";

import {JSReferenceIdentifier} from "@romejs/ast";
import JSIdentifier from "../auxiliary/JSIdentifier";

export default function JSReferenceIdentifier(
	builder: Builder,
	node: JSReferenceIdentifier,
): Token {
	return JSIdentifier(builder, node);
}
