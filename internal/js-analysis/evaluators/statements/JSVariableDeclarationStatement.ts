/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	JSVariableDeclarationStatement,
	jsVariableDeclarationStatement,
} from "@internal/ast";

export default function JSVariableDeclarationStatement(node: AnyNode) {
	node = jsVariableDeclarationStatement.assert(node);
	throw new Error("unimplemented");
}
