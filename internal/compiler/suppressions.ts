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
} from "@internal/diagnostics";
import {ob1Coerce0, ob1Number1} from "@internal/ob1";
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
					{line: ob1Number1, column: ob1Coerce0(2)},
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

			if (node.loc !== undefined && node.leadingComments !== undefined) {
				for (const comment of context.comments.getCommentsFromIds(
					node.leadingComments,
				)) {
					if (visitedComments.has(comment)) {
						continue;
					}

					visitedComments.add(comment);
					const result = extractSuppressionsFromComment(context, comment, node);
					if (result !== undefined) {
						context.diagnostics.addDiagnostics(result.diagnostics);
						context.suppressions = context.suppressions.concat(
							result.suppressions,
						);
					}
				}
			}

			return signals.retain;
		},
	});
}

export function matchesSuppression(
	category: DiagnosticCategory,
	categoryValue: undefined | string,
	{path, start, end}: DiagnosticLocation,
	suppression: DiagnosticSuppression,
): boolean {
	return (
		category === suppression.category &&
		equalPaths(path, suppression.path) &&
		start !== undefined &&
		end !== undefined &&
		start.line >= suppression.startLine &&
		end.line <= suppression.endLine &&
		(suppression.categoryValue === undefined ||
		categoryValue === suppression.categoryValue)
	);
}
