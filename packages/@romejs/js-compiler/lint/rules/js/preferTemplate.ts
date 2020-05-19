/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romejs/js-compiler";
import {
	JSTemplateLiteral,
	jsTemplateElement,
	jsTemplateLiteral,
} from "@romejs/ast";
import {descriptions} from "@romejs/diagnostics";
import {TransformExitResult} from "@romejs/js-compiler/types";
import {removeShallowLoc} from "@romejs/js-ast-utils";

export default {
	name: "preferTemplate",
	enter(path: Path): TransformExitResult {
		const {node} = path;

		if (
			node.type === "JSBinaryExpression" &&
			node.operator === "+" &&
			((node.left.type === "JSStringLiteral" && !node.left.value.includes("`")) ||
			(node.right.type === "JSStringLiteral" && !node.right.value.includes("`")))
		) {
			let autofix: undefined | JSTemplateLiteral;

			if (node.right.type === "JSStringLiteral") {
				const quasis = [
					jsTemplateElement.create({
						raw: "",
						cooked: "",
					}),
					jsTemplateElement.create({
						raw: node.right.value,
						cooked: node.right.value,
					}),
				];
				const expressions = [removeShallowLoc(node.left)];
				autofix = jsTemplateLiteral.create({
					expressions,
					quasis,
					loc: node.loc,
				});
			}

			if (node.left.type === "JSStringLiteral") {
				const quasis = [
					jsTemplateElement.create({
						raw: node.left.value,
						cooked: node.left.value,
					}),
					jsTemplateElement.create({
						raw: "",
						cooked: "",
					}),
				];

				// We need to remove the location or else if we were to show a preview the source map would resolve to the end of
				// this node
				const expressions = [removeShallowLoc(node.right)];
				autofix = jsTemplateLiteral.create({
					expressions,
					quasis,
					loc: node.loc,
				});
			}

			if (autofix === undefined) {
				path.context.addNodeDiagnostic(
					node,
					descriptions.LINT.JS_PREFER_TEMPLATE,
				);
			} else {
				return path.context.addFixableDiagnostic(
					{
						old: node,
						fixed: autofix,
					},
					descriptions.LINT.JS_PREFER_TEMPLATE,
				);
			}
		}

		return node;
	},
};
