/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSInferType} from "@romefrontend/ast";
import {Builder, Token, concat, space} from "@romefrontend/formatter";

export default function TSInferType(builder: Builder, node: TSInferType): Token {
	return concat(["infer", space, builder.tokenize(node.typeParameter, node)]);
}
