/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	TSAnyKeywordTypeAnnotation,
	tsAnyKeywordTypeAnnotation,
} from "@internal/ast";
import AnyT from "../../types/AnyT";

export default function TSAnyKeywordTypeAnnotation(node: AnyNode, scope: Scope) {
	node = tsAnyKeywordTypeAnnotation.assert(node);
	return new AnyT(scope, node);
}
