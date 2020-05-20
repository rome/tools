/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSComment, AnyNode, JSRoot} from "@romejs/ast";
import {CompilerContext} from "@romejs/compiler";
import {Number1, ob1Get1} from "@romejs/ob1";
import Path from "../lib/Path";
import {SUPPRESSION_START} from "../suppressions";
import {commentInjector} from "../transforms/defaultHooks/index";
import {LintCompilerOptionsDecision} from "../types";

function getStartLine(node: AnyNode): undefined | Number1 {
	const {loc} = node;
	if (loc === undefined) {
		return undefined;
	} else {
		return loc.start.line;
	}
}

function buildSuppressionCommentValue(categories: Set<string>): string {
	return `${SUPPRESSION_START} ${Array.from(categories).join(" ")}`;
}

export function addSuppressions(
	context: CompilerContext,
	ast: JSRoot,
): JSRoot {
	if (!context.hasLintDecisions()) {
		return ast;
	}

	const visitedLines: Set<Number1> = new Set();

	function addComment(
		path: Path,
		node: AnyNode,
		decisions: Array<LintCompilerOptionsDecision>,
	): AnyNode {
		// Find all suppression decisions
		const suppressionCategories: Set<string> = new Set();
		for (const {category, action} of decisions) {
			if (action === "suppress") {
				suppressionCategories.add(category);
			}
		}
		if (suppressionCategories.size === 0) {
			return node;
		}

		// Find existing suppression comment
		let updateComment: undefined | AnyJSComment;
		const lastComment = context.comments.getCommentsFromIds(
			node.leadingComments,
		).pop();
		if (
			lastComment !== undefined &&
			lastComment.value.includes(SUPPRESSION_START)
		) {
			updateComment = lastComment;
		}

		// Insert new comment if there's none to update
		if (updateComment === undefined) {
			const id = path.callHook(
				commentInjector,
				{
					type: "JSCommentLine",
					value: ` ${buildSuppressionCommentValue(suppressionCategories)}`,
				},
			);

			return {
				...node,
				leadingComments: [...(node.leadingComments || []), id],
			};
		}

		// Remove all categories that are already included in the suppression
		for (const category of suppressionCategories) {
			if (updateComment.value.includes(category)) {
				suppressionCategories.delete(category);
			}
		}

		// We may have eliminated them all
		if (suppressionCategories.size > 0) {
			path.callHook(
				commentInjector,
				{
					...updateComment,
					value: updateComment.value.replace(
						SUPPRESSION_START,
						buildSuppressionCommentValue(suppressionCategories),
					),
				},
			);
		}

		return node;
	}

	// Find the best node to attach comments to. This is generally the node with the largest range per line.
	return context.reduceRoot(
		ast,
		{
			name: "suppressionVisitor",
			enter(path: Path) {
				const {node} = path;

				// Don't allow attaching suppression comments to a comment or program...
				if (
					node.type === "JSCommentBlock" ||
					node.type === "JSCommentLine" ||
					node.type === "JSRoot"
				) {
					return node;
				}

				const line = getStartLine(node);
				if (line === undefined || visitedLines.has(line)) {
					return node;
				}

				const decisions = context.getLintDecisions(String(ob1Get1(line)));
				if (decisions.length === 0) {
					return node;
				}

				visitedLines.add(line);
				return addComment(path, node, decisions);
			},
		},
	);
}
