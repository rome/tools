/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@romefrontend/formatter";

import {JSTaggedTemplateExpression} from "@romefrontend/ast";

export default function JSTaggedTemplateExpression(
	builder: Builder,
	node: JSTaggedTemplateExpression,
): Token {
	return concat([
		builder.tokenize(node.tag, node),
		builder.tokenize(node.quasi, node),
	]);
}
