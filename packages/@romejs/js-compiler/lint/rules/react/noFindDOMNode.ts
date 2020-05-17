/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/js-ast";
import {Path} from "@romejs/js-compiler";
import {doesNodeMatchPattern} from "@romejs/js-ast-utils";

function hasFindMemberProperty(node: AnyNode) {
	return (
		node.type === "StaticMemberProperty" &&
		doesNodeMatchPattern(node.value, "findDOMNode")
	);
}

function hasFindCallExpression(node: AnyNode) {
	return (
		node.type === "CallExpression" &&
		doesNodeMatchPattern(node.callee, "findDOMNode")
	);
}

export default {
	name: "noFindDOMNode",

	enter(path: Path): AnyNode {
		const {node} = path;

		if (hasFindMemberProperty(node) || hasFindCallExpression(node)) {
			path.context.addNodeDiagnostic(node, descriptions.LINT.NO_FIND_DOM_NODE);
		}

		return node;
	},
};
