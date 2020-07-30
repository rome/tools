/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat, group, space} from "@internal/formatter";
import {TSTypeAlias} from "@internal/ast";

export default function TSTypeAlias(builder: Builder, node: TSTypeAlias): Token {
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
