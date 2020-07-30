/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {descriptions} from "@internal/diagnostics";
import {createVisitor, signals} from "@internal/compiler";
import {doesNodeMatchReactPattern} from "../../utils/react";

const reactDOMConfig = {
	packageName: "react-dom",
	importName: "ReactDOM",
};

export default createVisitor({
	name: "react/noFindDOMNode",

	enter(path) {
		const {node, scope} = path;

		if (
			node.type === "JSCallExpression" &&
			(doesNodeMatchReactPattern(
				node.callee,
				scope,
				"findDOMNode",
				reactDOMConfig,
			) ||
			doesNodeMatchReactPattern(
				node.callee,
				scope,
				"ReactDOM.findDOMNode",
				reactDOMConfig,
			))
		) {
			path.context.addNodeDiagnostic(
				node.callee,
				descriptions.LINT.REACT_NO_FIND_DOM_NODE,
			);
		}

		return signals.retain;
	},
});
