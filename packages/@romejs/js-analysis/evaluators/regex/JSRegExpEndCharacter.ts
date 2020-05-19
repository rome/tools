/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, JSRegExpEndCharacter, jsRegExpEndCharacter} from "@romejs/ast";

export default function JSRegExpEndCharacter(node: AnyNode) {
	node = jsRegExpEndCharacter.assert(node);
	throw new Error("unimplemented");
}
