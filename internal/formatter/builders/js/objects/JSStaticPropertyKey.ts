/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@internal/formatter";

import {JSStaticPropertyKey} from "@internal/ast";
import {isValidIdentifierName} from "@internal/js-ast-utils";

export default function JSStaticPropertyKey(
	builder: Builder,
	node: JSStaticPropertyKey,
): Token {
	// if (node.type === "JSStringLiteral") {
	// 	console.log(node, isValidIdentifierName(node.value));
	// }

	// if (
	// 	node.value.loc?.filename ===
	// 	"project-rome/@internal/formatter/builders/js/objects/JSStaticPropertyKey.ts"
	// ) {
	// 	const foo = {
	// 		"bar": true,
	// 	};
	// 	console.log(node);
	// 	console.log(foo["bar"]);
	// }

	if (node.value.type === 'JSStringLiteral' && isValidIdentifierName(node.value.value)) {
		return node.value.value;
	}

	return builder.tokenize(node.value, node);
}
