/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment, AnyNode} from "@internal/ast";
import {getLinesBetween} from "../node";
import {
	Token,
	comment,
	concat,
	hardline,
	ifBreak,
	join,
	lineSuffix,
	space,
} from "@internal/formatter";
import {DiagnosticLanguage} from "@internal/diagnostics";

export function hasInnerComments(node: AnyNode): boolean {
	return node.innerComments !== undefined && node.innerComments.length > 0;
}

type CommentWrapper = [string, string];

const CSS_COMMENT_WRAPPER: CommentWrapper = ["/*", "*/"];
const HTML_COMMENT_WRAPPER: CommentWrapper = ["<!--", "-->"];

const languageToBlockCommentWrapper: {
	[language in DiagnosticLanguage]?: CommentWrapper
} = {
	html: HTML_COMMENT_WRAPPER,
	md: HTML_COMMENT_WRAPPER,
	css: CSS_COMMENT_WRAPPER,
	js: CSS_COMMENT_WRAPPER,
	json: CSS_COMMENT_WRAPPER,
};

export function tokenizeComment(
	node: AnyComment,
	language: DiagnosticLanguage,
): Token {
	// Only JS can have line comments
	if (language === "js" && node.type === "CommentLine") {
		// NB: We assume that node.value doesn't have any newlines. Is that a safe assumption to make?
		return comment(`//${node.value.trimEnd()}`);
	}

	const wrapper = languageToBlockCommentWrapper[language];
	if (wrapper === undefined) {
		throw new Error(`No block comment wrapper found for language ${language}`);
	}

	const [prefix, suffix] = wrapper;
	const lines = node.value.split("\n");

	if (
		wrapper === CSS_COMMENT_WRAPPER &&
		lines.every((line) => line.trimStart().charAt(0) === "*")
	) {
		return comment(
			concat([
				prefix,
				join(
					hardline,
					lines.map((line, index) =>
						index === 0
							? line.trimEnd()
							: ` ${index < lines.length - 1 ? line.trim() : line.trimStart()}`
					),
				),
				suffix,
			]),
		);
	} else {
		return comment(`${prefix}${node.value}${suffix}`);
	}
}

function tokenizeCommentSeparator(left: AnyNode, right: AnyNode): Token {
	const linesBetween = getLinesBetween(left, right);
	if (linesBetween === 0) {
		return space;
	}
	if (linesBetween === 1) {
		return hardline;
	}
	return concat([hardline, hardline]);
}

export function tokenizeLeadingComment(
	node: AnyComment,
	next: AnyNode,
	language: DiagnosticLanguage,
): Token {
	const comment = tokenizeComment(node, language);
	if (node.type === "CommentLine") {
		return concat([comment, hardline]);
	} else {
		return concat([comment, tokenizeCommentSeparator(node, next)]);
	}
}

export function tokenizeTrailingComment(
	node: AnyComment,
	previous: AnyNode,
	language: DiagnosticLanguage,
): Token {
	const comment = tokenizeComment(node, language);
	const linesBetween = getLinesBetween(previous, node);

	if (linesBetween >= 1) {
		return lineSuffix(
			concat([linesBetween > 1 ? hardline : "", hardline, comment]),
		);
	} else {
		if (node.type === "CommentBlock") {
			return ifBreak(
				lineSuffix(concat([space, comment])),
				concat([space, comment]),
			);
		} else {
			return lineSuffix(concat([space, comment]));
		}
	}
}
