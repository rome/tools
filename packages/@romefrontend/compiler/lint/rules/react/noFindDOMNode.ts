/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {descriptions} from "@romefrontend/diagnostics";
import {AnyNode} from "@romefrontend/ast";
import {Path} from "@romefrontend/compiler";
import {doesNodeMatchReactPattern} from "../../utils/react";

const reactDOMConfig = {
	packageName: "react-dom",
	importName: "ReactDOM",
};

export default {
	name: "reactNoFindDOMNode",

	enter(path: Path): AnyNode {
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

		return node;
	},
};
