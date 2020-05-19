/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@romejs/formatter";

import {JSImportCall} from "@romejs/ast";

export default function JSImportCall(
	builder: Builder,
	node: JSImportCall,
): Token {
	return concat(["import(", builder.tokenize(node.argument, node), ")"]);
}
