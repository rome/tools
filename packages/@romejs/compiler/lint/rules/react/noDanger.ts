/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/compiler";
import {AnyNode} from "@romejs/ast";
import {getJSXAttribute} from "@romejs/js-ast-utils";
import {descriptions} from "@romejs/diagnostics";
import {getCreateElementProp} from "../../utils/react";

function getJSXDangerProp(node: AnyNode) {
	return (
		node.type === "JSXElement" &&
		getJSXAttribute(node, "dangerouslySetInnerHTML")
	);
}

export default {
	name: "reactNoDanger",
	enter(path: Path): AnyNode {
		const {node, scope} = path;
		const dangerProp =
			getJSXDangerProp(node) ||
			getCreateElementProp(node, scope, "dangerouslySetInnerHTML");
		if (dangerProp) {
			path.context.addNodeDiagnostic(
				dangerProp,
				descriptions.LINT.REACT_NO_DANGER,
			);
		}

		return node;
	},
};
