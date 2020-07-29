/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";

const commentPattern = /(^(\/\*\*|\/\*|\/\/)|\*\/$)/gm;

export default createVisitor({
	name: "jsx/noCommentText",
	enter(path) {
		const {node} = path;

		if (node.type === "JSXText") {
			if (commentPattern.test(node.value)) {
				path.addFixableDiagnostic(
					{
						fixed: signals.replace({
							...node,
							// TODO: This is exploiting JSXText and should be using properly create an expression container
							// and inject the comment
							value: `{/**${node.value.replace(commentPattern, "")}*/}`,
						}),
					},
					descriptions.LINT.JSX_NO_COMMENT_TEXT,
				);
			}
		}

		return signals.retain;
	},
});
