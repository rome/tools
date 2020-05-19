/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	JSAssignmentAssignmentPattern,
	jsAssignmentAssignmentPattern,
} from "@romejs/ast";

export default function JSAssignmentAssignmentPattern(node: AnyNode) {
	node = jsAssignmentAssignmentPattern.assert(node);
	throw new Error("unimplemented");
}
