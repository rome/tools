/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	TSTypeParameterDeclaration,
	tsTypeParameterDeclaration,
} from "@romejs/ast";

export default function TSTypeParameterDeclaration(node: AnyNode) {
	node = tsTypeParameterDeclaration.assert(node);
	throw new Error("unimplemented");
}
