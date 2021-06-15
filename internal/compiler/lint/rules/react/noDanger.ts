/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {createLintVisitor, signals} from "@internal/compiler";
import {getJSXAttribute} from "@internal/js-ast-utils";
import {descriptions} from "@internal/diagnostics";
import {getCreateElementProp} from "../../utils/react";

function getJSXDangerProp(node: AnyNode) {
	return (
		node.type === "JSXElement" &&
		getJSXAttribute(node, "dangerouslySetInnerHTML")
	);
}

export default createLintVisitor({
	name: "react/noDanger",
	enter(path) {
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

		return signals.retain;
	},
});
