/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	TSUnionTypeAnnotation,
	tsUnionTypeAnnotation,
} from "@internal/ast";
import UnionT from "../../types/UnionT";

export default function TSUnionTypeAnnotation(node: AnyNode, scope: Scope) {
	node = tsUnionTypeAnnotation.assert(node);

	return new UnionT(
		scope,
		node,
		node.types.map((type) => {
			return scope.evaluate(type);
		}),
	);
}
