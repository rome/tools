/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {
	AnyNode,
	JSRegExpNamedBackReference,
	jsRegExpNamedBackReference,
} from "@internal/ast";

export default function JSRegExpNamedBackReference(node: AnyNode) {
	node = jsRegExpNamedBackReference.assert(node);
	throw new Error("unimplemented");
}
