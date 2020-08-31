/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, space} from "@internal/formatter";

import {JSBindingIdentifier} from "@internal/ast";
import JSIdentifier from "../auxiliary/JSIdentifier";
import {printPatternMeta} from "../utils";

export default function JSBindingIdentifier(
	builder: Builder,
	node: JSBindingIdentifier,
): Token {
	if (node.name[0] === "*") {
		// Internal name
		return "";
	}

	return concat([
		node.meta?.accessibility ? concat([node.meta.accessibility, space]) : "",
		JSIdentifier(builder, node),
		printPatternMeta(builder, node, node.meta),
	]);
}
