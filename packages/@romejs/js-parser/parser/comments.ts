/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/**
 * Based on the comment attachment algorithm used in espree and estraverse.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL <COPYRIGHT HOLDER> BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

import {JSParser} from "../parser";
import {SourceLocation} from "@romejs/parser-core";
import {AnyJSComment, AnyNode} from "@romejs/ast";
import {Number0} from "@romejs/ob1";

function last<T>(stack: Array<T>): T {
	return stack[stack.length - 1];
}

function getIds(comments: Array<AnyJSComment>): Array<string> {
	return comments.map((comment) => comment.id);
}

function getLoc(node: AnyNode): SourceLocation {
	const {loc} = node;
	if (loc === undefined) {
		throw new Error("No loc found");
	}
	return loc;
}

function start(node: AnyNode): Number0 {
	return getLoc(node).start.index;
}

function end(node: AnyNode): Number0 {
	return getLoc(node).end.index;
}

function hasComments(
	comments: undefined | Array<unknown>,
): comments is Array<unknown> {
	return comments !== undefined && comments.length > 0;
}

function setComments(
	node: AnyNode,
	key: "leadingComments" | "trailingComments",
	comments: Array<AnyJSComment>,
) {
	let innerEndIndex = -1;

	for (let i = 0; i < comments.length; i++) {
		const comment = comments[i];
		if (start(comment) >= start(node) && end(comment) <= end(node)) {
			innerEndIndex++;
		} else {
			break;
		}
	}

	if (innerEndIndex === -1) {
		node[key] = getIds(comments);
	} else {
		node.innerComments = getIds(comments.slice(0, innerEndIndex + 1));
		node[key] = getIds(comments.slice(innerEndIndex + 1));
	}
}

export function addComment(parser: JSParser, comment: AnyJSComment): void {
	parser.state.trailingComments.push(comment);
	parser.state.leadingComments.push(comment);
}

function adjustCommentsAfterTrailingComma(
	parser: JSParser,
	node: AnyNode,
	elements: Array<undefined | AnyNode>,
	// When the current node is followed by a token which hasn't a respective AST node, we
	// need to take all the trailing comments to prevent them from being attached to an
	// unrelated node. e.g. in
	//     var { x } /* cmt */ = { y }
	// we don't want /* cmt */ to be attached to { y }.
	// On the other hand, in
	//     fn(x) [new line] /* cmt */ [new line] y
	// /* cmt */ is both a trailing comment of fn(x) and a leading comment of y
	takeAllComments?: boolean,
) {
	if (parser.state.leadingComments.length === 0) {
		return;
	}

	let lastElement = undefined;
	let i = elements.length;
	while (lastElement === undefined && i > 0) {
		lastElement = elements[--i];
	}
	if (lastElement === undefined) {
		return;
	}

	const {commentPreviousNode} = parser.state;
	if (commentPreviousNode === undefined) {
		throw new Error("No commentPreviousNode found");
	}

	for (let j = 0; j < parser.state.leadingComments.length; j++) {
		if (end(parser.state.leadingComments[j]) < end(commentPreviousNode)) {
			parser.state.leadingComments.splice(j, 1);
			j--;
		}
	}

	const newTrailingComments: Array<AnyJSComment> = [];
	for (let i = 0; i < parser.state.leadingComments.length; i++) {
		const leadingComment = parser.state.leadingComments[i];
		if (end(leadingComment) < end(node)) {
			newTrailingComments.push(leadingComment);

			// Perf: we don't need to splice if we are going to reset the array anyway
			if (!takeAllComments) {
				parser.state.leadingComments.splice(i, 1);
				i--;
			}
		} else {
			if (node.trailingComments === undefined) {
				node.trailingComments = [];
			}
			node.trailingComments.push(leadingComment.id);
		}
	}
	if (takeAllComments) {
		parser.state.leadingComments = [];
	}

	if (newTrailingComments.length > 0) {
		lastElement.trailingComments = getIds(newTrailingComments);
	} else if (lastElement.trailingComments !== undefined) {
		lastElement.trailingComments = [];
	}
}

export function attachComments(parser: JSParser, node: AnyNode) {
	if (node.type === "JSRoot" && node.body.length > 0) {
		return;
	}

	const {commentStack, commentPreviousNode} = parser.state;

	let trailingComments: undefined | Array<AnyJSComment>;

	if (parser.state.trailingComments.length > 0) {
		// If the first comment in trailingComments comes after the
		// current node, then we're good - all comments in the array will
		// come after the node and so it's safe to add them as official
		// trailingComments.
		if (start(parser.state.trailingComments[0]) >= end(node)) {
			trailingComments = parser.state.trailingComments;
			parser.state.trailingComments = [];
		} else {
			// Otherwise, if the first comment doesn't come after the
			// current node, that means we have a mix of leading and trailing
			// comments in the array and that leadingComments contains the
			// same items as trailingComments. Reset trailingComments to
			// zero items and we'll handle this by evaluating leadingComments
			// later.
			parser.state.trailingComments = [];
		}
	} else if (commentStack.length > 0) {
		const lastInStack = last(commentStack);
		if (
			hasComments(lastInStack.trailingComments) &&
			start(
				parser.comments.assertGetCommentFromId(lastInStack.trailingComments[0]),
			) >= end(node)
		) {
			trailingComments = parser.comments.getCommentsFromIds(
				lastInStack.trailingComments,
			);
			lastInStack.trailingComments = undefined;
		}
	}

	// Eating the stack.
	let firstChild;
	if (commentStack.length > 0 && start(last(commentStack)) >= start(node)) {
		firstChild = commentStack.pop();
	}

	let lastChild;
	while (commentStack.length > 0 && start(last(commentStack)) >= start(node)) {
		lastChild = commentStack.pop();
	}

	if (!lastChild && firstChild) {
		lastChild = firstChild;
	}

	// Adjust comments that follow a trailing comma on the last element in a
	// comma separated list of nodes to be the trailing comments on the last
	// element
	if (firstChild) {
		switch (node.type) {
			case "JSObjectExpression": {
				adjustCommentsAfterTrailingComma(parser, node, node.properties);
				break;
			}

			case "JSBindingObjectPattern":
			case "JSAssignmentObjectPattern": {
				adjustCommentsAfterTrailingComma(parser, node, node.properties, true);
				break;
			}

			case "JSCallExpression": {
				adjustCommentsAfterTrailingComma(parser, node, node.arguments);
				break;
			}

			case "JSArrayExpression": {
				adjustCommentsAfterTrailingComma(parser, node, node.elements);
				break;
			}

			case "JSBindingArrayPattern":
			case "JSAssignmentArrayPattern": {
				adjustCommentsAfterTrailingComma(parser, node, node.elements, true);
				break;
			}
		}
	} else if (
		commentPreviousNode !== undefined &&
		((commentPreviousNode.type === "JSImportSpecifier" &&
		node.type !== "JSImportSpecifier") ||
		(commentPreviousNode.type === "JSExportLocalSpecifier" &&
		node.type !== "JSExportExternalSpecifier") ||
		(commentPreviousNode.type === "JSExportExternalSpecifier" &&
		node.type !== "JSExportExternalSpecifier"))
	) {
		adjustCommentsAfterTrailingComma(
			parser,
			node,
			[parser.state.commentPreviousNode],
		);
	}

	if (lastChild !== undefined) {
		if (hasComments(lastChild.leadingComments)) {
			if (
				lastChild !== node &&
				end(
					parser.comments.assertGetCommentFromId(
						last(lastChild.leadingComments),
					),
				) <= start(node)
			) {
				setComments(
					node,
					"leadingComments",
					parser.comments.getCommentsFromIds(lastChild.leadingComments),
				);
				lastChild.leadingComments = undefined;
			} else {
				// A leading comment for an anonymous class had been stolen by its first JSClassMethod,
				// so this takes back the leading comment.
				// See also: https://github.com/eslint/espree/issues/158
				for (let i = lastChild.leadingComments.length - 2; i >= 0; --i) {
					if (
						end(
							parser.comments.assertGetCommentFromId(
								lastChild.leadingComments[i],
							),
						) <=
						start(node)
					) {
						setComments(
							node,
							"leadingComments",
							parser.comments.getCommentsFromIds(
								lastChild.leadingComments.splice(0, i + 1),
							),
						);
						break;
					}
				}
			}
		}
	} else if (parser.state.leadingComments.length > 0) {
		if (end(last(parser.state.leadingComments)) <= start(node)) {
			if (parser.state.commentPreviousNode) {
				for (let j = 0; j < parser.state.leadingComments.length; j++) {
					if (
						end(parser.state.leadingComments[j]) <
						end(parser.state.commentPreviousNode)
					) {
						parser.state.leadingComments.splice(j, 1);
						j--;
					}
				}
			}

			if (parser.state.leadingComments.length > 0) {
				setComments(node, "leadingComments", parser.state.leadingComments);
				parser.state.leadingComments = [];
			}
		} else {
			// https://github.com/eslint/espree/issues/2
			//
			// In special cases, such as return (without a value) and
			// debugger, all comments will end up as leadingComments and
			// will otherwise be eliminated. This step runs when the
			// commentStack is empty and there are comments left
			// in leadingComments.
			//
			// This loop figures out the stopping point between the actual
			// leading and trailing comments by finding the location of the
			// first comment that comes after the given node.
			let i = 0;
			while (i < parser.state.leadingComments.length) {
				if (end(parser.state.leadingComments[i]) > start(node)) {
					break;
				} else {
					i++;
				}
			}

			// Split the array based on the location of the first comment
			// that comes after the node. Keep in mind that this could
			// result in an empty array, and if so, the array must be
			// deleted.

			const leadingComments = parser.state.leadingComments.slice(0, i);

			if (leadingComments.length > 0) {
				setComments(node, "leadingComments", leadingComments);
			}

			// Similarly, trailing comments are attached later. The variable
			// must be reset to null if there are no trailing comments.
			trailingComments = parser.state.leadingComments.slice(i);
			if (trailingComments.length === 0) {
				trailingComments = undefined;
			}
		}
	}

	parser.state.commentPreviousNode = node;

	if (trailingComments) {
		setComments(node, "trailingComments", trailingComments);
	}

	commentStack.push(node);
}
