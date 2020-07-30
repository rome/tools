/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	JSRegExpNonWordCharacter,
	jsRegExpNonWordCharacter,
} from "@internal/ast";

export default function JSRegExpNonWordCharacter(node: AnyNode) {
	node = jsRegExpNonWordCharacter.assert(node);
	throw new Error("unimplemented");
}
