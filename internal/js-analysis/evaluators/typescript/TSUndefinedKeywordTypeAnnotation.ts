/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	TSUndefinedKeywordTypeAnnotation,
	tsUndefinedKeywordTypeAnnotation,
} from "@internal/ast";

export default function TSUndefinedKeywordTypeAnnotation(node: AnyNode) {
	node = tsUndefinedKeywordTypeAnnotation.assert(node);
	throw new Error("unimplemented");
}
