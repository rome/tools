/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment, AnyNode, AnyRoot} from "@internal/ast";
import {CompilerContext, signals} from "@internal/compiler";
import {Number1, ob1Get1} from "@internal/ob1";
import Path from "../lib/Path";
import {SUPPRESSION_START} from "../suppressions";
import {LintCompilerOptionsDecision} from "../types";
import {injectComment} from "../transforms/helpers";

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

export function addSuppressions(context: CompilerContext, ast: AnyRoot): AnyRoot {
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
		let updateComment: undefined | AnyComment;
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
			const id = injectComment(
				path,
				{
					type: "CommentLine",
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
			injectComment(
				path,
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
		{
			name: "suppressionVisitor",
			enter(path) {
				const {node} = path;

				// Don't allow attaching suppression comments to a comment or program...
				if (
					node.type === "CommentBlock" ||
					node.type === "CommentLine" ||
					node.type === "JSRoot"
				) {
					return signals.retain;
				}

				const line = getStartLine(node);
				if (line === undefined || visitedLines.has(line)) {
					return signals.retain;
				}

				const decisions = context.getLintDecisions(String(ob1Get1(line)));
				if (decisions.length === 0) {
					return signals.retain;
				}

				visitedLines.add(line);
				return signals.replace(addComment(path, node, decisions));
			},
		},
		ast,
	);
}
