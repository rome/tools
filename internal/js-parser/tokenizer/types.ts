/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSParser} from "../parser";

// ## Token types
// The assignment of fine-grained, information-carrying type objects
// allows the tokenizer to store the information it has about a
// token in a way that is very cheap for the parser to look up.
// All token type variables start with an underscore, to make them
// easy to recognize.
// The `beforeExpr` property is used to disambiguate between regular
// expressions and divisions. It is set on all token types that can
// be followed by an expression (thus, a slash after them would be a
// regular expression).
//
// `isLoop` marks a keyword as starting a loop, which is important
// to know when parsing a label, in order to allow or disallow
// continue jumps to that label.
const beforeExpr = true;
const startsExpr = true;
const isLoop = true;
const isAssign = true;
const prefix = true;
const postfix = true;

type TokenOptions = {
	keyword?: string;
	beforeExpr?: boolean;
	startsExpr?: boolean;
	rightAssociative?: boolean;
	isLoop?: boolean;
	isAssign?: boolean;
	prefix?: boolean;
	postfix?: boolean;
	binop?: number;
};

export class TokenType<Label extends string = string> {
	constructor(label: Label, conf: TokenOptions = {}) {
		this.label = label;
		this.keyword = conf.keyword;
		this.beforeExpr = !!conf.beforeExpr;
		this.startsExpr = !!conf.startsExpr;
		this.rightAssociative = !!conf.rightAssociative;
		this.isLoop = !!conf.isLoop;
		this.isAssign = !!conf.isAssign;
		this.prefix = !!conf.prefix;
		this.postfix = !!conf.postfix;
		this.binop = typeof conf.binop === "number" ? conf.binop : undefined;
		this.updateContext = undefined;
	}

	public label: Label;
	public keyword: undefined | string;
	public beforeExpr: boolean;
	public startsExpr: boolean;
	public rightAssociative: boolean;
	public isLoop: boolean;
	public isAssign: boolean;
	public prefix: boolean;
	public postfix: boolean;
	public binop: undefined | number;
	public updateContext:
		| undefined
		| ((parser: JSParser, prevType: TokenType) => void);

	public getBinop(): number {
		const {binop} = this;
		if (binop === undefined) {
			throw new Error(`Token ${this.label} doesn't have a binop`);
		}
		return binop;
	}
}

export const keywords: Map<string, KeywordTokenType<string>> = new Map();

export class KeywordTokenType<Label extends string> extends TokenType<Label> {
	constructor(name: Label, options: TokenOptions = {}) {
		options.keyword = name;

		super(name, options);

		keywords.set(name, this);
	}
}

export class BinopTokenType<Label extends string> extends TokenType<Label> {
	constructor(name: Label, prec: number) {
		super(name, {beforeExpr, binop: prec});
	}
}

export const types = {
	num: new TokenType("num", {startsExpr}),
	bigint: new TokenType("bigint", {startsExpr}),
	regexp: new TokenType("regexp", {startsExpr}),
	string: new TokenType("string", {startsExpr}),
	name: new TokenType("name", {startsExpr}),
	eof: new TokenType("eof"),
	invalid: new TokenType("invalid"),
	comment: new TokenType("comment"),
	// Punctuation token types.
	bracketL: new TokenType("[", {beforeExpr, startsExpr}),
	bracketR: new TokenType("]"),
	braceL: new TokenType("{", {beforeExpr, startsExpr}),
	braceBarL: new TokenType("{|", {beforeExpr, startsExpr}),
	braceR: new TokenType("}"),
	braceBarR: new TokenType("|}"),
	parenL: new TokenType("(", {beforeExpr, startsExpr}),
	parenR: new TokenType(")"),
	comma: new TokenType(",", {beforeExpr}),
	semi: new TokenType(";", {beforeExpr}),
	colon: new TokenType(":", {beforeExpr}),
	doubleColon: new TokenType("::", {beforeExpr}),
	dot: new TokenType("."),
	question: new TokenType("?", {beforeExpr}),
	questionDot: new TokenType("?."),
	arrow: new TokenType("=>", {beforeExpr}),
	template: new TokenType("template"),
	ellipsis: new TokenType("...", {beforeExpr}),
	backQuote: new TokenType("`", {startsExpr}),
	dollarBraceL: new TokenType("${", {beforeExpr, startsExpr}),
	at: new TokenType("@"),
	hash: new TokenType("#"),
	// Operators. These carry several kinds of properties to help the
	// parser use them properly (the presence of these properties is
	// what categorizes them as operators).
	//
	// `binop`, when present, specifies that this operator is a binary
	// operator, and will refer to its precedence.
	//
	// `prefix` and `postfix` mark the operator as a prefix or postfix
	// unary operator.
	//
	// `isAssign` marks all of `=`, `+=`, `-=` etcetera, which act as
	// binary operators with a very low precedence, that should result
	// in JSAssignmentExpression nodes.
	eq: new TokenType("=", {beforeExpr, isAssign}),
	assign: new TokenType("_=", {beforeExpr, isAssign}),
	incDec: new TokenType("++/--", {prefix, postfix, startsExpr}),
	bang: new TokenType("!", {beforeExpr, prefix, startsExpr}),
	tilde: new TokenType("~", {beforeExpr, prefix, startsExpr}),
	nullishCoalescing: new BinopTokenType("??", 1),
	logicalOR: new BinopTokenType("||", 1),
	logicalAND: new BinopTokenType("&&", 2),
	bitwiseOR: new BinopTokenType("|", 3),
	bitwiseXOR: new BinopTokenType("^", 4),
	bitwiseAND: new BinopTokenType("&", 5),
	equality: new BinopTokenType("==/!=", 6),
	relational: new BinopTokenType("</>", 7),
	bitShift: new BinopTokenType("<</>>", 8),
	plusMin: new TokenType("+/-", {beforeExpr, binop: 9, prefix, startsExpr}),
	modulo: new BinopTokenType("%", 10),
	star: new BinopTokenType("*", 10),
	slash: new BinopTokenType("/", 10),
	exponent: new TokenType(
		"**",
		{
			beforeExpr,
			binop: 11,
			rightAssociative: true,
		},
	),
	jsxName: new TokenType("jsxName"),
	jsxText: new TokenType("jsxText", {beforeExpr: true}),
	jsxTagStart: new TokenType("jsxTagStart", {startsExpr: true}),
	jsxTagEnd: new TokenType("jsxTagEnd"),
	_break: new KeywordTokenType("break"),
	_case: new KeywordTokenType("case", {beforeExpr}),
	_catch: new KeywordTokenType("catch"),
	_continue: new KeywordTokenType("continue"),
	_debugger: new KeywordTokenType("debugger"),
	_default: new KeywordTokenType("default", {beforeExpr}),
	_do: new KeywordTokenType("do", {isLoop, beforeExpr}),
	_else: new KeywordTokenType("else", {beforeExpr}),
	_finally: new KeywordTokenType("finally"),
	_for: new KeywordTokenType("for", {isLoop}),
	_function: new KeywordTokenType("function", {startsExpr}),
	_if: new KeywordTokenType("if"),
	_return: new KeywordTokenType("return", {beforeExpr}),
	_switch: new KeywordTokenType("switch"),
	_throw: new KeywordTokenType("throw", {beforeExpr, prefix, startsExpr}),
	_try: new KeywordTokenType("try"),
	_var: new KeywordTokenType("var"),
	_const: new KeywordTokenType("const"),
	_while: new KeywordTokenType("while", {isLoop}),
	_with: new KeywordTokenType("with"),
	_new: new KeywordTokenType("new", {beforeExpr, startsExpr}),
	_this: new KeywordTokenType("this", {startsExpr}),
	_super: new KeywordTokenType("super", {startsExpr}),
	_class: new KeywordTokenType("class", {startsExpr}),
	_extends: new KeywordTokenType("extends", {beforeExpr}),
	_export: new KeywordTokenType("export"),
	_import: new KeywordTokenType("import", {startsExpr}),
	_null: new KeywordTokenType("null", {startsExpr}),
	_true: new KeywordTokenType("true", {startsExpr}),
	_false: new KeywordTokenType("false", {startsExpr}),
	_in: new KeywordTokenType("in", {beforeExpr, binop: 7}),
	_instanceof: new KeywordTokenType("instanceof", {beforeExpr, binop: 7}),
	_typeof: new KeywordTokenType("typeof", {beforeExpr, prefix, startsExpr}),
	_void: new KeywordTokenType("void", {beforeExpr, prefix, startsExpr}),
	_delete: new KeywordTokenType("delete", {beforeExpr, prefix, startsExpr}),
};

export type TokenTypes = typeof types[keyof typeof types];

// Get the labels of all tokens
// This is gross lol but better than having a manual union
export type TokenLabels = {
	[Key in keyof typeof types]: typeof types[Key]["label"]
}[keyof typeof types];
