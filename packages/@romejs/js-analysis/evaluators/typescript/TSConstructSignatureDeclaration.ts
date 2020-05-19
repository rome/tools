/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	TSConstructSignatureDeclaration,
	tsConstructSignatureDeclaration,
} from "@romejs/ast";

export default function TSConstructSignatureDeclaration(node: AnyNode) {
	node = tsConstructSignatureDeclaration.assert(node);
	throw new Error("unimplemented");
}
