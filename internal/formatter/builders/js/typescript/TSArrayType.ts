/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSArrayType} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function TSArrayType(builder: Builder, node: TSArrayType): Token {
	return concat([builder.tokenize(node.elementType, node), "[]"]);
}
