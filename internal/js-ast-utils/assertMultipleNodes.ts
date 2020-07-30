/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, AnyNodes} from "@internal/ast";

export function assertMultipleNodes(result: AnyNodes): Array<AnyNode> {
	if (Array.isArray(result)) {
		return result;
	} else if (result === undefined) {
		return [];
	} else {
		return [result];
	}
}
