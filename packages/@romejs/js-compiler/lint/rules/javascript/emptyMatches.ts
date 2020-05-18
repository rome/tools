/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyRegExpBodyItem, AnyRegExpExpression} from "@romejs/js-ast";
import {Path, TransformExitResult} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";

function isQuantifiedMinZero(el: AnyRegExpBodyItem): boolean {
	return el.type === "RegExpQuantified" && el.min === 0;
}

function lintEmptyMatches(expr: AnyRegExpExpression): boolean {
	if (expr.type === "RegExpSubExpression") {
		for (const item of expr.body) {
			let matches = false;
			if (
				item.type === "RegExpGroupNonCapture" ||
				item.type === "RegExpGroupCapture"
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
	name: "emptyMatches",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;
		if (node.type === "RegExpLiteral" && lintEmptyMatches(node.expression)) {
			context.addNodeDiagnostic(
				node,
				descriptions.LINT.JAVASCRIPT_EMPTY_MATCHES,
			);
		}
		return node;
	},
};
