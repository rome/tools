/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	JSRegExpNonDigitCharacter,
	jsRegExpNonDigitCharacter,
} from "@internal/ast";

export default function JSRegExpNonDigitCharacter(node: AnyNode) {
	node = jsRegExpNonDigitCharacter.assert(node);
	throw new Error("unimplemented");
}
