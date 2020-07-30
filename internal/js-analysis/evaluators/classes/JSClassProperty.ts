/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ClassScope, Scope, ThisScope} from "../../scopes";
import {AnyNode, JSClassProperty, jsClassProperty} from "@internal/ast";
import AnyT from "../../types/AnyT";
import ObjPropT from "../../types/ObjPropT";

export default function JSClassProperty(node: AnyNode, scope: Scope) {
	node = jsClassProperty.assert(node);

	if (node.key.type === "JSComputedPropertyKey") {
		// TODO
		return undefined;
	}

	const classScope = scope.find(ClassScope);
	const funcScope = new ThisScope(
		{parentScope: scope},
		classScope.meta.instance,
	);

	let annotatedType;
	let inferredType;

	if (node.typeAnnotation) {
		annotatedType = funcScope.evaluate(node.typeAnnotation);
	}

	if (node.value) {
		inferredType = funcScope.evaluate(node.value);

		if (annotatedType !== undefined) {
			inferredType.shouldMatch(annotatedType);
		}
	}

	if (annotatedType === undefined && inferredType === undefined) {
		// TODO what do we do here?
		inferredType = new AnyT(scope, node);
	}

	const actualValue = annotatedType === undefined ? inferredType : annotatedType;
	if (actualValue === undefined) {
		throw new Error("Expected actual value");
	}

	if (node.key.value.type !== "JSIdentifier") {
		throw new Error("Expected only an jsIdentifier key");
	}

	return new ObjPropT(scope, node, node.key.value.name, actualValue);
}
