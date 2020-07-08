/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romefrontend/compiler";
import {AnyNode} from "@romefrontend/ast";
import {getJSXAttribute} from "@romefrontend/js-ast-utils";
import {descriptions} from "@romefrontend/diagnostics";
import {getCreateElementProp} from "../../utils/react";

function getJSXChildrenProp(node: AnyNode) {
	return node.type === "JSXElement" && getJSXAttribute(node, "children");
}

export default {
	name: "reactNoChildrenProp",
	enter(path: Path): AnyNode {
		const {node, scope} = path;
		const childrenProp =
			getJSXChildrenProp(node) || getCreateElementProp(node, scope, "children");
		if (childrenProp) {
			path.context.addNodeDiagnostic(
				childrenProp,
				descriptions.LINT.REACT_NO_CHILDREN_PROP,
			);
		}

		return node;
	},
};
