/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from "@romejs/compiler";
import {jsBlockStatement} from "@romejs/ast";
import {descriptions} from "@romejs/diagnostics";
import {commentInjector} from "../../../transforms/defaultHooks";

export default {
	name: "preferBlockStatements",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (node.type === "JSIfStatement") {
			let shouldFix = false;
			let consequent = node.consequent;
			let alternate = node.alternate;

			if (node.consequent.type !== "JSBlockStatement") {
				consequent = jsBlockStatement.quick([node.consequent]);
				shouldFix = true;
			}

			if (
				node.alternate !== undefined &&
				node.alternate.type !== "JSBlockStatement" &&
				node.alternate.type !== "JSIfStatement"
			) {
				alternate = jsBlockStatement.quick([node.alternate]);
				shouldFix = true;
			}

			if (shouldFix) {
				return context.addFixableDiagnostic(
					{
						old: node,
						fixed: {
							...node,
							consequent,
							alternate,
						},
					},
					descriptions.LINT.JS_PREFER_BLOCK_STATEMENT,
				);
			}
		} else if (
			node.type === "JSForStatement" ||
			node.type === "JSForInStatement" ||
			node.type === "JSForOfStatement" ||
			node.type === "JSDoWhileStatement" ||
			node.type === "JSWhileStatement" ||
			node.type === "JSWithStatement"
		) {
			if (node.body.type === "JSEmptyStatement") {
				const id = path.callHook(
					commentInjector,
					{
						type: "JSCommentLine",
						value: " empty",
					},
				);

				return context.addFixableDiagnostic(
					{
						old: node,
						fixed: {
							...node,
							body: jsBlockStatement.create({
								innerComments: [id],
								body: [],
							}),
						},
					},
					descriptions.LINT.JS_PREFER_BLOCK_STATEMENT,
				);
			}

			if (node.body.type !== "JSBlockStatement") {
				return context.addFixableDiagnostic(
					{
						old: node,
						fixed: {
							...node,
							body: jsBlockStatement.quick([node.body]),
						},
					},
					descriptions.LINT.JS_PREFER_BLOCK_STATEMENT,
				);
			}
		}

		return node;
	},
};
