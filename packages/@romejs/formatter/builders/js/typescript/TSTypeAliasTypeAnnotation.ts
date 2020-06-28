/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, group, space} from "@romejs/formatter";
import {TSTypeAliasTypeAnnotation} from "@romejs/ast";

export default function TSTypeAliasTypeAnnotation(
	builder: Builder,
	node: TSTypeAliasTypeAnnotation,
): Token {
	return group(
		concat([
			"type",
			space,
			builder.tokenize(node.id, node),
			builder.tokenize(node.typeParameters, node),
			space,
			"=",
			space,
			builder.tokenize(node.right, node),
			";",
		]),
	);
}
