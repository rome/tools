/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {
	AnyNode,
	JSVariableDeclaration,
	jsVariableDeclaration,
} from "@internal/ast";
import OpenT from "../../types/OpenT";
import VoidT from "../../types/VoidT";
import executeAtom from "../../utils/executeAtom";

export default function JSVariableDeclaration(node: AnyNode, scope: Scope) {
	node = jsVariableDeclaration.assert(node);

	for (const declarator of node.declarations) {
		const {id, init} = declarator;
		let inferredType;

		if (init === undefined) {
			inferredType = new OpenT(scope, declarator);
			inferredType.shouldMatch(new VoidT(scope, declarator));
		} else {
			inferredType = scope.evaluate(init);
		}

		let actualType = inferredType;

		if (id.meta !== undefined && id.meta.typeAnnotation !== undefined) {
			const annotatedType = scope.evaluate(id.meta.typeAnnotation);
			inferredType.shouldMatch(annotatedType);
			actualType = annotatedType;
		}

		executeAtom(id, actualType, scope);
	}
}
