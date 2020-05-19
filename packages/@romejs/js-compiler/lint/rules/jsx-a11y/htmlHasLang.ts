/**
* Copyright (c) Facebook, Inc. and its affiliates.
*
* This source code is licensed under the MIT license found in the
* LICENSE file in the root directory of this source tree.
*/

import {descriptions} from "@romejs/diagnostics";
import {AnyNode} from "@romejs/ast";
import {Path} from "@romejs/js-compiler";
import {hasJSXAttribute, isJSXElement} from "@romejs/js-ast-utils";

function jsxHTMLMissingLang(node: AnyNode) {
	return isJSXElement(node, "html") && !hasJSXAttribute(node, "lang");
}

export default {
	name: "jsxA11YHTMLHasLang",

	enter(path: Path): AnyNode {
		const {node} = path;

		if (jsxHTMLMissingLang(node)) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.JSX_A11Y_HTML_HAS_LANG,
			);
		}

		return node;
	},
};
