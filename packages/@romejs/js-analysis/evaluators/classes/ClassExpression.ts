/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ClassScope, Scope} from "../../scopes";
import {AnyNode, ClassExpression, classExpression} from "@romejs/js-ast";
import InstanceT from "../../types/InstanceT";
import ClassT from "../../types/ClassT";
import T from "../../types/T";
import OpenT from "../../types/OpenT";

export default function ClassExpression(node: AnyNode, scope: Scope) {
	node = node.type === "ClassDeclaration" ? node : classExpression.assert(node);

	const instances = [];
	const statics = [];

	//
	const classInstance = new OpenT(scope, node);
	const classId = new OpenT(scope, node);

	//
	const bodyScope = new ClassScope(
		{parentScope: scope},
		{
			instance: classInstance,
			static: classId,
		},
	);

	if (node.id !== undefined) {
		bodyScope.addBinding(node.id.name, classId);
	}

	if (node.meta.typeParameters !== undefined) {
		bodyScope.evaluate(node.meta.typeParameters);
	}

	let _constructor: undefined | T = undefined;

	for (const bodyNode of node.meta.body) {
		const type = bodyScope.evaluate(bodyNode);

		if (bodyNode.type === "ClassMethod" && bodyNode.kind === "constructor") {
			_constructor = type;
		} else {
			if (bodyNode.type !== "TSIndexSignature" && bodyNode.meta.static === true) {
				statics.push(type);
			} else {
				instances.push(type);
			}
		}
	}

	//
	const classOrigin = node.id ? node.id : node;
	let type = new ClassT(
		scope,
		classOrigin,
		{
			_constructor,
			instances,
			statics,
			extends: node.meta.superClass
				? scope.evaluate(node.meta.superClass)
				: undefined,
		},
	);
	if (node.id) {
		type.setHuman(node.id.name);
	}

	//
	classId.shouldMatch(type);

	//
	const instance = new InstanceT(scope, classOrigin, type, []);
	classInstance.shouldMatch(instance);

	return type;
}
