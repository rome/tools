/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSRegExpBodyItem, AnyJSRegExpExpression} from "@romefrontend/ast";
import {Path, TransformExitResult} from "@romefrontend/compiler";
import {descriptions} from "@romefrontend/diagnostics";

function isQuantifiedMinZero(el: AnyJSRegExpBodyItem): boolean {
	return el.type === "JSRegExpQuantified" && el.min === 0;
}

function lintEmptyMatches(expr: AnyJSRegExpExpression): boolean {
	if (expr.type === "JSRegExpSubExpression") {
		for (const item of expr.body) {
			let matches = false;
			if (
				item.type === "JSRegExpGroupNonCapture" ||
				item.type === "JSRegExpGroupCapture"
			) {
				matches = lintEmptyMatches(item.expression);
			} else {
				matches = isQuantifiedMinZero(item);
			}
			if (!matches) {
				return false;
			}
		}
		return true;
	} else {
		return lintEmptyMatches(expr.left) || lintEmptyMatches(expr.right);
	}
}

export default {
	name: "js/emptyMatches",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;
		if (node.type === "JSRegExpLiteral" && lintEmptyMatches(node.expression)) {
			context.addNodeDiagnostic(node, descriptions.LINT.JS_EMPTY_MATCHES);
		}
		return node;
	},
};
