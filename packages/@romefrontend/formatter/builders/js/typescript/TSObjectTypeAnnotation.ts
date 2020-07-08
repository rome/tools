/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSObjectTypeAnnotation} from "@romefrontend/ast";
import {Builder, Token} from "@romefrontend/formatter";

import {printTSBraced} from "../utils";

export default function TSObjectTypeAnnotation(
	builder: Builder,
	node: TSObjectTypeAnnotation,
): Token {
	return printTSBraced(builder, node, node.members);
}
