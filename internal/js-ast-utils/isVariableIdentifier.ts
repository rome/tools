/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSVariableIdentifier, AnyNode} from "@internal/ast";

export function isVariableIdentifier(
	node: AnyNode,
): node is AnyJSVariableIdentifier {
	return (
		node.type === "JSBindingIdentifier" ||
		node.type === "JSAssignmentIdentifier" ||
		node.type === "JSReferenceIdentifier" ||
		node.type === "JSXReferenceIdentifier"
	);
}
