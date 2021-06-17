/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment, AnyNode} from "@internal/ast";
import {
	DiagnosticCategory,
	DiagnosticLocation,
	DiagnosticSuppression,
	equalCategoryNames,
} from "@internal/diagnostics";
import {OneIndexed, ZeroIndexed} from "@internal/numbers";
import {addPositions} from "@internal/parser-core";
import {equalPaths} from "@internal/path";
import CompilerContext from "./lib/CompilerContext";
import * as signals from "./signals";
import {
	ExtractedSuppressions,
	parseCommentSuppressions,
} from "./suppressionsParser";
import {AnyVisitor} from "./types";
import {createVisitor} from "./utils";

function extractSuppressionsFromComment(
	context: CompilerContext,
	comment: AnyComment,
	targetNode: undefined | AnyNode,
): undefined | ExtractedSuppressions {
	const commentLocation = comment.loc;
	if (commentLocation === undefined) {
		return undefined;
	}

	const {diagnostics, suppressions} = parseCommentSuppressions({
		input: comment.value,
		requireExplanations: context.project.config.lint.requireSuppressionExplanations,
		targetNode,
		path: context.path,
		offsetPosition: comment.loc === undefined
			? undefined
			: addPositions(
					comment.loc.start,
					{line: new OneIndexed(), column: new ZeroIndexed(2)},
				),
	});

	if (suppressions.length === 0 && diagnostics.length === 0) {
		return undefined;
	} else {
		return {diagnostics, suppressions};
	}
}

export function createSuppressionsVisitor(): AnyVisitor {
	const visitedComments: Set<AnyComment> = new Set();

	return createVisitor({
		name: "suppressions",

		enter(path) {
			const {node, context} = path;

			if (node.loc !== undefined) {
				if (
					node.type === "JSXExpressionContainer" &&
					node.expression.innerComments !== undefined
				) {
					for (const comment of context.comments.getCommentsFromIds(
						node.expression.innerComments,
					)) {
						if (visitedComments.has(comment)) {
							continue;
						}
						if (path.parent.type === "JSXElement") {
							let currentNodeFound = false;
							let nextSibling: AnyNode | undefined = undefined;
							// scan the children in order to find the next sibling that should have the suppression
							for (const child of path.parent.children) {
								if (
									child.loc?.start === node.loc?.start &&
									child.loc?.end === node.loc?.end
								) {
									currentNodeFound = true;
									continue;
								}
								// here we don't check text because our parser tracks newlines and tabs/spaces as text
								if (currentNodeFound && child.type !== "JSXText") {
									nextSibling = child;
									visitedComments.add(comment);
									break;
								}
							}

							if (nextSibling) {
								storeSuppressions(context, comment, nextSibling);
							}
						}
					}
				} else if (node.leadingComments !== undefined) {
					for (const comment of context.comments.getCommentsFromIds(
						node.leadingComments,
					)) {
						if (visitedComments.has(comment)) {
							continue;
						}

						visitedComments.add(comment);
						storeSuppressions(context, comment, node);
					}
				}
			}

			return signals.retain;
		},
	});
}

function storeSuppressions(
	context: CompilerContext,
	comment: AnyComment,
	node: AnyNode,
) {
	const result = extractSuppressionsFromComment(context, comment, node);
	if (result !== undefined) {
		context.diagnostics.addDiagnostics(result.diagnostics);
		context.suppressions = context.suppressions.concat(result.suppressions);
	}
}

export function matchesSuppression(
	category: DiagnosticCategory,
	categoryValue: undefined | string,
	{path, start, end}: DiagnosticLocation,
	suppression: DiagnosticSuppression,
): boolean {
	return (
		equalCategoryNames(category, suppression.category) &&
		equalPaths(path, suppression.path) &&
		start !== undefined &&
		end !== undefined &&
		start.line.valueOf() >= suppression.startLine.valueOf() &&
		end.line.valueOf() <= suppression.endLine.valueOf() &&
		(suppression.categoryValue === undefined ||
		categoryValue === suppression.categoryValue)
	);
}
