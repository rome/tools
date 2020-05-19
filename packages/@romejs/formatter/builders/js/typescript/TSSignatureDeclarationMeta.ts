/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Builder, Token, concat} from "@romejs/formatter";

import {TSSignatureDeclarationMeta} from "@romejs/ast";
import {printBindingPatternParams} from "../utils";

export default function TSSignatureDeclarationMeta(
	builder: Builder,
	node: TSSignatureDeclarationMeta,
): Token {
	return concat([
		builder.tokenize(node.typeParameters, node),
		printBindingPatternParams(builder, node, node.parameters, node.rest),
	]);
}
