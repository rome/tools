/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSFunctionType} from "@romejs/ast";
import {Builder, Token, concat, group, space} from "@romejs/formatter";

export default function TSFunctionType(
	builder: Builder,
	node: TSFunctionType,
): Token {
	return group(
		concat([
			builder.tokenize(node.meta, node),
			space,
			"=>",
			space,
			builder.tokenize(node.typeAnnotation, node),
		]),
	);
}
