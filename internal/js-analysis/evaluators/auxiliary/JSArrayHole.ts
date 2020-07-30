/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {AnyNode, JSArrayHole, jsArrayHole} from "@internal/ast";

export default function JSArrayHole(node: AnyNode) {
	node = jsArrayHole.assert(node);
	throw new Error("unimplemented");
}
