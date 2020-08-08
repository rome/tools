/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {descriptions} from "@internal/diagnostics";
import {AnyNode} from "@internal/ast";
import {createVisitor, signals} from "@internal/compiler";
import {getJSXAttribute, isJSXElement} from "@internal/js-ast-utils";

function jsxImgRedundantAlt(node: AnyNode) {
	if (!isJSXElement(node, "img")) {
		return false;
	}

	const attr = getJSXAttribute(node, "alt");
	return (
		attr !== undefined &&
		attr.value &&
		attr.value.type === "JSStringLiteral" &&
		/(image)|(picture)|(photo)/i.test(attr.value.value)
	);
}

export default createVisitor({
	name: "jsx-a11y/noRedundantAlt",

	enter(path) {
		const {node} = path;

		if (jsxImgRedundantAlt(node)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_IMG_REDUNDANT_ALT,
			);
		}

		return signals.retain;
	},
});
