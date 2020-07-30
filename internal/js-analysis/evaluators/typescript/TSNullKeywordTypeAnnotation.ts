/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	TSNullKeywordTypeAnnotation,
	tsNullKeywordTypeAnnotation,
} from "@internal/ast";
import NullT from "../../types/NullT";

export default function TSNullKeywordTypeAnnotation(node: AnyNode, scope: Scope) {
	node = tsNullKeywordTypeAnnotation.assert(node);
	return new NullT(scope, node);
}
