/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	JSRegExpWordCharacter,
	jsRegExpWordCharacter,
} from "@internal/ast";

export default function JSRegExpWordCharacter(node: AnyNode) {
	node = jsRegExpWordCharacter.assert(node);
	throw new Error("unimplemented");
}
