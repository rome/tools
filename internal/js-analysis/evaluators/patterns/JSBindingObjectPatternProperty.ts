/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	JSBindingObjectPatternProperty,
	jsBindingObjectPatternProperty,
} from "@internal/ast";

export default function JSBindingObjectPatternProperty(node: AnyNode) {
	node = jsBindingObjectPatternProperty.assert(node);
	throw new Error("unimplemented");
}
