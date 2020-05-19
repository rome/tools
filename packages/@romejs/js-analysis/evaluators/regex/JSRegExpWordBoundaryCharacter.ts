/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	JSRegExpWordBoundaryCharacter,
	jsRegExpWordBoundaryCharacter,
} from "@romejs/ast";

export default function JSRegExpWordBoundaryCharacter(node: AnyNode) {
	node = jsRegExpWordBoundaryCharacter.assert(node);
	throw new Error("unimplemented");
}
