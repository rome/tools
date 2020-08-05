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
	DiagnosticSuppressions,
	Diagnostics,
	descriptions,
} from "@internal/diagnostics";
import CompilerContext from "./lib/CompilerContext";
import * as signals from "./signals";
import {AnyVisitor} from "./types";
import {createVisitor} from "./utils";

export const SUPPRESSION_START = "rome-ignore";
export const INCORRECT_SUPPRESSION_START = [
	"rome-disable",
	"@rome-ignore",
	"@rome-disable",
	"romefrontend-ignore",
	"romefrontend-disable",
	"@romefrontend-ignore",
	"@romefrontend-disable",
];

type ExtractedSuppressions = {
	suppressions: DiagnosticSuppressions;
	diagnostics: Diagnostics;
};

function extractSuppressionsFromComment(
	context: CompilerContext,
	comment: AnyComment,
	targetNode: undefined | AnyNode,
): undefined | ExtractedSuppressions {
	const commentLocation = comment.loc;
	if (commentLocation === undefined) {
		return undefined;
	}

	const suppressedCategories: Set<string> = new Set();
	const diagnostics: Diagnostics = [];
	const suppressions: DiagnosticSuppressions = [];

	const lines = comment.value.split("\n");
	const cleanLines = lines.map((line) => {
		// Trim line and remove leading star
		return line.trim().replace(/\*[\s]/, "");
	});

	for (const line of cleanLines) {
		if (
			INCORRECT_SUPPRESSION_START.some((incorrectStart) =>
				line.startsWith(incorrectStart)
			)
		) {
			diagnostics.push({
				description: descriptions.SUPPRESSIONS.INCORRECT_SUPPRESSION_START,
				location: commentLocation,
			});
		}

		if (!line.startsWith(SUPPRESSION_START)) {
			continue;
		}

		if (targetNode === undefined || targetNode.loc === undefined) {
			diagnostics.push({
				description: descriptions.SUPPRESSIONS.MISSING_TARGET,
				location: commentLocation,
			});
			break;
		}

		const startLine = targetNode.loc.start.line;
		const endLine = targetNode.loc.end.line;

		const lineWithoutPrefix = line.slice(SUPPRESSION_START.length);
		if (lineWithoutPrefix[0] !== " ") {
			diagnostics.push({
				description: descriptions.SUPPRESSIONS.MISSING_SPACE,
				location: commentLocation,
			});
			continue;
		}

		const categories = lineWithoutPrefix.trim().split(" ");
		const cleanCategories = categories.map((category) => category.trim());

		for (let category of cleanCategories) {
			if (category === "") {
				continue;
			}

			// If a category ends with a colon then all the things that follow it are an explanation
			let shouldBreak = false;
			if (category[category.length - 1] === ":") {
				shouldBreak = true;
				category = category.slice(-1);
			}
			if (suppressedCategories.has(category)) {
				diagnostics.push({
					description: descriptions.SUPPRESSIONS.DUPLICATE(category),
					location: commentLocation,
				});
			} else {
				suppressedCategories.add(category);

				suppressions.push({
					filename: context.filename,
					category,
					commentLocation,
					startLine,
					endLine,
				});
			}

			if (shouldBreak) {
				break;
			}
		}
	}

	if (suppressions.length === 0 && diagnostics.length === 0) {
		return undefined;
	} else {
		return {diagnostics, suppressions};
	}
}

export function createSuppressionsVisitor(): AnyVisitor {
	const visitedComments: Set<AnyComment> = new Set();

	// TODO verify all comments

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
	{filename, start, end}: DiagnosticLocation,
	suppression: DiagnosticSuppression,
): boolean {
	return (
		category === suppression.category &&
		filename === suppression.filename &&
		start !== undefined &&
		end !== undefined &&
		start.line >= suppression.startLine &&
		end.line <= suppression.endLine
	);
}
