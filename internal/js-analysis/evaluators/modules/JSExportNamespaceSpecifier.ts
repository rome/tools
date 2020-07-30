/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	JSExportNamespaceSpecifier,
	jsExportNamespaceSpecifier,
} from "@internal/ast";

export default function JSExportNamespaceSpecifier(node: AnyNode, scope: Scope) {
	node = jsExportNamespaceSpecifier.assert(node);
	scope;
	throw new Error("unimplemented");
}
