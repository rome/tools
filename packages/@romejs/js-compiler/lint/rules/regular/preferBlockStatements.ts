/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path, TransformExitResult} from "@romejs/js-compiler";
import {blockStatement} from "@romejs/js-ast";
import {descriptions} from "@romejs/diagnostics";
import {commentInjector} from "../../../transforms/defaultHooks";

export default {
	name: "preferBlockStatements",
	enter(path: Path): TransformExitResult {
		const {context, node} = path;

		if (node.type === "IfStatement") {
			let shouldFix = false;
			let consequent = node.consequent;
			let alternate = node.alternate;

			if (node.consequent.type !== "BlockStatement") {
				consequent = blockStatement.quick([node.consequent]);
				shouldFix = true;
			}

			if (
				node.alternate !== undefined &&
				node.alternate.type !== "BlockStatement" &&
				node.alternate.type !== "IfStatement"
			) {
				alternate = blockStatement.quick([node.alternate]);
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
					descriptions.LINT.PREFER_BLOCK_STATEMENT,
				);
			}
		} else if (
			node.type === "ForStatement" ||
			node.type === "ForInStatement" ||
			node.type === "ForOfStatement" ||
			node.type === "DoWhileStatement" ||
			node.type === "WhileStatement" ||
			node.type === "WithStatement"
		) {
			if (node.body.type === "EmptyStatement") {
				const id = path.callHook(
					commentInjector,
					{
						type: "CommentLine",
						value: " empty",
					},
				);

				return context.addFixableDiagnostic(
					{
						old: node,
						fixed: {
							...node,
							body: blockStatement.create({
								innerComments: [id],
								body: [],
							}),
						},
					},
					descriptions.LINT.PREFER_BLOCK_STATEMENT,
				);
			}

			if (node.body.type !== "BlockStatement") {
				return context.addFixableDiagnostic(
					{
						old: node,
						fixed: {
							...node,
							body: blockStatement.quick([node.body]),
						},
					},
					descriptions.LINT.PREFER_BLOCK_STATEMENT,
				);
			}
		}

		return node;
	},
};
