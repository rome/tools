/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token} from "@romejs/formatter";
import {JSArrayHole} from "@romejs/ast";

export default function JSArrayHole(builder: Builder, node: JSArrayHole): Token {
	return builder.tokenizeInnerComments(node, false);
}
