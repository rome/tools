/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSIdentifier, AnyNode} from "@internal/ast";

export function isIdentifierish(node: AnyNode): node is AnyJSIdentifier {
	return (
		node.type === "JSIdentifier" ||
		node.type === "JSXIdentifier" ||
		node.type === "JSXReferenceIdentifier" ||
		node.type === "JSBindingIdentifier" ||
		node.type === "JSAssignmentIdentifier" ||
		node.type === "JSReferenceIdentifier"
	);
}
