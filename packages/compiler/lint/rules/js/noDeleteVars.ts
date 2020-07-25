/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romefrontend/compiler";
import {AnyNode} from "@romefrontend/ast";
import {descriptions} from "@romefrontend/diagnostics";

export default {
	name: "js/noDeleteVars",
	enter(path: Path): AnyNode {
		const {node} = path;

		if (
			node.type === "JSUnaryExpression" &&
			node.operator === "delete" &&
			node.argument.type === "JSReferenceIdentifier"
		) {
			path.context.addNodeDiagnostic(node, descriptions.LINT.JS_NO_DELETE_VARS);
		}

		return node;
	},
};
