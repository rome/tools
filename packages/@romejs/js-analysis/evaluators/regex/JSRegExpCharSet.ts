/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, JSRegExpCharSet, jsRegExpCharSet} from "@romejs/ast";

export default function JSRegExpCharSet(node: AnyNode) {
	node = jsRegExpCharSet.assert(node);
	throw new Error("unimplemented");
}
