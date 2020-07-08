/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypeParameterInstantiation} from "@romefrontend/ast";
import {Builder, Token} from "@romefrontend/formatter";

import TSTypeParameterDeclaration from "./TSTypeParameterDeclaration";

export default function TSTypeParameterInstantiation(
	builder: Builder,
	node: TSTypeParameterInstantiation,
): Token {
	return TSTypeParameterDeclaration(builder, node);
}
