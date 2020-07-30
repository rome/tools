/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	JSExportAllDeclaration,
	jsExportAllDeclaration,
} from "@internal/ast";
import Hub from "../../Hub";

export default function JSExportAllDeclaration(
	node: AnyNode,
	scope: Scope,
	{evaluator}: Hub,
) {
	node = jsExportAllDeclaration.assert(node);
	evaluator.addExportAll(node.source.value);
}
