/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {isPlainObject} from "@internal/typescript-helpers";

export function isNodeLike(node: unknown): boolean {
	if (node == null) {
		return false;
	} else {
		return isPlainObject(node) && typeof node.type === "string";
	}
}
