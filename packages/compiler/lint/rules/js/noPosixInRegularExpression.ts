/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, JSRegExpCharSet} from "@romefrontend/ast";
import {CompilerContext, Path} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";

function checkRegEx(
	node: JSRegExpCharSet,
	context: CompilerContext,
): JSRegExpCharSet {
	node.body.forEach((currNode, i) => {
		const nextNode = node.body[i + 1];
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
				{fixable: false},
			);
		}
	});

	return node;
}

export default {
	name: "js/noPosixInRegularExpression",
	enter(path: Path): AnyNode {
		const {context, node} = path;

		if (node.type === "JSRegExpCharSet" && node.body.length > 2) {
			return checkRegEx(node, context);
		}

		return node;
	},
};
