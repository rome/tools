/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSExpressionWithTypeArguments} from "@romefrontend/ast";
import {Builder, Token, concat} from "@romefrontend/formatter";

export default function TSExpressionWithTypeArguments(
	builder: Builder,
	node: TSExpressionWithTypeArguments,
): Token {
	return concat([
		builder.tokenize(node.expression, node),
		builder.tokenize(node.typeParameters, node),
	]);
}
