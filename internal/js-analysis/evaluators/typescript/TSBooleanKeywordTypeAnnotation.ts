/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	TSBooleanKeywordTypeAnnotation,
	tsBooleanKeywordTypeAnnotation,
} from "@internal/ast";
import BooleanT from "../../types/BooleanT";

export default function TSBooleanKeywordTypeAnnotation(
	node: AnyNode,
	scope: Scope,
) {
	node = tsBooleanKeywordTypeAnnotation.assert(node);
	return new BooleanT(scope, node);
}
