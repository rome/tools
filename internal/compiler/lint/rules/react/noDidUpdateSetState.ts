/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {CompilerPath, createLintVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";
import {doesNodeMatchPattern, isConditional} from "@internal/js-ast-utils";
import {insideClassComponent} from "../../utils/react";

function inComponentDidUpdate(path: CompilerPath): boolean {
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

export default createLintVisitor({
	name: "react/noDidUpdateSetState",
	enter(path) {
		const {node} = path;

		if (
			doesNodeMatchPattern(node, "this.setState") &&
			insideClassComponent(path) &&
			inComponentDidUpdate(path)
		) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.REACT_NO_DID_UPDATE_SET_STATE,
			);
		}

		return signals.retain;
	},
});
