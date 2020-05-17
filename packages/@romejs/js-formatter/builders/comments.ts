/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment, AnyNode} from "@romejs/js-ast";
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
} from "../tokens";

export function hasInnerComments(node: AnyNode): boolean {
	return node.innerComments !== undefined && node.innerComments.length > 0;
}

export function printComment(node: AnyComment): Token {
	switch (node.type) {
		case "CommentBlock": {
			const lines = node.value.split("\n");
			if (lines.every((line) => line.trimStart().charAt(0) === "*")) {
				return comment(
					concat([
						"/*",
						join(
							hardline,
							lines.map((line, index) =>
								index === 0
									? line.trimEnd()
									: ` ${index < lines.length - 1 ? line.trim() : line.trimStart()}`
							),
						),
						"*/",
					]),
				);
			} else {
				return comment(`/*${node.value}*/`);
			}
		}

		case "CommentLine": {
			return comment(`//${node.value.trimEnd()}`);
		}
	}
}

function printCommentSeparator(left: AnyNode, right: AnyNode): Token {
	const linesBetween = getLinesBetween(left, right);
	return linesBetween === 0
		? space
		: linesBetween === 1
			? hardline
			: concat([hardline, hardline]);
}

export function printLeadingComment(node: AnyComment, next: AnyNode): Token {
	const comment = printComment(node);
	if (node.type === "CommentLine") {
		return concat([comment, hardline]);
	} else {
		return concat([comment, printCommentSeparator(node, next)]);
	}
}

export function printTrailingComment(node: AnyComment, previous: AnyNode): Token {
	const comment = printComment(node);
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
