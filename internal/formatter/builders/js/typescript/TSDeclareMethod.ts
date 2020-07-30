/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSDeclareMethod} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

import {printMethod} from "../utils";

export default function TSDeclareMethod(
	builder: Builder,
	node: TSDeclareMethod,
): Token {
	return concat([
		builder.tokenize(node.meta, node),
		builder.tokenize(node.key, node),
		printMethod(builder, node),
	]);
}
