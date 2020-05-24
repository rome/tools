/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romejs/ast";
import isIdentifierish from "./isIdentifierish";

type Parts = Array<{
	value: string;
	node: AnyNode;
}>;

type Result = {
	bailed: boolean;
	parts: Parts;
};

const cache: WeakMap<AnyNode, Result> = new WeakMap();

const EMPTY: Result = {
	bailed: true,
	parts: [],
};

export default function getNodeReferenceParts(node: undefined | AnyNode): Result {
	if (node === undefined) {
		return EMPTY;
	}

	const cached = cache.get(node);
	if (cached !== undefined) {
		return cached;
	}

	const parts: Parts = [];

	function add(node: AnyNode): boolean {
		if (isIdentifierish(node)) {
			parts.push({node, value: node.name});
			return false;
		} else if (node.type === "JSThisExpression") {
			parts.push({node, value: "this"});
			return false;
		} else if (node.type === "JSStringLiteral") {
			parts.push({node, value: node.value});
			return false;
		} else if (node.type === "JSMetaProperty") {
			parts.push({node, value: node.meta.name});
			parts.push({node, value: node.property.name});
			return false;
		} else if (
			node.type === "JSMemberExpression" ||
			node.type === "JSXMemberExpression"
		) {
			const stop = add(node.object);
			if (stop) {
				return true;
			} else {
				return add(node.property);
			}
		} else if (
			node.type === "JSComputedMemberProperty" &&
			node.value.type === "JSStringLiteral"
		) {
			return add(node.value);
		} else if (node.type === "JSStaticMemberProperty") {
			return add(node.value);
		} else {
			return true;
		}
	}

	const bailed = add(node);
	const result: Result = {bailed, parts};
	cache.set(node, result);
	return result;
}
