/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment, AnyNode, AnyRoot} from "@internal/ast";
import {CompilerContext, signals} from "@internal/compiler";
import CompilerPath from "../lib/CompilerPath";
import {LintCompilerOptionsDecision} from "../types";
import {injectComment} from "../transforms/helpers";
import {extractSuppressionsFromComment} from "../suppressions";
import {SUPPRESSION_START} from "../suppressionsParser";
import {formatCategoryDescription, joinCategoryName} from "@internal/diagnostics";

function getStartLine(node: AnyNode): undefined | number {
	const {loc} = node;
	if (loc === undefined) {
		return undefined;
	} else {
		return loc.start.line.valueOf();
	}
}

function buildSuppressionCommentValue(
	categories: Set<string>,
	explanation = "suppressed via --review",
): string {
	return `${SUPPRESSION_START} ${Array.from(categories).sort().join(" ")}: ${explanation}`;
}

function addComment({path, decisions, explanation}: {
	path: CompilerPath,
	decisions: LintCompilerOptionsDecision[],
	explanation: undefined | string;
}): AnyNode {
	const {node, context} = path;

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
	const lastComment = context.comments.getCommentsFromIds(
		node.leadingComments,
	).pop();
	if (
		lastComment !== undefined &&
		lastComment.value.includes(SUPPRESSION_START)
	) {
		// Try to update it
		const updated = updateExistingComment({
			path,
			comment: lastComment,
			injectCategories: suppressionCategories,
			explanation,
		});
		if (updated) {
			return node;
		}
	}

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

function updateExistingComment({path, comment, injectCategories, explanation}: {
	path: CompilerPath;
	comment: AnyComment;
	injectCategories: Set<string>;
	explanation: undefined | string;
}): boolean {
	const existingParsed = extractSuppressionsFromComment({
		context: path.context,
		comment,
		targetNode: path.node,
		overrideRequireExplanations: false,
	});

	if (existingParsed === undefined || existingParsed.diagnostics.length > 0) {
		// Wasn't a valid suppression comment
		return false;
	}

	if (explanation !== undefined && existingParsed.explanation !== undefined && existingParsed.explanation !== explanation) {
		// We have a specific explanation so inject a new comment
		return false;
	}

	const allCategories: Set<string> = new Set();

	// Add new categories from the existing suppression
	const existingCategories = new Set();
	for (const {category} of existingParsed.suppressions) {
		const joinedCategory = joinCategoryName(category);
		existingCategories.add(joinedCategory);
		allCategories.add(joinedCategory);
	}

	// Check if all of the categories we want to add already exist on the existing suppression
	let needsUpdate = false;
	for (const category of injectCategories) {
		allCategories.add(category);
		if (!existingCategories.has(category)) {
			needsUpdate = true;
		}
	}

	if (needsUpdate) {
		injectComment(
			path,
			{
				...comment,
				value: ` ${buildSuppressionCommentValue(allCategories, explanation)}`,
			},
		);
	}

	return true;
}

export function addSuppressions(
	context: CompilerContext,
	ast: AnyRoot,
	explanation?: string,
): AnyRoot {
	if (!context.hasLintDecisions()) {
		return ast;
	}

	const visitedLines: Set<number> = new Set();
	const erroredLines: Set<number> = new Set();

	for (const diag of context.diagnostics.getDiagnostics()) {
		const start = diag.location.start;
		if (start !== undefined) {
			erroredLines.add(start.line.valueOf());
		}
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

				const decisions = context.getLintDecisions(String(line.valueOf()), erroredLines.has(line.valueOf()));
				if (decisions.length === 0) {
					return signals.retain;
				}

				visitedLines.add(line);
				return signals.replace(addComment({
					path,
					decisions,
					explanation,
				}));
			},
		},
		ast,
	);
}
