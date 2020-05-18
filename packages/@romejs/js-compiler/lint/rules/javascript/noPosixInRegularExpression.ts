/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, RegExpCharSet} from "@romejs/js-ast";
import {CompilerContext, Path} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";

function checkRegEx(
	node: RegExpCharSet,
	context: CompilerContext,
): RegExpCharSet {
	node.body.forEach((currNode, i) => {
		const nextNode = node.body[i + 1];
		const lastNode = node.body[node.body.length - 1];
		if (
			currNode.type === "RegExpCharacter" &&
			currNode.value === "[" &&
			nextNode &&
			nextNode.type === "RegExpCharacter" &&
			(nextNode.value === ":" || nextNode.value === ".") &&
			lastNode.type === "RegExpCharacter" &&
			lastNode.value === nextNode.value
		) {
			context.addNodeDiagnostic(
				currNode,
				descriptions.LINT.JAVASCRIPT_NO_POSIX_IN_REGULAR_EXPRESSION,
				{fixable: false},
			);
		}
	});

	return node;
}

export default {
	name: "noPosixInRegularExpression",
	enter(path: Path): AnyNode {
		const {context, node} = path;

		if (node.type === "RegExpCharSet" && node.body.length > 2) {
			return checkRegEx(node, context);
		}

		return node;
	},
};
