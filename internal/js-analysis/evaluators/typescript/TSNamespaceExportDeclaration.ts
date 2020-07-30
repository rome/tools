/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	TSNamespaceExportDeclaration,
	tsNamespaceExportDeclaration,
} from "@internal/ast";

export default function TSNamespaceExportDeclaration(node: AnyNode) {
	node = tsNamespaceExportDeclaration.assert(node);
	throw new Error("unimplemented");
}
