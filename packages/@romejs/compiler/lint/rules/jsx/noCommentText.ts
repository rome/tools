/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/compiler";
import {AnyNode} from "@romejs/ast";
import {descriptions} from "@romejs/diagnostics";

const commentPattern = /(^(\/\*\*|\/\*|\/\/)|\*\/$)/gm;

export default {
	name: "jsxNoCommentText",
	enter(path: Path): AnyNode {
		const {node} = path;

		if (node.type === "JSXText") {
			if (commentPattern.test(node.value)) {
				path.context.addFixableDiagnostic(
					{
						old: node,
						fixed: {
							...node,
							value: `{/**${node.value.replace(commentPattern, "")}*/}`,
						},
					},
					descriptions.LINT.JSX_NO_COMMENT_TEXT,
				);
			}
		}

		return node;
	},
};
