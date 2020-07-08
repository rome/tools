/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";
import {doesNodeMatchPattern, isConditional} from "@romefrontend/js-ast-utils";
import {insideClassComponent} from "../../utils/react";

function inComponentDidUpdate(path: Path): boolean {
	const func = path.findAncestry(({node}) => isConditional(node)) !== undefined;
	return (
		!func &&
		path.findAncestry(({node}) =>
			node.type === "JSClassMethod" &&
			node.key.type === "JSStaticPropertyKey" &&
			node.key.value.type === "JSIdentifier" &&
			node.key.value.name === "componentDidUpdate"
		) !== undefined
	);
}

export default {
	name: "reactNoDidUpdateSetState",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (
			insideClassComponent(path) &&
			doesNodeMatchPattern(node, "this.setState") &&
			inComponentDidUpdate(path)
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_DID_UPDATE_SET_STATE,
			);
		}

		return node;
	},
};
