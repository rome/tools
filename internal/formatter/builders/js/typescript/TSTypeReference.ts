/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypeReference} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function TSTypeReference(
	builder: Builder,
	node: TSTypeReference,
): Token {
	return concat([
		builder.tokenize(node.typeName, node),
		builder.tokenize(node.typeParameters, node),
	]);
}
