/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {getJSXAttribute} from "@internal/js-ast-utils";
import {descriptions} from "@internal/diagnostics";
import {getCreateElementProp} from "../../utils/react";
import {AnyNode} from "@internal/ast";

function getJSXChildrenProp(node: AnyNode) {
	return node.type === "JSXElement" && getJSXAttribute(node, "children");
}

export default createVisitor({
	name: "react/noChildrenProp",
	enter(path) {
		const {node, scope} = path;
		const childrenProp =
			getJSXChildrenProp(node) || getCreateElementProp(node, scope, "children");
		if (childrenProp) {
			path.context.addNodeDiagnostic(
				childrenProp,
				descriptions.LINT.REACT_NO_CHILDREN_PROP,
			);
		}

		return signals.retain;
	},
});
