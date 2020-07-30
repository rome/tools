/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSFunction, AnyNode} from "@internal/ast";

export function isFunctionNode(node: AnyNode): node is AnyJSFunction {
	return (
		node.type === "JSFunctionDeclaration" ||
		node.type === "JSFunctionExpression" ||
		node.type === "JSObjectMethod" ||
		node.type === "JSArrowFunctionExpression" ||
		node.type === "JSClassMethod"
	);
}
