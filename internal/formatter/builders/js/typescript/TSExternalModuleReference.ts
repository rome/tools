/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSExternalModuleReference} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function TSExternalModuleReference(
	builder: Builder,
	node: TSExternalModuleReference,
): Token {
	return concat(["require(", builder.tokenize(node.expression, node), ")"]);
}
