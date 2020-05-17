/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {Path, TransformExitResult} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";
import {doesNodeMatchPattern, isConditional} from "@romejs/js-ast-utils";

function inComponentDidUpdate(path: Path): boolean {
	const func = path.findAncestry(({node}) => isConditional(node)) !== undefined;
	return (
		!func &&
		path.findAncestry(({node}) =>
			node.type === "ClassMethod" &&
			node.key.type === "StaticPropertyKey" &&
			node.key.value.type === "Identifier" &&
			node.key.value.name === "componentDidUpdate"
		) !== undefined
	);
}

export default {
	name: "noDidUpdateSetState",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (
			doesNodeMatchPattern(node, "this.setState") &&
			inComponentDidUpdate(path)
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.NO_DID_UPDATE_SET_STATE,
			);
		}

		return node;
	},
};
