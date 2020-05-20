/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import isTypeNode from "./isTypeNode";
import isTypeExpressionWrapperNode from "./isTypeExpressionWrapperNode";

// Is this honestly the best heuristics?
function getTypeNode(path: Path): undefined | AnyNode {
	const {parent, parentPath} = path;
	if (parent === undefined || parentPath === undefined) {
		return undefined;
	}

	if (isTypeNode(parent)) {
		return parent;
	}

	if (isTypeNode(parentPath.parent)) {
		return parentPath.parent;
	}

	return undefined;
}

export default function isInTypeAnnotation(path: Path): boolean {
	const match = getTypeNode(path);
	if (match === undefined) {
		return false;
	}

	if (isTypeExpressionWrapperNode(match)) {
		return false;
	} else {
		return true;
	}
}
