/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	NullKeywordTypeAnnotation,
	nullKeywordTypeAnnotation,
} from "@romejs/js-ast";
import NullT from "../../types/NullT";

export default function NullKeywordTypeAnnotation(node: AnyNode, scope: Scope) {
	node = nullKeywordTypeAnnotation.assert(node);
	return new NullT(scope, node);
}
