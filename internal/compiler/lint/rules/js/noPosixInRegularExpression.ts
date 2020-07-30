/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSRegExpCharSet} from "@internal/ast";
import {CompilerContext, createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

function checkRegEx(
	node: JSRegExpCharSet,
	context: CompilerContext,
): JSRegExpCharSet {
	for (let i = 0; i < node.body.length; i++) {
		const nextNode = node.body[i + 1];
		const currNode = node.body[i];
		const lastNode = node.body[node.body.length - 1];
		if (
			currNode.type === "JSRegExpCharacter" &&
			currNode.value === "[" &&
			nextNode &&
			nextNode.type === "JSRegExpCharacter" &&
			(nextNode.value === ":" || nextNode.value === ".") &&
			lastNode.type === "JSRegExpCharacter" &&
			lastNode.value === nextNode.value
		) {
			context.addNodeDiagnostic(
				currNode,
				descriptions.LINT.JS_NO_POSIX_IN_REGULAR_EXPRESSION,
				{tags: {fixable: true}},
			);
		}
	}

	return node;
}

export default createVisitor({
	name: "js/noPosixInRegularExpression",
	enter(path) {
		const {context, node} = path;

		if (node.type === "JSRegExpCharSet" && node.body.length > 2) {
			return signals.replace(checkRegEx(node, context));
		}

		return signals.retain;
	},
});
