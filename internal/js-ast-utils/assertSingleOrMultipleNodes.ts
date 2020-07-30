/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, AnyNodes} from "@internal/ast";

export function assertSingleOrMultipleNodes(
	result: AnyNodes,
): AnyNode | Array<AnyNode> {
	if (result === undefined) {
		throw new Error("Expected node or node list but got null");
	} else if (typeof result === "symbol") {
		throw new Error("No symbols expected here");
	} else {
		return result;
	}
}
