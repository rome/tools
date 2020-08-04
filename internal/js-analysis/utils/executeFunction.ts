/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSFunction} from "@internal/ast";
import {FunctionScope, Scope} from "../scopes";
import T from "../types/T";
import executeAtom from "./executeAtom";
import FunctionT from "../types/FunctionT";
import MaybeT from "../types/MaybeT";
import VoidT from "../types/VoidT";
import OpenT from "../types/OpenT";

export default function executeFunction(
	node: AnyJSFunction,
	scope: Scope,
	bindId: boolean,
	thisContext?: T,
): FunctionT {
	const {head} = node;

	// build return type
	const returns = new OpenT(scope, head.returnType ? head.returnType : node);

	// type check the body
	const bodyScope = new FunctionScope(
		{
			parentScope: scope,
		},
		{
			thisContext: thisContext ? thisContext : new VoidT(scope, undefined),
			returnType: returns,
		},
	);
	if (head.typeParameters) {
		bodyScope.evaluate(head.typeParameters);
	}

	// build param types
	const params = [];
	let rest;
	for (let paramNode of head.params) {
		let optional =
			paramNode.meta !== undefined && paramNode.meta.optional === true;
		if (paramNode.type === "JSBindingAssignmentPattern") {
			optional = false;
			paramNode = paramNode.left;
		}

		let paramType;
		if (
			paramNode.meta !== undefined &&
			paramNode.meta.typeAnnotation !== undefined
		) {
			paramType = scope.evaluate(paramNode.meta.typeAnnotation);
		} else {
			paramType = new OpenT(scope, paramNode);
		}

		if (optional) {
			paramType = new MaybeT(scope, paramNode, paramType);
		}

		params.push(paramType);
	}

	for (let i = 0; i < head.params.length; i++) {
		executeAtom(head.params[i], params[i], scope);
	}
	const block = bodyScope.evaluate(node.body);

	// if no types have flowed into the return type then it'll return undefined
	if (!returns.hasConnections()) {
		//const ret = new VoidT(scope, node);
		//returns.shouldMatch(ret);
	}

	if (head.returnType) {
		returns.shouldMatch(scope.evaluate(head.returnType));
	}

	// create the function
	return new FunctionT(scope, node, {params, rest, returns, body: block});
}
