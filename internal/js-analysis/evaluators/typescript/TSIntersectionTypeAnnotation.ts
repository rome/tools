/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	TSIntersectionTypeAnnotation,
	tsIntersectionTypeAnnotation,
} from "@internal/ast";
import IntersectionT from "../../types/IntersectionT";

export default function TSIntersectionTypeAnnotation(
	node: AnyNode,
	scope: Scope,
) {
	node = tsIntersectionTypeAnnotation.assert(node);

	return new IntersectionT(
		scope,
		node,
		node.types.map((type) => {
			return scope.evaluate(type);
		}),
	);
}
