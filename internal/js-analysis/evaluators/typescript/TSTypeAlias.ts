/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, TSTypeAlias, tsTypeAlias} from "@internal/ast";

export default function TSTypeAlias(node: AnyNode, scope: Scope) {
	node = tsTypeAlias.assert(node);

	const typeScope = scope.fork();
	if (node.typeParameters) {
		typeScope.evaluate(node.typeParameters);
	}

	const right = typeScope.evaluate(node.right);
	scope.addBinding(node.id.name, right);
	return right;
}
