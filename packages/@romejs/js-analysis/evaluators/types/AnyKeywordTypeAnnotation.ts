/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyKeywordTypeAnnotation,
	AnyNode,
	anyKeywordTypeAnnotation,
} from "@romejs/js-ast";
import AnyT from "../../types/AnyT";

export default function AnyKeywordTypeAnnotation(node: AnyNode, scope: Scope) {
	node = anyKeywordTypeAnnotation.assert(node);
	return new AnyT(scope, node);
}
