/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@romefrontend/formatter";
import {JSPrivateName} from "@romefrontend/ast";

export default function JSPrivateName(
	builder: Builder,
	node: JSPrivateName,
): Token {
	return concat(["#", builder.tokenize(node.id, node)]);
}
