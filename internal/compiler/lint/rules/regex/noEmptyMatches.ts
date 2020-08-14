/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSRegExpBodyItem, AnyJSRegExpExpression} from "@internal/ast";
import {createVisitor, signals} from "@internal/compiler";
import {descriptions} from "@internal/diagnostics";

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

export default createVisitor({
	name: "regex/noEmptyMatches",
	enter(path) {
		const {context, node} = path;
		if (node.type === "JSRegExpLiteral" && lintEmptyMatches(node.expression)) {
			context.addNodeDiagnostic(node, descriptions.LINT.REGEX_NO_EMPTY_MATCHES);
		}
		return signals.retain;
	},
});
