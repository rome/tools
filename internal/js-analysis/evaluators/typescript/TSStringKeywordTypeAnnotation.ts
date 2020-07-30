/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	TSStringKeywordTypeAnnotation,
	tsStringKeywordTypeAnnotation,
} from "@internal/ast";
import StringT from "../../types/StringT";

export default function TSStringKeywordTypeAnnotation(
	node: AnyNode,
	scope: Scope,
) {
	node = tsStringKeywordTypeAnnotation.assert(node);
	return new StringT(scope, node);
}
