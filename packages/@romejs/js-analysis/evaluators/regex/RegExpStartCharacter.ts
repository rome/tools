/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	RegExpStartCharacter,
	regExpStartCharacter,
} from "@romejs/js-ast";

export default function RegExpStartCharacter(node: AnyNode) {
	node = regExpStartCharacter.assert(node);
	throw new Error("unimplemented");
}
