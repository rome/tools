/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment, AnyNode, AnyRoot} from "@internal/ast";
import {CompilerContext, signals} from "@internal/compiler";
import {IndexedNumberSet, OneIndexed} from "@internal/numbers";
import CompilerPath from "../lib/CompilerPath";
import {LintCompilerOptionsDecision} from "../types";
import {injectComment} from "../transforms/helpers";
import {SUPPRESSION_START} from "../suppressionsParser";
import {formatCategoryDescription} from "@internal/diagnostics";

function getStartLine(node: AnyNode): undefined | OneIndexed {
	const {loc} = node;
	if (loc === undefined) {
		return undefined;
	} else {
		return loc.start.line;
	}
}

function buildSuppressionCommentValue(
	categories: Set<string>,
	explanation = "suppressed via --review",
): string {
	return `${SUPPRESSION_START} ${Array.from(categories).join(" ")}: ${explanation}`;
}

export function addSuppressions(
	context: CompilerContext,
	ast: AnyRoot,
	explanation?: string,
): AnyRoot {
	if (!context.hasLintDecisions()) {
		return ast;
	}

	const visitedLines = new IndexedNumberSet<OneIndexed>();

	function addComment(
		path: CompilerPath,
		node: AnyNode,
		decisions: LintCompilerOptionsDecision[],
	): AnyNode {
		// Find all suppression decisions
		const suppressionCategories: Set<string> = new Set();
		for (const decision of decisions) {
			if (decision.action === "suppress") {
				suppressionCategories.add(formatCategoryDescription(decision));
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
		if (lastComment?.value.includes(SUPPRESSION_START)) {
			updateComment = lastComment;
		}
		// Insert new comment if there's none to update
		if (updateComment === undefined) {
			const id = injectComment(
				path,
				{
					type: "CommentLine",
					value: ` ${buildSuppressionCommentValue(
						suppressionCategories,
						explanation,
					)}`,
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
			// get from comment possible existing categories
			// TODO: make sure that category matches the rules we have (remove "lint/" from category)
			const computedCategories = updateComment.value.slice(
				updateComment.value.indexOf(SUPPRESSION_START),
				updateComment.value.indexOf(":"),
			).replace(SUPPRESSION_START, "").split(" ").filter(Boolean);

			for (const computedCategory of computedCategories) {
				suppressionCategories.add(computedCategory);
			}
			injectComment(
				path,
				{
					...updateComment,
					value: buildSuppressionCommentValue(suppressionCategories),
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

				const decisions = context.getLintDecisions(String(line.valueOf()));
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
