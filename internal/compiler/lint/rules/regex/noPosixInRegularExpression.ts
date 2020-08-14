/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

export default createVisitor({
	name: "regex/noPosixInRegularExpression",
	enter(path) {
		const {context, node} = path;

		if (node.type === "JSRegExpCharSet" && node.body.length > 2) {
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
						descriptions.LINT.REGEX_NO_POSIX_IN_REGULAR_EXPRESSION,
					);
				}
			}
		}

		return signals.retain;
	},
});
