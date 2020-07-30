/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	TSVoidKeywordTypeAnnotation,
	tsVoidKeywordTypeAnnotation,
} from "@internal/ast";
import VoidT from "../../types/VoidT";

export default function TSVoidKeywordTypeAnnotation(node: AnyNode, scope: Scope) {
	node = tsVoidKeywordTypeAnnotation.assert(node);
	return new VoidT(scope, node);
}
