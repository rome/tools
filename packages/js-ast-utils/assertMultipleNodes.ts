/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TransformExitResult} from "@romefrontend/compiler";
import {AnyNode} from "@romefrontend/ast";

export function assertMultipleNodes(result: TransformExitResult): Array<AnyNode> {
	if (Array.isArray(result)) {
		return result;
	} else if (result === undefined) {
		return [];
	} else if (typeof result === "symbol") {
		throw new Error("No symbols expected here");
	} else {
		return [result];
	}
}
