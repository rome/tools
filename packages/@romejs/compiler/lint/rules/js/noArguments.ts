/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@romejs/ast";
import {Path} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";

export default {
	name: "noArguments",
	enter(path: Path): AnyNode {
		const {node, scope} = path;

		if (node.type === "JSReferenceIdentifier" && node.name === "arguments") {
			const args = scope.getBinding("arguments");
			if (args && args.kind === "arguments") {
				path.context.addNodeDiagnostic(node, descriptions.LINT.JS_NO_ARGUMENTS);
			}
		}

		return node;
	},
};
