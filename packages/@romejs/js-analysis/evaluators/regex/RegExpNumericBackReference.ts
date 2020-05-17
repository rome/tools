/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {
	AnyNode,
	RegExpNumericBackReference,
	regExpNumericBackReference,
} from "@romejs/js-ast";

export default function RegExpNumericBackReference(node: AnyNode) {
	node = regExpNumericBackReference.assert(node);
	throw new Error("unimplemented");
}
