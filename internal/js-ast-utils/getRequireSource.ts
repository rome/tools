/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "@internal/compiler";
import {AnyNode} from "@internal/ast";
import {doesNodeMatchPattern} from "./doesNodeMatchPattern";

export function getRequireSource(
	node: undefined | AnyNode,
	scope: Scope,
	allowStaticMember: boolean = false,
): undefined | string {
	if (node === undefined) {
		return undefined;
	}

	if (
		allowStaticMember &&
		node.type === "JSMemberExpression" &&
		node.property.type === "JSStaticMemberProperty"
	) {
		node = node.object;
	}

	if (node.type !== "JSCallExpression") {
		return undefined;
	}

	const {arguments: args, callee} = node;

	const [firstArg] = args;
	if (args.length !== 1 || firstArg.type !== "JSStringLiteral") {
		return undefined;
	}

	const validRequireCallee =
		callee.type === "JSReferenceIdentifier" &&
		callee.name === "require" &&
		scope.getBinding("require") === undefined;

	const validRomeRequreCallee =
		(doesNodeMatchPattern(callee, "Rome.requireDefault") ||
		doesNodeMatchPattern(callee, "Rome.requireNamespace")) &&
		scope.getBinding("Rome") === undefined;

	if (validRequireCallee || validRomeRequreCallee) {
		return firstArg.value;
	}

	return undefined;
}
