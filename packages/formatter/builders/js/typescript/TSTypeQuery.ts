/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypeQuery} from "@romefrontend/ast";
import {Builder, Token, concat, space} from "@romefrontend/formatter";

export default function TSTypeQuery(builder: Builder, node: TSTypeQuery): Token {
	return concat(["typeof", space, builder.tokenize(node.exprName, node)]);
}
