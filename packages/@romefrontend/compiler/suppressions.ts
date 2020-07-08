/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment, AnyNode, AnyRoot} from "@romefrontend/ast";
import {
	DiagnosticLocation,
	DiagnosticSuppression,
	DiagnosticSuppressions,
	Diagnostics,
	descriptions,
} from "@romefrontend/diagnostics";
import CompilerContext from "./lib/CompilerContext";
import Path from "./lib/Path";

export const SUPPRESSION_START = "rome-ignore";

type ExtractedSuppressions = {
	suppressions: DiagnosticSuppressions;
	diagnostics: Diagnostics;
};

type NodeToComment = Map<AnyComment, AnyNode>;

function extractSuppressionsFromComment(
	context: CompilerContext,
	comment: AnyComment,
	nodeToComment: NodeToComment,
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
		if (!line.startsWith(SUPPRESSION_START)) {
			continue;
		}

		const nextNode = nodeToComment.get(comment);
		if (nextNode === undefined || nextNode.loc === undefined) {
			diagnostics.push({
				description: descriptions.SUPPRESSIONS.MISSING_TARGET,
				location: commentLocation,
			});
			continue;
		}

		const startLine = nextNode.loc.start.line;
		const endLine = nextNode.loc.end.line;

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

export function extractSuppressionsFromProgram(
	context: CompilerContext,
	ast: AnyRoot,
): ExtractedSuppressions {
	const {comments} = ast;

	let diagnostics: Diagnostics = [];
	let suppressions: DiagnosticSuppressions = [];

	const nodeToComment: NodeToComment = new Map();
	context.reduce(
		ast,
		{
			name: "extractSuppressions",
			enter(path: Path): AnyNode {
				const {node} = path;

				for (const comment of context.comments.getCommentsFromIds(
					node.leadingComments,
				)) {
					nodeToComment.set(comment, node);
				}

				return node;
			},
		},
		{
			noScopeCreation: true,
		},
	);

	for (const comment of comments) {
		const result = extractSuppressionsFromComment(
			context,
			comment,
			nodeToComment,
		);
		if (result !== undefined) {
			diagnostics = diagnostics.concat(result.diagnostics);
			suppressions = suppressions.concat(result.suppressions);
		}
	}

	return {suppressions, diagnostics};
}

export function matchesSuppression(
	{filename, start, end}: DiagnosticLocation,
	suppression: DiagnosticSuppression,
): boolean {
	return (
		filename === suppression.filename &&
		start !== undefined &&
		end !== undefined &&
		start.line >= suppression.startLine &&
		end.line <= suppression.endLine
	);
}
