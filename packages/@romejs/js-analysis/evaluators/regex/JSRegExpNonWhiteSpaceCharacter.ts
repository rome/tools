/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	JSRegExpNonWhiteSpaceCharacter,
	jsRegExpNonWhiteSpaceCharacter,
} from "@romejs/ast";

export default function JSRegExpNonWhiteSpaceCharacter(node: AnyNode) {
	node = jsRegExpNonWhiteSpaceCharacter.assert(node);
	throw new Error("unimplemented");
}
