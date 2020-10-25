import {PrinterOptions, printTokenToString} from "./Printer";
/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment, AnyNode} from "@internal/ast";
import {isTypeExpressionWrapperNode, isTypeNode} from "@internal/js-ast-utils";
import {CommentsConsumer} from "@internal/js-parser";
import {
	tokenizeComment,
	tokenizeLeadingComment,
	tokenizeTrailingComment,
} from "./builders/comments";
import builders from "./builders/index";
import * as n from "./node/index";
import {Token, Tokens, concat, hardline, indent, join, mark} from "./tokens";
import {ob1Get1} from "@internal/ob1";
import {isRoot} from "@internal/ast-utils";
import {DiagnosticLanguage} from "@internal/diagnostics";
import {inferDiagnosticLanguageFromRootAST} from "@internal/cli-diagnostics";

export type BuilderMethod<T extends AnyNode = AnyNode> = (
	builder: Builder,
	node: T,
	parent: AnyNode,
) => Token | never;

export type BuilderOptions = {
	typeAnnotations: boolean;
	format?: "pretty" | "compact";
	sourceMaps?: boolean;
	sourceText?: string;
	allowInterpreterDirective: boolean;
};

export default class Builder {
	constructor(
		opts: BuilderOptions,
		printOpts: PrinterOptions,
		comments: Array<AnyComment> = [],
	) {
		this.options = opts;
		this.printOptions = printOpts;
		this.comments = new CommentsConsumer(comments);
		this.printedComments = new Set();
		this.printStack = [];
		this.language = undefined;
	}

	private language: undefined | DiagnosticLanguage;
	public options: BuilderOptions;
	private printOptions: PrinterOptions;
	private comments: CommentsConsumer;
	private printedComments: Set<string>;
	private printStack: Array<AnyNode>;

	private getLanguage(): DiagnosticLanguage {
		const {language} = this;
		if (language === undefined) {
			throw new Error(
				"This operation needs to know the root language but none was found",
			);
		}
		return language;
	}

	public tokenize(node: undefined | AnyNode, parent: AnyNode): Token {
		if (node === undefined) {
			return "";
		}

		if (
			!this.options.typeAnnotations &&
			isTypeNode(node) &&
			!isTypeExpressionWrapperNode(node)
		) {
			return "";
		}

		const tokenizeNode = builders.assert(node.type);
		if (tokenizeNode === undefined) {
			throw new Error(
				`No known builder for node ${node.type} with parent ${parent.type}`,
			);
		}

		const oldRootType = this.language;
		let changedRootType = false;

		if (isRoot(node)) {
			changedRootType = true;
			this.language = inferDiagnosticLanguageFromRootAST(node);
		}
		this.printStack.push(node);

		let tokens: Tokens = [];

		// Print leading comments
		const leadingComments = this.getComments("leadingComments", node);
		if (leadingComments !== undefined) {
			let next = node;

			// Leading comments are traversed backward in order to get `next` right
			for (let i = leadingComments.length - 1; i >= 0; i--) {
				const comment = leadingComments[i];
				this.printedComments.add(comment.id);
				tokens.unshift(
					tokenizeLeadingComment(comment, next, this.getLanguage()),
				);
				next = comment;
			}
		}

		// Print node itself
		let printedNode = tokenizeNode(this, node, parent);

		const needsParens = n.needsParens(node, parent, this.printStack);

		this.printStack.pop();

		if (printedNode !== "") {
			if (this.options.sourceMaps && node.loc !== undefined) {
				printedNode = concat([
					mark(node.loc, "start"),
					printedNode,
					mark(node.loc, "end"),
				]);
			}

			if (needsParens) {
				printedNode = concat(["(", printedNode, ")"]);
			}
		}

		tokens.push(printedNode);

		// Print trailing comments
		const trailingComments = this.getComments("trailingComments", node);
		if (trailingComments !== undefined) {
			let previous = node;

			for (const comment of trailingComments) {
				this.printedComments.add(comment.id);
				tokens.push(
					tokenizeTrailingComment(comment, previous, this.getLanguage()),
				);
				previous = comment;
			}
		}

		if (changedRootType) {
			this.language = oldRootType;
		}

		return concat(tokens);
	}

	public tokenizeStatementList(nodes: Array<AnyNode>, parent: AnyNode): Token {
		if (nodes.length === 0) {
			return "";
		}

		const tokens: Array<Token> = [];

		for (let i = 0; i < nodes.length; i++) {
			const isLast = i === nodes.length - 1;
			const node = nodes[i];

			if (node.type === "JSEmptyStatement") {
				continue;
			}

			let printed = this.tokenize(node, parent);
			if (printed === "") {
				continue;
			}

			if (!isLast) {
				const nextNode = nodes[i + 1];

				if (this.getLinesBetween(node, nextNode) > 1) {
					printed = concat([printed, hardline]);
				}
			}

			tokens.push(printed);
		}

		return join(hardline, tokens);
	}

	public tokenizeInnerComments(node: AnyNode, shouldIndent: boolean): Token {
		const innerComments = this.getComments("innerComments", node);
		if (innerComments === undefined) {
			return "";
		}

		const tokens: Array<Token> = [];

		for (const comment of innerComments) {
			this.printedComments.add(comment.id);
			tokens.push(tokenizeComment(comment, this.getLanguage()));
		}

		return shouldIndent
			? indent(join(hardline, tokens), true)
			: join(hardline, tokens);
	}

	private getComments(
		kind: "leadingComments" | "trailingComments" | "innerComments",
		node: AnyNode,
		all: boolean = false,
	): undefined | Array<AnyComment> {
		if (!node) {
			return undefined;
		}

		const ids = node[kind];
		if (ids === undefined) {
			return undefined;
		}

		const comments = this.comments.getCommentsFromIds(ids);

		if (all) {
			return comments;
		} else {
			return comments.filter((comment) => !this.printedComments.has(comment.id));
		}
	}

	public getLinesBetween(a: AnyNode, b: AnyNode): number {
		if (a.loc === undefined || b.loc === undefined) {
			return 0;
		}

		let aEndLine = ob1Get1(a.loc.end.line);
		let bStartLine = ob1Get1(b.loc.start.line);

		// Simple cases:
		//  1. `a` and `b` are on the same line
		//  2. `a` and `b` are on their own line without empty lines between them
		if (bStartLine - aEndLine <= 1) {
			return bStartLine - aEndLine;
		}

		// If the are more than one line between `a` and `b`, the comment nodes must
		// be inspected to detect empty lines.
		//
		// In the following example, `getLinesBetween` should return `1`.
		//
		//     a;
		//     /* COMMENT */
		//     b;

		const aTrailingComments = this.getComments("trailingComments", a, true);
		const bLeadingComments = this.getComments("leadingComments", b, true);

		// Comments must be deduplicated because they are shared between nodes.
		// Walk them in order to calculate the nodes' boundaries.
		if (aTrailingComments !== undefined || bLeadingComments !== undefined) {
			const seenComments: Set<AnyComment> = new Set();

			// Expand `a` boundaries
			if (aTrailingComments !== undefined) {
				for (const comment of aTrailingComments) {
					seenComments.add(comment);

					if (comment.loc !== undefined) {
						aEndLine = Math.max(aEndLine, ob1Get1(comment.loc.end.line));
					}
				}
			}

			// Expand `b` boundaries
			if (bLeadingComments !== undefined) {
				for (const comment of bLeadingComments) {
					if (seenComments.has(comment)) {
						continue;
					}

					if (comment.loc !== undefined) {
						bStartLine = Math.min(bStartLine, ob1Get1(comment.loc.start.line));
					}
				}
			}
		}

		return bStartLine - aEndLine;
	}

	public alignCenter(token: Token, width: number): Token {
		const printed = printTokenToString(token, this.printOptions).code;
		const diff = width - printed.length;
		if (diff > 0) {
			return (
				" ".repeat(Math.floor(diff / 2)) +
				printed +
				" ".repeat(Math.ceil(diff / 2))
			);
		} else {
			return printed;
		}
	}
}
