/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, ObjectExpression, objectExpression} from "@romejs/js-ast";
import ObjPropT from "../../types/ObjPropT";
import ObjT from "../../types/ObjT";

export default function ObjectExpression(node: AnyNode, scope: Scope) {
	node = objectExpression.assert(node);
	const props = [];

	for (const prop of node.properties) {
		if (prop.type === "SpreadProperty") {
			// TODO
		} else if (prop.type === "ObjectProperty") {
			if (prop.key.type === "ComputedPropertyKey") {
				// TODO
			} else {
				const {
					key: {value: key},
					value,
				} = prop;

				let keyStr;
				if (key.type === "Identifier") {
					keyStr = key.name;
				} else {
					// TODO
					continue;
				}

				if (keyStr === undefined) {
					throw new Error("Expected keyStr");
				}

				props.push(new ObjPropT(scope, prop, keyStr, scope.evaluate(value)));
			}
		} else {
			// TODO
		}
	}

	return new ObjT(
		scope,
		node,
		{
			calls: [],
			props,
			proto: scope.intrinsics.ObjectPrototype,
		},
	);
}
