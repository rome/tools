/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyBindingPattern,
	AnyNode,
	BreakStatement,
	ClassMethod,
	ContinueStatement,
	ObjectMethod,
	PatternMeta,
	ReturnStatement,
	TSDeclareMethod,
	ThrowStatement,
} from "@romejs/js-ast";
import {isBinary} from "@romejs/js-ast-utils";
import Builder, {BuilderMethod} from "../Builder";
import {
	Token,
	concat,
	group,
	hardline,
	ifBreak,
	indent,
	join,
	lineOrSpace,
	softline,
	space,
} from "../tokens";
import {hasInnerComments} from "./comments";

export function buildLabelStatementBuilder(
	prefix: string,
): BuilderMethod<BreakStatement | ContinueStatement> {
	return (builder, node): Token => {
		const tokens: Array<Token> = [prefix];

		if (node.label) {
			tokens.push(space, builder.tokenize(node.label, node));
		}

		tokens.push(";");

		return concat(tokens);
	};
}

export function buildThrowAndReturnStatementBuilder(
	prefix: string,
): BuilderMethod<ReturnStatement | ThrowStatement> {
	return (builder, node): Token => {
		const tokens: Array<Token> = [prefix];

		if (node.argument) {
			tokens.push(space);

			if (
				node.argument.type === "BinaryExpression" ||
				node.argument.type === "LogicalExpression" ||
				node.argument.type === "SequenceExpression"
			) {
				tokens.push(
					group(
						concat([
							ifBreak("("),
							indent(concat([softline, builder.tokenize(node.argument, node)])),
							softline,
							ifBreak(")"),
						]),
					),
				);
			} else {
				tokens.push(builder.tokenize(node.argument, node));
			}
		}

		tokens.push(";");

		return concat(tokens);
	};
}

export function printMethod(
	builder: Builder,
	node: TSDeclareMethod | ClassMethod | ObjectMethod,
): Token {
	const kind = node.kind;

	const tokens: Array<Token> = [];

	if (kind === "method" && node.head.generator === true) {
		tokens.push("*");
	}

	if (kind === "get" || kind === "set") {
		tokens.push(kind);
		tokens.push(space);
	}

	if (node.head.async === true) {
		tokens.push("async");
		tokens.push(space);
	}

	if (node.type === "TSDeclareMethod") {
		return concat([concat(tokens), builder.tokenize(node.head, node)]);
	}

	return concat([
		concat(tokens),
		builder.tokenize(node.key, node),
		builder.tokenize(node.head, node),
		space,
		builder.tokenize(node.body, node),
	]);
}

export function printBindingPatternParams(
	builder: Builder,
	node: AnyNode,
	params: Array<AnyBindingPattern>,
	rest: undefined | AnyBindingPattern,
): Token {
	if (params.length === 0 && rest === undefined) {
		if (hasInnerComments(node)) {
			return concat([
				"(",
				builder.tokenizeInnerComments(node, true),
				hardline,
				")",
			]);
		} else {
			return "()";
		}
	}

	const tokens: Array<Token> = [
		softline,
		join(
			concat([",", lineOrSpace]),
			params.map((param) => builder.tokenize(param, node)),
		),
	];

	if (rest) {
		if (params.length > 0) {
			tokens.push(",", lineOrSpace);
		}
		tokens.push("...", builder.tokenize(rest, node));
	}

	if (params.length > 0 && !rest) {
		tokens.push(ifBreak(","));
	}

	return concat(["(", indent(concat(tokens)), softline, ")"]);
}

export function printTSBraced(
	builder: Builder,
	node: AnyNode,
	members: Array<AnyNode>,
): Token {
	if (members.length === 0) {
		return group(
			concat(["{", builder.tokenizeInnerComments(node, true), softline, "}"]),
		);
	}

	return group(
		concat([
			"{",
			indent(
				concat([
					hardline,
					join(
						hardline,
						members.map((member, index) => {
							const printed = builder.tokenize(member, node);
							if (
								index > 0 &&
								builder.getLinesBetween(members[index - 1], member) > 1
							) {
								return concat([hardline, printed]);
							} else {
								return printed;
							}
						}),
					),
				]),
			),
			hardline,
			"}",
		]),
		true,
	);
}

export function printPatternMeta(
	builder: Builder,
	node: AnyNode,
	meta: undefined | PatternMeta,
): Token {
	if (builder.options.typeAnnotations && meta !== undefined) {
		const tokens: Array<Token> = [];

		if (meta.optional) {
			tokens.push("?");
		}

		if (meta.typeAnnotation) {
			tokens.push(":", space, builder.tokenize(meta.typeAnnotation, node));
		}

		return concat(tokens);
	} else {
		return "";
	}
}

export function printClause(
	builder: Builder,
	clause: AnyNode,
	parent: AnyNode,
): Token {
	if (clause.type === "EmptyStatement") {
		return ";";
	}

	if (clause.type === "BlockStatement") {
		return concat([space, builder.tokenize(clause, parent)]);
	}

	return indent(concat([lineOrSpace, builder.tokenize(clause, parent)]));
}

export function printCommaList(
	builder: Builder,
	nodes: Array<AnyNode>,
	parent: AnyNode,
): Token {
	return join(
		concat([",", lineOrSpace]),
		nodes.map((node) => builder.tokenize(node, parent)),
	);
}

export function printAssignment(
	builder: Builder,
	node: AnyNode,
	left: AnyNode,
	operator: Token,
	right: AnyNode,
): Token {
	const canBreak =
		right.type === "BinaryExpression" ||
		right.type === "LogicalExpression" ||
		right.type === "SequenceExpression" ||
		(right.type === "ConditionalExpression" && isBinary(right.test));

	return group(
		concat([
			builder.tokenize(left, node),
			operator,
			canBreak
				? group(indent(concat([lineOrSpace, builder.tokenize(right, node)])))
				: concat([space, builder.tokenize(right, node)]),
		]),
	);
}
