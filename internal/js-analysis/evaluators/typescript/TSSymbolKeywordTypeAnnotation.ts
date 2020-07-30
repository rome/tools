/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	TSSymbolKeywordTypeAnnotation,
	tsSymbolKeywordTypeAnnotation,
} from "@internal/ast";

export default function TSSymbolKeywordTypeAnnotation(node: AnyNode) {
	node = tsSymbolKeywordTypeAnnotation.assert(node);
	throw new Error("unimplemented");
}
