/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	TSNeverKeywordTypeAnnotation,
	tsNeverKeywordTypeAnnotation,
} from "@internal/ast";

export default function TSNeverKeywordTypeAnnotation(node: AnyNode) {
	node = tsNeverKeywordTypeAnnotation.assert(node);
	throw new Error("unimplemented");
}
