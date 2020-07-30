/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	TSMixedKeywordTypeAnnotation,
	tsMixedKeywordTypeAnnotation,
} from "@internal/ast";
import MixedT from "../../types/MixedT";

export default function TSMixedKeywordTypeAnnotation(
	node: AnyNode,
	scope: Scope,
) {
	node = tsMixedKeywordTypeAnnotation.assert(node);
	return new MixedT(scope, node);
}
