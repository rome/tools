/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypeOperator} from "@romefrontend/ast";
import {Builder, Token, concat, space} from "@romefrontend/formatter";

export default function TSTypeOperator(
	builder: Builder,
	node: TSTypeOperator,
): Token {
	return concat([
		node.operator,
		space,
		builder.tokenize(node.typeAnnotation, node),
	]);
}
