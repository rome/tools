/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	IntersectionTypeAnnotation,
	intersectionTypeAnnotation,
} from "@romejs/js-ast";
import IntersectionT from "../../types/IntersectionT";

export default function IntersectionTypeAnnotation(node: AnyNode, scope: Scope) {
	node = intersectionTypeAnnotation.assert(node);

	return new IntersectionT(
		scope,
		node,
		node.types.map((type) => {
			return scope.evaluate(type);
		}),
	);
}
