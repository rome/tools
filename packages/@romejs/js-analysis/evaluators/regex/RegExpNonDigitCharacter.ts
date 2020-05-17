/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	RegExpNonDigitCharacter,
	regExpNonDigitCharacter,
} from "@romejs/js-ast";

export default function RegExpNonDigitCharacter(node: AnyNode) {
	node = regExpNonDigitCharacter.assert(node);
	throw new Error("unimplemented");
}
