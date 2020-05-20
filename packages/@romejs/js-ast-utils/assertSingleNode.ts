/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TransformExitResult} from "@romejs/compiler";
import {AnyNode} from "@romejs/ast";

export default function assertSingleNode(result: TransformExitResult): AnyNode {
	if (Array.isArray(result)) {
		if (result.length !== 1) {
			throw new Error(`Expected node list length of 1 but got ${result.length}`);
		}
		return result[0];
	} else if (result === undefined) {
		throw new Error("Expected node or node list but got null");
	} else if (typeof result === "symbol") {
		throw new Error("No symbols expected here");
	} else {
		return result;
	}
}
