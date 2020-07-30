/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {
	AnyNode,
	JSRegExpNumericBackReference,
	jsRegExpNumericBackReference,
} from "@internal/ast";

export default function JSRegExpNumericBackReference(node: AnyNode) {
	node = jsRegExpNumericBackReference.assert(node);
	throw new Error("unimplemented");
}
