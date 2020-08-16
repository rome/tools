/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyComment,
	JSInterpreterDirective,
	JSNumericLiteral,
} from "@internal/ast";
import {Position, SourceLocation, TokenBase} from "@internal/parser-core";
import {
	JSParser,
	getPositionFromState,
	inScope,
	match,
	popScope,
	pushScope,
	pushToken,
	resetTokenizerLine,
	shouldCreateToken,
	shouldTokenizeJSX,
	unexpectedDiagnostic,
} from "../parser";
import {xhtmlEntityNameToChar} from "@internal/html-parser";
import {
	getFullCharCodeAt,
	isIdentifierChar,
	isIdentifierStart,
	isNewLine,
	lineBreak,
	lineBreakG,
	nonASCIIwhitespace,
	validateRegexFlags,
} from "@internal/js-parser-utils";
import {
	TokenLabels,
	TokenType,
	TokenTypes,
	keywords as keywordTypes,
	types as tt,
} from "./types";
import {TokContext, types as ct} from "./context";
import {ASCII_NAMES, UNICODE_MISTAKES} from "./unicodeMistakes";
import * as charCodes from "@internal/string-charcodes";
import {descriptions} from "@internal/diagnostics";
import {
	Number0,
	ob1Add,
	ob1Coerce0,
	ob1Dec,
	ob1Get0,
	ob1Inc,
	ob1Number0,
	ob1Number0Neg1,
	ob1Sub,
} from "@internal/ob1";
import {removeCarriageReturn} from "@internal/string-utils";

const HEX_NUMBER = /^[\da-fA-F]+$/;
const DECIMAL_NUMBER = /^\d+$/;

// The following character codes are forbidden from 'being
// an immediate sibling of JSNumericLiteralSeparator _
const forbiddenNumericSeparatorSiblings = {
	decBinOct: [
		charCodes.dot,
		charCodes.uppercaseB,
		charCodes.uppercaseE,
		charCodes.uppercaseO,
		charCodes.underscore,
		// multiple separators are not allowed
		charCodes.lowercaseB,
		charCodes.lowercaseE,
		charCodes.lowercaseO,
	],
	hex: [
		charCodes.dot,
		charCodes.uppercaseX,
		charCodes.underscore,
		// multiple separators are not allowed
		charCodes.lowercaseX,
	],
};

const allowedNumericSeparatorSiblingsBin = [
	// 0 - 1
	charCodes.digit0,
	charCodes.digit1,
];
const allowedNumericSeparatorSiblingsOct = [
	// 0 - 7
	...allowedNumericSeparatorSiblingsBin,
	charCodes.digit2,
	charCodes.digit3,
	charCodes.digit4,
	charCodes.digit5,
	charCodes.digit6,
	charCodes.digit7,
];
const allowedNumericSeparatorSiblingsDec = [
	// 0 - 9
	...allowedNumericSeparatorSiblingsOct,
	charCodes.digit8,
	charCodes.digit9,
];

const allowedNumericSeparatorSiblingsHex = [
	// 0 - 9, A - F, a - f,
	...allowedNumericSeparatorSiblingsDec,
	charCodes.uppercaseA,
	charCodes.uppercaseB,
	charCodes.uppercaseC,
	charCodes.uppercaseD,
	charCodes.uppercaseE,
	charCodes.uppercaseF,
	charCodes.lowercaseA,
	charCodes.lowercaseB,
	charCodes.lowercaseC,
	charCodes.lowercaseD,
	charCodes.lowercaseE,
	charCodes.lowercaseF,
];
const allowedNumericSeparatorSiblings = {
	bin: allowedNumericSeparatorSiblingsBin,
	oct: allowedNumericSeparatorSiblingsOct,
	dec: allowedNumericSeparatorSiblingsDec,
	hex: allowedNumericSeparatorSiblingsHex,
};

// Object type used to represent tokens. Note that normally, tokens
// simply exist as properties on the parser object. This is only
// used for the onToken callback and the external tokenizer.
export type Token = {
	type: TokenTypes;
	loc: SourceLocation;
};

export interface PublicToken extends TokenBase {
	type: TokenLabels;
}

export class RegExpTokenValue {
	constructor(pattern: string, flags: Set<string>) {
		this.pattern = pattern;
		this.flags = flags;
	}

	public pattern: string;
	public flags: Set<string>;
}

export class NumberTokenValue {
	constructor(value: number, format: JSNumericLiteral["format"]) {
		this.value = value;
		this.format = format;
	}

	public value: number;
	public format: JSNumericLiteral["format"];
}

// ## Tokenizer
function bumpIndex(parser: JSParser): Number0 {
	const index = ob1Inc(parser.state.index);
	parser.state.index = index;
	return index;
}

function getIndex(parser: JSParser): number {
	return ob1Get0(parser.state.index);
}

function codePointToString(code: number): string {
	// UTF-16 Decoding
	if (code <= 65_535) {
		return String.fromCharCode(code);
	} else {
		return String.fromCharCode(
			(code - 65_536 >> 10) + 55_296,
			(code - 65_536 & 1_023) + 56_320,
		);
	}
}

// Toggle strict mode. Re-reads the next number or string to please
// pedantic tests (`"use strict"; 010;` should fail).
export function setStrict(parser: JSParser, isStrict: boolean): void {
	pushScope(parser, "STRICT", isStrict);

	if (!match(parser, tt.num) && !match(parser, tt.string)) {
		return undefined;
	}

	parser.state.index = parser.state.startIndex;
	while (parser.state.index < parser.state.lineStartIndex) {
		parser.state.lineStartIndex = ob1Coerce0(
			parser.input.lastIndexOf("\n", ob1Get0(parser.state.lineStartIndex) - 2) +
			1,
		);
		parser.state.curLine = ob1Inc(parser.state.curLine);
	}
	nextToken(parser);
}

export function getCurContext(parser: JSParser): undefined | TokContext {
	return parser.state.context[parser.state.context.length - 1];
}

// Read a single token, updating the parser object's token-related
// properties.
export function nextToken(parser: JSParser): void {
	const curContext = getCurContext(parser);

	if (!curContext || !curContext.preserveSpace) {
		skipSpace(parser);
	}

	parser.state.containsOctal = false;
	parser.state.octalPosition = undefined;
	parser.state.startPos = getPositionFromState(parser);
	parser.state.startIndex = parser.state.index;

	if (parser.state.index >= parser.length) {
		finishToken(parser, tt.eof);
		return undefined;
	}

	if (curContext !== undefined && curContext.override) {
		curContext.override(parser);
	} else {
		readToken(parser, fullCharCodeAtPos(parser));
	}
}

function readToken(parser: JSParser, code: number): void {
	const matchedJSX = readJSXToken(parser, code);
	if (matchedJSX) {
		return undefined;
	} else {
		return readNormalToken(parser, code);
	}
}

function readJSXToken(parser: JSParser, code: number): boolean {
	if (inScope(parser, "PROPERTY_NAME")) {
		return false;
	}

	if (inScope(parser, "TYPE")) {
		return false;
	}

	if (!shouldTokenizeJSX(parser)) {
		return false;
	}

	const context = getCurContext(parser);

	if (context === ct.jsxInner) {
		readTokenJsx(parser);
		return true;
	}

	if (context === ct.jsxOpenTag || context === ct.jsxCloseTag) {
		if (isIdentifierStart(code)) {
			readTokenJsxWord(parser);
			return true;
		}

		if (code === charCodes.greaterThan) {
			bumpIndex(parser);
			finishToken(parser, tt.jsxTagEnd);
			return true;
		}

		if (
			(code === charCodes.quotationMark || code === charCodes.apostrophe) &&
			context === ct.jsxOpenTag
		) {
			readTokenJsxString(parser, code);
			return true;
		}
	}

	if (
		code === charCodes.lessThan &&
		parser.state.exprAllowed &&
		parser.input.charCodeAt(getIndex(parser) + 1) !== charCodes.exclamationMark
	) {
		bumpIndex(parser);
		finishToken(parser, tt.jsxTagStart);
		return true;
	}

	return false;
}

function readNormalToken(parser: JSParser, code: number): void {
	// JSIdentifier or keyword. '\uXXXX' sequences are allowed in
	// identifiers, so '\' also dispatches to that.
	if (isIdentifierStart(code) || code === charCodes.backslash) {
		readWord(parser);
	} else {
		getTokenFromCode(parser, code);
	}
}

function fullCharCodeAtPos(parser: JSParser): number {
	return getFullCharCodeAt(parser.input, getIndex(parser));
}

function pushComment(
	parser: JSParser,
	opts: {
		block: boolean;
		text: string;
		startPos: Position;
		endPos: Position;
	},
): AnyComment {
	const loc = parser.finishLocAt(opts.startPos, opts.endPos);
	let comment: AnyComment;
	if (opts.block) {
		comment = parser.comments.createComment({
			type: "CommentBlock",
			value: removeCarriageReturn(opts.text),
			loc,
		});
	} else {
		comment = parser.comments.createComment({
			type: "CommentLine",
			value: opts.text,
			loc,
		});
	}

	// We should enable jsx syntax when there's a comment with @\jsx
	if (opts.text.includes("@jsx")) {
		parser.meta.syntax.add("jsx");
	}

	if (!parser.meta.isLookahead) {
		parser.registerComment(comment);
	}

	if (shouldCreateToken(parser)) {
		pushToken(
			parser,
			{
				type: tt.comment,
				loc,
			},
		);
	}

	return comment;
}

function skipBlockComment(parser: JSParser): void {
	const startPos = getPositionFromState(parser);
	const startIndex = parser.state.index;
	parser.state.index = ob1Add(parser.state.index, 2);

	const endIndex = ob1Coerce0(parser.input.indexOf("*/", getIndex(parser)));

	if (endIndex === ob1Number0Neg1) {
		unexpectedDiagnostic(
			parser,
			{
				end: parser.getPositionFromIndex(ob1Sub(parser.state.index, 2)),
				description: descriptions.JS_PARSER.UNTERMINATED_BLOCK_COMMENT,
			},
		);
		return undefined;
	}

	// Skip */
	parser.state.index = ob1Add(endIndex, 2);

	lineBreakG.lastIndex = ob1Get0(startIndex);

	let match;
	while (
		(match = lineBreakG.exec(parser.input)) &&
		match.index < ob1Get0(parser.state.index)
	) {
		parser.state.curLine = ob1Inc(parser.state.curLine);
		resetTokenizerLine(parser);
		parser.state.lineStartIndex = ob1Coerce0(match.index + match[0].length);
	}

	pushComment(
		parser,
		{
			block: true,
			text: parser.getRawInput(ob1Add(startIndex, 2), endIndex),
			startPos,
			endPos: getPositionFromState(parser),
		},
	);
}

function parseLineComment(
	parser: JSParser,
	startSkip: number,
): {
	startPos: Position;
	endPos: Position;
	text: string;
} {
	const startIndex = parser.state.index;
	const startPos = getPositionFromState(parser);
	parser.state.index = ob1Add(parser.state.index, startSkip);
	let ch = parser.input.charCodeAt(getIndex(parser));
	if (parser.state.index < parser.length) {
		while (
			ch !== charCodes.lineFeed &&
			ch !== charCodes.carriageReturn &&
			ch !== charCodes.lineSeparator &&
			ch !== charCodes.paragraphSeparator &&
			bumpIndex(parser) < parser.length
		) {
			ch = parser.input.charCodeAt(getIndex(parser));
		}
	}
	const endPos = getPositionFromState(parser);
	const text = parser.getRawInput(
		ob1Add(startIndex, startSkip),
		parser.state.index,
	);

	return {
		startPos,
		endPos,
		text,
	};
}

export function skipLineComment(parser: JSParser, startSkip: number): AnyComment {
	const lineComment = parseLineComment(parser, startSkip);
	return pushComment(
		parser,
		{
			block: false,
			text: lineComment.text,
			startPos: lineComment.startPos,
			endPos: lineComment.endPos,
		},
	);
}

export function skipInterpreterDirective(
	parser: JSParser,
	startSkip: number,
): JSInterpreterDirective {
	const lineComment = parseLineComment(parser, startSkip);
	const loc = parser.finishLocAt(lineComment.startPos, lineComment.endPos);
	return {
		type: "JSInterpreterDirective",
		value: lineComment.text,
		loc,
	};
}

// Called at the start of the parse and after every token. Skips
// whitespace and comments, and.
function skipSpace(parser: JSParser): void {
	loop: while (parser.state.index < parser.length) {
		const ch = parser.input.charCodeAt(getIndex(parser));

		if (parser.state.lineStart) {
			if (ch === charCodes.space || ch === charCodes.tab) {
				parser.state.indentLevel = ob1Inc(parser.state.indentLevel);
			} else {
				parser.state.lineStart = false;
			}
		}

		if (
			ch === charCodes.carriageReturn &&
			parser.input.charCodeAt(ob1Get0(parser.state.index) + 1) ===
			charCodes.lineFeed
		) {
			bumpIndex(parser);
		}

		switch (ch) {
			case charCodes.space:
			case charCodes.nonBreakingSpace: {
				bumpIndex(parser);
				break;
			}

			case charCodes.carriageReturn:
			case charCodes.lineFeed:
			case charCodes.lineSeparator:
			case charCodes.paragraphSeparator: {
				bumpIndex(parser);
				parser.state.curLine = ob1Inc(parser.state.curLine);
				resetTokenizerLine(parser);
				break;
			}

			case charCodes.slash: {
				switch (parser.input.charCodeAt(getIndex(parser) + 1)) {
					case charCodes.asterisk: {
						// Break the loop and don't consume Flow comment code
						if (
							parser.input.charCodeAt(getIndex(parser) + 2) === charCodes.colon &&
							parser.input.charCodeAt(getIndex(parser) + 3) === charCodes.colon
						) {
							break loop;
						}

						skipBlockComment(parser);
						break;
					}

					case charCodes.slash: {
						skipLineComment(parser, 2);
						break;
					}

					default:
						break loop;
				}
				break;
			}

			default:
				if (
					(ch > charCodes.backSpace && ch < charCodes.shiftOut) ||
					(ch >= charCodes.oghamSpaceMark &&
					nonASCIIwhitespace.test(String.fromCharCode(ch)))
				) {
					bumpIndex(parser);
				} else {
					break loop;
				}
		}
	}
}

// Called at the end of every token. Sets `end`, `val`, and
// maintains `context` and `exprAllowed`, and skips the space after
// the token, so that the next one's `start` will point at the
// right position.
export function finishToken(
	parser: JSParser,
	type: TokenTypes,
	val?: unknown,
): void {
	parser.state.endPos = getPositionFromState(parser);
	parser.state.endIndex = parser.state.index;

	const prevType = parser.state.tokenType;
	parser.state.tokenType = type;
	parser.state.tokenValue = val;

	updateContext(parser, prevType);
}

function readTokenDot(parser: JSParser): void {
	const next = parser.input.charCodeAt(getIndex(parser) + 1);
	if (next >= charCodes.digit0 && next <= charCodes.digit9) {
		readNumber(parser, true);
		return undefined;
	}

	const next2 = parser.input.charCodeAt(getIndex(parser) + 2);
	if (next === charCodes.dot && next2 === charCodes.dot) {
		parser.state.index = ob1Add(parser.state.index, 3);
		finishToken(parser, tt.ellipsis);
	} else {
		bumpIndex(parser);
		finishToken(parser, tt.dot);
	}
}

function readTokenSlash(parser: JSParser): void {
	const next = parser.input.charCodeAt(getIndex(parser) + 1);

	// If this starts with /*:: then it's a Flow comment

	// TODO Flow also allows "flow-include" in place of "::"
	if (
		next === charCodes.asterisk &&
		parser.input.charCodeAt(getIndex(parser) + 2) === charCodes.colon &&
		parser.input.charCodeAt(getIndex(parser) + 3) === charCodes.colon
	) {
		parser.state.index = ob1Add(parser.state.index, 4);
		pushScope(parser, "FLOW_COMMENT");
		nextToken(parser);
		return;
	}

	// '/'
	if (parser.state.exprAllowed) {
		bumpIndex(parser);
		readRegexp(parser);
		return;
	}

	if (next === charCodes.equalsTo) {
		finishOp(parser, tt.assign, 2);
	} else {
		finishOp(parser, tt.slash, 1);
	}
}

function readTokenMultModulo(parser: JSParser, code: number): void {
	let next = parser.input.charCodeAt(getIndex(parser) + 1);

	// */ Is the end of a Flow comment
	if (
		code === charCodes.asterisk &&
		inScope(parser, "FLOW_COMMENT") &&
		next === charCodes.slash
	) {
		popScope(parser, "FLOW_COMMENT");
		parser.state.index = ob1Add(parser.state.index, 2);
		nextToken(parser);
		return;
	}

	// '%*'
	let type:
		| typeof tt["star"]
		| typeof tt["modulo"]
		| typeof tt["exponent"]
		| typeof tt["assign"] = code === charCodes.asterisk ? tt.star : tt.modulo;
	let width = 1;
	const exprAllowed = parser.state.exprAllowed;

	// Exponentiation operator **
	if (code === charCodes.asterisk && next === charCodes.asterisk) {
		width++;
		next = parser.input.charCodeAt(getIndex(parser) + 2);
		type = tt.exponent;
	}

	if (next === charCodes.equalsTo && !exprAllowed) {
		width++;
		type = tt.assign;
	}

	finishOp(parser, type, width);
}

function readTokenPipeAmp(parser: JSParser, code: number): void {
	// '|&'
	const next = parser.input.charCodeAt(getIndex(parser) + 1);

	if (next === code) {
		finishOp(
			parser,
			code === charCodes.verticalBar ? tt.logicalOR : tt.logicalAND,
			2,
		);
		return undefined;
	}

	// '|}'
	if (code === charCodes.verticalBar && next === charCodes.rightCurlyBrace) {
		finishOp(parser, tt.braceBarR, 2);
		return undefined;
	}

	if (next === charCodes.equalsTo) {
		finishOp(parser, tt.assign, 2);
		return undefined;
	}

	finishOp(
		parser,
		code === charCodes.verticalBar ? tt.bitwiseOR : tt.bitwiseAND,
		1,
	);
}

function readTokenCaret(parser: JSParser): void {
	// '^'
	const next = parser.input.charCodeAt(getIndex(parser) + 1);
	if (next === charCodes.equalsTo) {
		finishOp(parser, tt.assign, 2);
	} else {
		finishOp(parser, tt.bitwiseXOR, 1);
	}
}

function readTokenPlusMin(parser: JSParser, code: number): void {
	// '+-'
	const next = parser.input.charCodeAt(getIndex(parser) + 1);

	if (next === code) {
		if (
			next === charCodes.dash &&
			!parser.meta.inModule &&
			parser.input.charCodeAt(getIndex(parser) + 2) === charCodes.greaterThan &&
			(parser.state.lastEndIndex === ob1Number0 ||
			lineBreak.test(
				parser.getRawInput(parser.state.lastEndIndex, parser.state.index),
			))
		) {
			// A `-->` line comment
			skipLineComment(parser, 3);
			skipSpace(parser);
			nextToken(parser);
			return undefined;
		}
		finishOp(parser, tt.incDec, 2);
		return undefined;
	}

	if (next === charCodes.equalsTo) {
		finishOp(parser, tt.assign, 2);
	} else {
		finishOp(parser, tt.plusMin, 1);
	}
}

function readTokenLtGt(parser: JSParser, code: number): void {
	// '<>'
	const next = parser.input.charCodeAt(getIndex(parser) + 1);
	let size = 1;

	// we need to check if we're in a type to avoid interpreting the >> in Array<Array<string>> as a bitshift
	if (next === code && !inScope(parser, "TYPE")) {
		size =
			code === charCodes.greaterThan &&
			parser.input.charCodeAt(getIndex(parser) + 2) === charCodes.greaterThan
				? 3
				: 2;
		if (parser.input.charCodeAt(getIndex(parser) + size) === charCodes.equalsTo) {
			finishOp(parser, tt.assign, size + 1);
			return undefined;
		}
		finishOp(parser, tt.bitShift, size);
		return undefined;
	}

	if (
		code === charCodes.lessThan &&
		next === charCodes.exclamationMark &&
		!parser.meta.inModule &&
		parser.input.charCodeAt(getIndex(parser) + 2) === charCodes.dash &&
		parser.input.charCodeAt(getIndex(parser) + 3) === charCodes.dash
	) {
		// `<!--`, an XML-style comment that should be interpreted as a line comment
		skipLineComment(parser, 4);
		skipSpace(parser);
		nextToken(parser);
		return undefined;
	}

	if (next === charCodes.equalsTo) {
		// <= | >=
		size = 2;
	}

	finishOp(parser, tt.relational, size);
}

function readTokenEqExcl(parser: JSParser, code: number): void {
	// '=!'
	const next = parser.input.charCodeAt(getIndex(parser) + 1);
	if (next === charCodes.equalsTo) {
		finishOp(
			parser,
			tt.equality,
			parser.input.charCodeAt(getIndex(parser) + 2) === charCodes.equalsTo
				? 3
				: 2,
		);
		return undefined;
	}
	if (code === charCodes.equalsTo && next === charCodes.greaterThan) {
		// '=>'
		parser.state.index = ob1Add(parser.state.index, 2);
		finishToken(parser, tt.arrow);
		return undefined;
	}
	finishOp(parser, code === charCodes.equalsTo ? tt.eq : tt.bang, 1);
}

function readTokenQuestion(parser: JSParser): void {
	const next = parser.input.charCodeAt(getIndex(parser) + 1);
	const next2 = parser.input.charCodeAt(getIndex(parser) + 2);
	if (next === charCodes.questionMark && !inScope(parser, "TYPE")) {
		if (next2 === charCodes.equalsTo) {
			// '??='
			finishOp(parser, tt.assign, 3);
		} else {
			// '??'
			finishOp(parser, tt.nullishCoalescing, 2);
		}
	} else if (
		next === charCodes.dot &&
		!(next2 >= charCodes.digit0 && next2 <= charCodes.digit9)
	) {
		// '.' not followed by a number
		parser.state.index = ob1Add(parser.state.index, 2);
		finishToken(parser, tt.questionDot);
	} else {
		// '?'
		bumpIndex(parser);
		finishToken(parser, tt.question);
	}
}

function readTokenNumberSign(parser: JSParser): void {
	// Only tokenize a hash if we're inside of a class, or if we're the first character in the file (hashbang indicator)
	if (ob1Get0(parser.state.classLevel) > 0 || parser.state.index === ob1Number0) {
		bumpIndex(parser);
		finishToken(parser, tt.hash);
		return undefined;
	}

	// TODO make this a diagnostic, and advance to the next line if suspected hashbang
	unexpectedDiagnostic(
		parser,
		{
			description: descriptions.JS_PARSER.UNEXPECTED_HASH(
				parser.input[getIndex(parser) + 1] === "!",
			),
		},
	);
	bumpIndex(parser);
	nextToken(parser);
}

function getTokenFromCode(parser: JSParser, code: number): void {
	if (code === charCodes.digit0) {
		const next = parser.input.charCodeAt(getIndex(parser) + 1);

		// '0x', '0X' - hex number
		if (next === charCodes.lowercaseX || next === charCodes.uppercaseX) {
			readRadixNumber(parser, 16, "hex");
			return undefined;
		}

		// '0o', '0O' - octal number
		if (next === charCodes.lowercaseO || next === charCodes.uppercaseO) {
			readRadixNumber(parser, 8, "octal");
			return undefined;
		}

		// '0b', '0B' - binary number
		if (next === charCodes.lowercaseB || next === charCodes.uppercaseB) {
			readRadixNumber(parser, 2, "binary");
			return undefined;
		}
	}

	switch (code) {
		case charCodes.numberSign:
			return readTokenNumberSign(parser);

		// The interpretation of a dot depends on whether it is followed

		// by a digit or another two dots.
		case charCodes.dot: {
			readTokenDot(parser);
			return undefined;
		}

		// Punctuation tokens.
		case charCodes.leftParenthesis: {
			bumpIndex(parser);
			finishToken(parser, tt.parenL);
			return undefined;
		}

		case charCodes.rightParenthesis: {
			bumpIndex(parser);
			finishToken(parser, tt.parenR);
			return undefined;
		}

		case charCodes.semicolon: {
			bumpIndex(parser);
			finishToken(parser, tt.semi);
			return undefined;
		}

		case charCodes.comma: {
			bumpIndex(parser);
			finishToken(parser, tt.comma);
			return undefined;
		}

		case charCodes.leftSquareBracket: {
			bumpIndex(parser);
			finishToken(parser, tt.bracketL);
			return undefined;
		}

		case charCodes.rightSquareBracket: {
			bumpIndex(parser);
			finishToken(parser, tt.bracketR);
			return undefined;
		}

		case charCodes.leftCurlyBrace: {
			if (
				parser.input.charCodeAt(getIndex(parser) + 1) ===
				charCodes.verticalBar
			) {
				finishOp(parser, tt.braceBarL, 2);
			} else {
				bumpIndex(parser);
				finishToken(parser, tt.braceL);
			}
			return undefined;
		}

		case charCodes.rightCurlyBrace: {
			bumpIndex(parser);
			finishToken(parser, tt.braceR);
			return undefined;
		}

		case charCodes.colon: {
			if (parser.input.charCodeAt(getIndex(parser) + 1) === charCodes.colon) {
				finishOp(parser, tt.doubleColon, 2);
			} else {
				bumpIndex(parser);
				finishToken(parser, tt.colon);
			}
			return undefined;
		}

		case charCodes.questionMark: {
			readTokenQuestion(parser);
			return undefined;
		}

		case charCodes.atSign: {
			// The token @@ is the start of a Flow iterator name
			const next = parser.input.charCodeAt(getIndex(parser) + 1);
			if (next === charCodes.atSign) {
				parser.state.isIterator = true;
				readWord(parser);
			} else {
				bumpIndex(parser);
				finishToken(parser, tt.at);
			}
			return undefined;
		}

		case charCodes.graveAccent: {
			bumpIndex(parser);
			finishToken(parser, tt.backQuote);
			return undefined;
		}

		// Anything else beginning with a digit is an integer, octal

		// number, or float.
		case charCodes.digit0:
		case charCodes.digit1:
		case charCodes.digit2:
		case charCodes.digit3:
		case charCodes.digit4:
		case charCodes.digit5:
		case charCodes.digit6:
		case charCodes.digit7:
		case charCodes.digit8:
		case charCodes.digit9: {
			readNumber(parser, false);
			return undefined;
		}

		// Quotes produce strings.
		case charCodes.quotationMark:
		case charCodes.apostrophe: {
			readString(parser, code);
			return undefined;
		}

		// Operators are parsed inline in tiny state machines. '=' (charCodes.equalsTo) is

		// often referred to. `finishOp` simply skips the amount of

		// characters it is given as second argument, and returns a token

		// of the type given by its first argument.
		case charCodes.slash: {
			readTokenSlash(parser);
			return undefined;
		}

		case charCodes.percentSign:
		case charCodes.asterisk: {
			readTokenMultModulo(parser, code);
			return undefined;
		}

		case charCodes.verticalBar:
		case charCodes.ampersand: {
			readTokenPipeAmp(parser, code);
			return undefined;
		}

		case charCodes.caret: {
			readTokenCaret(parser);
			return undefined;
		}

		case charCodes.plusSign:
		case charCodes.dash: {
			readTokenPlusMin(parser, code);
			return undefined;
		}

		case charCodes.lessThan:
		case charCodes.greaterThan: {
			readTokenLtGt(parser, code);
			return undefined;
		}

		case charCodes.equalsTo:
		case charCodes.exclamationMark: {
			readTokenEqExcl(parser, code);
			return undefined;
		}

		case charCodes.tilde: {
			finishOp(parser, tt.tilde, 1);
			return undefined;
		}
	}

	const char = parser.input[getIndex(parser)];
	const unicodeMistake = UNICODE_MISTAKES.get(char);
	if (unicodeMistake !== undefined) {
		const [unicodeName, equivalentChar] = unicodeMistake;
		const equivalentName = ASCII_NAMES.get(equivalentChar);
		if (equivalentName === undefined) {
			throw new Error(`Expected ASCII name for ${equivalentChar}`);
		}

		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.UNEXPECTED_UNICODE_CHARACTER(
					char,
					unicodeName,
					equivalentChar,
					equivalentName,
				),
			},
		);

		// Read the token as the equivalent character
		getTokenFromCode(parser, equivalentChar.charCodeAt(0));
		return;
	}

	unexpectedDiagnostic(
		parser,
		{
			description: descriptions.PARSER_CORE.UNEXPECTED_CHARACTER(
				codePointToString(code),
			),
		},
	);

	// Skip unknown characters
	bumpIndex(parser);
	nextToken(parser);
}

function finishOp(parser: JSParser, type: TokenTypes, size: number): void {
	const str = parser.getRawInput(
		parser.state.index,
		ob1Add(parser.state.index, size),
	);
	parser.state.index = ob1Add(parser.state.index, size);
	finishToken(parser, type, str);
}

export function readRegexp(parser: JSParser): void {
	const start = parser.state.index;
	let escaped;
	let inClass;
	while (true) {
		if (parser.state.index >= parser.length) {
			unexpectedDiagnostic(
				parser,
				{
					end: parser.getPositionFromIndex(parser.state.index),
					description: descriptions.JS_PARSER.UNTERMINATED_REGEX,
				},
			);
			break;
		}

		const ch = parser.input.charAt(getIndex(parser));
		if (lineBreak.test(ch)) {
			if (
				parser.input.charAt(getIndex(parser) - 2) ===
				String.fromCharCode(charCodes.backslash) ||
				parser.input.charAt(getIndex(parser) - 3) ===
				String.fromCharCode(charCodes.backslash)
			) {
				const line = parser.input.slice(0, getIndex(parser));
				const backslashIndex = line.lastIndexOf(
					String.fromCharCode(charCodes.backslash),
				);
				unexpectedDiagnostic(
					parser,
					{
						end: parser.getPositionFromIndex(ob1Coerce0(backslashIndex)),
						description: descriptions.JS_PARSER.DANGLING_BACKSLASH_IN_REGEX,
					},
				);
				break;
			}
			unexpectedDiagnostic(
				parser,
				{
					end: parser.getPositionFromIndex(parser.state.index),
					description: descriptions.JS_PARSER.UNTERMINATED_REGEX,
				},
			);
			break;
		}

		if (escaped) {
			escaped = false;
		} else {
			if (ch === "[") {
				inClass = true;
			} else if (ch === "]" && inClass) {
				inClass = false;
			} else if (ch === "/" && !inClass) {
				break;
			}

			escaped = ch === "\\";
		}

		bumpIndex(parser);
	}

	const content = parser.getRawInput(start, parser.state.index);
	bumpIndex(parser);

	const rawMods = readWord1(parser);
	if (parser.state.escapePosition !== undefined) {
		unexpectedDiagnostic(
			parser,
			{
				index: parser.state.escapePosition,
				description: descriptions.JS_PARSER.UNICODE_ESCAPE_IN_REGEX_FLAGS,
			},
		);
	}
	const mods = validateRegexFlags(
		rawMods,
		(metadata, flagPosition) => {
			unexpectedDiagnostic(
				parser,
				{
					index: ob1Add(
						ob1Coerce0(getIndex(parser) - rawMods.length),
						flagPosition,
					),
					description: metadata,
				},
			);
		},
	);

	finishToken(parser, tt.regexp, new RegExpTokenValue(content, mods));
}

// Read an integer in the given radix. Return null if zero digits
// were read, the integer value otherwise. When `len` is given, this
// will return `null` unless the integer has exactly `len` digits.
function readInt(
	parser: JSParser,
	radix: number,
	len?: number,
): number | undefined {
	const start = parser.state.index;
	const forbiddenSiblings =
		radix === 16
			? forbiddenNumericSeparatorSiblings.hex
			: forbiddenNumericSeparatorSiblings.decBinOct;

	let allowedSiblings;
	if (radix === 16) {
		allowedSiblings = allowedNumericSeparatorSiblings.hex;
	} else if (radix === 10) {
		allowedSiblings = allowedNumericSeparatorSiblings.dec;
	} else if (radix === 8) {
		allowedSiblings = allowedNumericSeparatorSiblings.oct;
	} else {
		allowedSiblings = allowedNumericSeparatorSiblings.bin;
	}

	let total = 0;

	for (let i = 0, e = len === undefined ? Infinity : len; i < e; ++i) {
		const code = parser.input.charCodeAt(getIndex(parser));
		let val;

		const prev = parser.input.charCodeAt(getIndex(parser) - 1);
		const next = parser.input.charCodeAt(getIndex(parser) + 1);
		if (code === charCodes.underscore) {
			if (allowedSiblings.indexOf(next) === -1) {
				unexpectedDiagnostic(
					parser,
					{
						description: descriptions.JS_PARSER.INVALID_INT_TOKEN,
					},
				);
			}

			if (
				forbiddenSiblings.indexOf(prev) > -1 ||
				forbiddenSiblings.indexOf(next) > -1 ||
				Number.isNaN(next)
			) {
				unexpectedDiagnostic(
					parser,
					{
						description: descriptions.JS_PARSER.INVALID_INT_TOKEN,
					},
				);
			}

			// Ignore this _ character
			bumpIndex(parser);
			continue;
		}

		if (code >= charCodes.lowercaseA) {
			val = code - charCodes.lowercaseA + charCodes.lineFeed;
		} else if (code >= charCodes.uppercaseA) {
			val = code - charCodes.uppercaseA + charCodes.lineFeed;
		} else if (charCodes.isDigit(code)) {
			val = code - charCodes.digit0; // 0-9
		} else {
			val = Infinity;
		}

		if (val >= radix) {
			break;
		}

		bumpIndex(parser);
		total = total * radix + val;
	}

	if (
		parser.state.index === start ||
		(len !== undefined && getIndex(parser) - ob1Get0(start) !== len)
	) {
		return undefined;
	}

	return total;
}

function readRadixNumber(
	parser: JSParser,
	radix: number,
	format: JSNumericLiteral["format"],
): void {
	const start = parser.state.index;
	let isBigInt = false;

	parser.state.index = ob1Add(parser.state.index, 2); // 0x
	let val = readInt(parser, radix);
	if (val === undefined) {
		unexpectedDiagnostic(
			parser,
			{
				index: ob1Add(start, 2),
				description: descriptions.JS_PARSER.EXPECTED_NUMBER_IN_RADIX(radix),
			},
		);
		val = 0;
	}

	if (parser.input.charCodeAt(getIndex(parser)) === charCodes.lowercaseN) {
		bumpIndex(parser);
		isBigInt = true;
	}

	if (isIdentifierStart(fullCharCodeAtPos(parser))) {
		unexpectedDiagnostic(
			parser,
			{
				index: parser.state.index,
				description: descriptions.JS_PARSER.IDENTIFIER_AFTER_NUMBER,
			},
		);
	}

	if (isBigInt) {
		const str = parser.getRawInput(start, parser.state.index).replace(
			/[_n]/g,
			"",
		);
		finishToken(parser, tt.bigint, str);
		return undefined;
	}

	finishToken(parser, tt.num, new NumberTokenValue(val, format));
}

// Read an integer, octal integer, or floating-point number.
function readNumber(parser: JSParser, startsWithDot: boolean): void {
	const start = parser.state.startIndex;
	let isFloat = false;
	let isBigInt = false;

	if (!startsWithDot && readInt(parser, 10) === undefined) {
		unexpectedDiagnostic(
			parser,
			{
				index: parser.state.index,
				description: descriptions.JS_PARSER.INVALID_NUMBER,
			},
		);
	}

	let isOctal =
		ob1Get0(parser.state.index) - ob1Get0(start) >= 2 &&
		parser.input.charCodeAt(ob1Get0(start)) === charCodes.digit0;
	if (isOctal) {
		if (inScope(parser, "STRICT")) {
			unexpectedDiagnostic(
				parser,
				{
					index: parser.state.index,
					description: descriptions.JS_PARSER.LEGACY_OCTAL_IN_STRICT_MODE,
				},
			);
		}

		if (/[89]/.test(parser.getRawInput(start, parser.state.index))) {
			isOctal = false;
		}
	}

	let next = parser.input.charCodeAt(getIndex(parser));
	if (next === charCodes.dot && !isOctal) {
		bumpIndex(parser);
		readInt(parser, 10);
		isFloat = true;
		next = parser.input.charCodeAt(getIndex(parser));
	}

	if (
		(next === charCodes.uppercaseE || next === charCodes.lowercaseE) &&
		!isOctal
	) {
		next = parser.input.charCodeAt(ob1Get0(bumpIndex(parser)));

		if (next === charCodes.plusSign || next === charCodes.dash) {
			bumpIndex(parser);
		}

		if (readInt(parser, 10) === undefined) {
			unexpectedDiagnostic(
				parser,
				{
					index: parser.state.index,
					description: descriptions.JS_PARSER.INVALID_NUMBER,
				},
			);
		}

		isFloat = true;
		next = parser.input.charCodeAt(getIndex(parser));
	}

	if (next === charCodes.lowercaseN) {
		// Disallow floats and legacy octal syntax, new style octal ("0o") is handled in readRadixNumber
		if (isFloat) {
			unexpectedDiagnostic(
				parser,
				{
					index: parser.state.index,
					description: descriptions.JS_PARSER.DECIMAL_BIGINT,
				},
			);
		}

		if (isOctal) {
			unexpectedDiagnostic(
				parser,
				{
					index: parser.state.index,
					description: descriptions.JS_PARSER.OCTAL_BIGINT,
				},
			);
		}

		bumpIndex(parser);
		isBigInt = true;
	}

	if (isIdentifierStart(parser.input.codePointAt(getIndex(parser)))) {
		unexpectedDiagnostic(
			parser,
			{
				index: parser.state.index,
				description: descriptions.JS_PARSER.IDENTIFIER_AFTER_NUMBER,
			},
		);
	}

	// Remove "_" for numeric literal separator, and "n" for BigInts
	const str = parser.getRawInput(start, parser.state.index).replace(/[_n]/g, "");

	if (isBigInt) {
		finishToken(parser, tt.bigint, str);
		return undefined;
	}

	const num = isOctal ? parseInt(str, 8) : parseFloat(str);
	finishToken(
		parser,
		tt.num,
		new NumberTokenValue(num, isOctal ? "octal" : undefined),
	);
}

// Read a string value, interpreting backslash-escapes.
function readCodePoint(
	parser: JSParser,
	throwOnInvalid: boolean,
): number | undefined {
	const ch = parser.input.charCodeAt(getIndex(parser));
	let code;

	if (ch === charCodes.leftCurlyBrace) {
		const codePos = parser.state.index;
		bumpIndex(parser);
		code = readHexChar(
			parser,
			parser.input.indexOf("}", getIndex(parser)) - getIndex(parser),
			throwOnInvalid,
		);
		bumpIndex(parser);
		if (code === undefined) {
			// @ts-ignore
			parser.state.invalidTemplateEscapePosition--; // to point to the '\'' instead of the 'u'
		} else if (code > 1_114_111) {
			if (throwOnInvalid) {
				unexpectedDiagnostic(
					parser,
					{
						index: codePos,
						description: descriptions.JS_PARSER.OUT_OF_BOUND_CODE_POINT,
					},
				);
			} else {
				parser.state.invalidTemplateEscapePosition = ob1Sub(codePos, 2);
				return undefined;
			}
		}
	} else {
		code = readHexChar(parser, 4, throwOnInvalid);
	}
	return code;
}

function readString(parser: JSParser, quote: number): void {
	let out = "";
	let chunkStart = bumpIndex(parser);

	while (true) {
		if (parser.state.index >= parser.length) {
			unexpectedDiagnostic(
				parser,
				{
					end: parser.getPositionFromIndex(parser.state.index),
					description: descriptions.JS_PARSER.UNTERMINATED_STRING,
				},
			);
			break;
		}

		const ch = parser.input.charCodeAt(getIndex(parser));
		if (ch === quote) {
			break;
		}

		if (ch === charCodes.backslash) {
			out += parser.getRawInput(chunkStart, parser.state.index);
			out += readEscapedChar(parser, false);
			chunkStart = parser.state.index;
		} else if (
			ch === charCodes.lineSeparator ||
			ch === charCodes.paragraphSeparator
		) {
			bumpIndex(parser);
			parser.state.curLine = ob1Inc(parser.state.curLine);
		} else {
			if (isNewLine(ch)) {
				unexpectedDiagnostic(
					parser,
					{
						end: parser.getPositionFromIndex(parser.state.index),
						description: descriptions.JS_PARSER.UNTERMINATED_STRING,
					},
				);
			}
			bumpIndex(parser);
		}
	}

	out += parser.getRawInput(chunkStart, parser.state.index);
	bumpIndex(parser);
	finishToken(parser, tt.string, out);
}

// Reads template string tokens.
export function readTemplateToken(parser: JSParser): void {
	let out = "";
	let chunkStart = parser.state.index;
	let containsInvalid = false;

	while (true) {
		if (parser.state.index >= parser.length) {
			unexpectedDiagnostic(
				parser,
				{
					end: parser.getPositionFromIndex(parser.state.index),
					description: descriptions.JS_PARSER.UNTERMINATED_TEMPLATE,
				},
			);
			break;
		}

		const ch = parser.input.charCodeAt(getIndex(parser));
		if (
			ch === charCodes.graveAccent ||
			(ch === charCodes.dollarSign &&
			parser.input.charCodeAt(getIndex(parser) + 1) === charCodes.leftCurlyBrace)
		) {
			if (
				parser.state.index === parser.state.startIndex &&
				match(parser, tt.template)
			) {
				if (ch === charCodes.dollarSign) {
					parser.state.index = ob1Add(parser.state.index, 2);
					finishToken(parser, tt.dollarBraceL);
					return undefined;
				} else {
					bumpIndex(parser);
					finishToken(parser, tt.backQuote);
					return undefined;
				}
			}
			out += parser.getRawInput(chunkStart, parser.state.index);
			finishToken(parser, tt.template, containsInvalid ? undefined : out);
			return undefined;
		}

		if (ch === charCodes.backslash) {
			out += parser.getRawInput(chunkStart, parser.state.index);
			const escaped = readEscapedChar(parser, true);
			if (escaped === undefined) {
				containsInvalid = true;
			} else {
				out += escaped;
			}
			chunkStart = parser.state.index;
		} else if (isNewLine(ch)) {
			out += parser.getRawInput(chunkStart, parser.state.index);
			bumpIndex(parser);

			if (
				ch === charCodes.carriageReturn &&
				parser.input.charCodeAt(getIndex(parser)) === charCodes.lineFeed
			) {
				bumpIndex(parser);
			}

			switch (ch) {
				case charCodes.carriageReturn:
				case charCodes.lineFeed: {
					out += "\n";
					break;
				}

				default: {
					out += String.fromCharCode(ch);
					break;
				}
			}

			parser.state.curLine = ob1Inc(parser.state.curLine);
			resetTokenizerLine(parser);
			chunkStart = parser.state.index;
		} else {
			bumpIndex(parser);
		}
	}
}

// Used to read escaped characters
function readEscapedChar(
	parser: JSParser,
	inTemplate: boolean,
): string | undefined {
	const throwOnInvalid = !inTemplate;
	const ch = parser.input.charCodeAt(ob1Get0(bumpIndex(parser)));
	bumpIndex(parser);

	if (
		ch === charCodes.carriageReturn &&
		parser.input.charCodeAt(getIndex(parser)) === charCodes.lineFeed
	) {
		bumpIndex(parser);
	}

	switch (ch) {
		case charCodes.lowercaseN:
			return "\n";

		case charCodes.lowercaseR:
			return "\r";

		case charCodes.lowercaseX: {
			const code = readHexChar(parser, 2, throwOnInvalid);
			return code === undefined ? undefined : String.fromCharCode(code);
		}

		case charCodes.lowercaseU: {
			const code = readCodePoint(parser, throwOnInvalid);
			return code === undefined ? undefined : codePointToString(code);
		}

		case charCodes.lowercaseT:
			return "\t";

		case charCodes.lowercaseB:
			return "\b";

		case charCodes.lowercaseV:
			return "\x0b";

		case charCodes.lowercaseF:
			return "\f";

		case charCodes.carriageReturn:
		case charCodes.lineFeed: {
			parser.state.curLine = ob1Inc(parser.state.curLine);
			resetTokenizerLine(parser);
			return "";
		}

		default: {
			if (ch >= charCodes.digit0 && ch <= charCodes.digit7) {
				const codePos = ob1Dec(parser.state.index);
				const octalMatches = parser.input.substr(getIndex(parser) - 1, 3).match(
					/^[0-7]+/,
				)!;

				let octalStr = octalMatches[0];
				let octal = parseInt(octalStr, 8);
				if (octal > 255) {
					octalStr = octalStr.slice(0, -1);
					octal = parseInt(octalStr, 8);
				}

				if (octal > 0) {
					if (inTemplate) {
						parser.state.invalidTemplateEscapePosition = codePos;
						return undefined;
					} else if (inScope(parser, "STRICT")) {
						unexpectedDiagnostic(
							parser,
							{
								index: codePos,
								description: descriptions.JS_PARSER.OCTAL_IN_STRICT_MODE,
							},
						);
					} else if (!parser.state.containsOctal) {
						// These properties are only used to throw an error for an octal which occurs
						// in a directive which occurs prior to a "use strict" directive.
						parser.state.containsOctal = true;
						parser.state.octalPosition = codePos;
					}
				}

				parser.state.index = ob1Add(parser.state.index, octalStr.length - 1);
				return String.fromCharCode(octal);
			}

			return String.fromCharCode(ch);
		}
	}
}

// Used to read character escape sequences ('\x', '\u').
function readHexChar(
	parser: JSParser,
	len: number,
	throwOnInvalid: boolean,
): number | undefined {
	const start = parser.state.index;
	const n = readInt(parser, 16, len);

	if (n === undefined) {
		if (throwOnInvalid) {
			unexpectedDiagnostic(
				parser,
				{
					index: start,
					description: descriptions.JS_PARSER.BAD_HEX_ESCAPE,
				},
			);
			return 0;
		}

		const codePos = parser.state.index;
		parser.state.index = ob1Dec(codePos);
		parser.state.invalidTemplateEscapePosition = ob1Dec(codePos);
	}

	return n;
}

// Read an identifier, and return it as a string. Sets `parser.state.escapePosition`
// to an index if the word contained a '\u' escape.
//
// Incrementally adds only escaped chars, adding other chunks as-is
// as a micro-optimization.
function readWord1(parser: JSParser): string {
	parser.state.escapePosition = undefined;
	let word = "";
	let first = true;
	let chunkStart = parser.state.index;

	while (parser.state.index < parser.length) {
		const ch = fullCharCodeAtPos(parser);

		if (isIdentifierChar(ch)) {
			parser.state.index = ob1Add(parser.state.index, ch <= 65_535 ? 1 : 2);
		} else if (parser.state.isIterator && ch === charCodes.atSign) {
			bumpIndex(parser);
		} else if (ch === charCodes.backslash) {
			parser.state.escapePosition = parser.state.index;

			word += parser.getRawInput(chunkStart, parser.state.index);

			if (
				parser.input.charCodeAt(ob1Get0(bumpIndex(parser))) !==
				charCodes.lowercaseU
			) {
				unexpectedDiagnostic(
					parser,
					{
						index: parser.state.index,
						description: descriptions.JS_PARSER.EXPECTED_UNICODE_ESCAPE,
					},
				);
			}

			bumpIndex(parser);

			const esc = readCodePoint(parser, true);
			if (esc === undefined) {
				throw new Error("readCodePoint() should have thrown an error");
			}

			const isValid = first ? isIdentifierStart : isIdentifierChar;
			if (!isValid(esc)) {
				unexpectedDiagnostic(
					parser,
					{
						index: parser.state.index,
						description: descriptions.JS_PARSER.INVALID_UNICODE_ESCAPE,
					},
				);
			}

			word += codePointToString(esc);
			chunkStart = parser.state.index;
		} else {
			break;
		}

		first = false;
	}

	return word + parser.getRawInput(chunkStart, parser.state.index);
}

// Read an identifier or keyword token. Will check for reserved
// words when necessary.
function readWord(parser: JSParser): void {
	const word = readWord1(parser);

	// @ts-ignore: The value of keywordTypes has a generic parameter of `string` instead of the labels that we would actually find in keywordTypes
	let type: TokenTypes = keywordTypes.get(word) || tt.name;

	if (type.keyword !== undefined && parser.state.escapePosition !== undefined) {
		unexpectedDiagnostic(
			parser,
			{
				index: parser.state.escapePosition,
				description: descriptions.JS_PARSER.ESCAPE_SEQUENCE_IN_KEYWORD(word),
			},
		);
	}

	if (parser.state.isIterator && (!isIterator(word) || !inScope(parser, "TYPE"))) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.INVALID_IDENTIFIER_NAME(word),
			},
		);
	}

	finishToken(parser, type, word);
}

function isIterator(word: string): boolean {
	return word === "@@iterator" || word === "@@asyncIterator";
}

export function isBraceBlock(parser: JSParser, prevType: TokenType): boolean {
	const parent = getCurContext(parser);
	if (parent === ct.functionExpression || parent === ct.functionStatement) {
		return true;
	}
	if (
		prevType === tt.colon &&
		(parent === ct.braceStatement || parent === ct.braceExpression)
	) {
		return !parent.isExpr;
	}

	// The check for `tt.name && exprAllowed` detects whether we are

	// after a `yield` or `of` construct. See the `updateContext` for

	// `tt.name`.
	if (
		prevType === tt._return ||
		(prevType === tt.name && parser.state.exprAllowed)
	) {
		return lineBreak.test(
			parser.getRawInput(parser.state.lastEndIndex, parser.state.startIndex),
		);
	}

	if (
		prevType === tt._else ||
		prevType === tt.semi ||
		prevType === tt.eof ||
		prevType === tt.parenR ||
		prevType === tt.arrow
	) {
		return true;
	}

	if (prevType === tt.braceL) {
		return parent === ct.braceStatement;
	}

	if (prevType === tt._var || prevType === tt.name || prevType === tt._const) {
		return false;
	}

	if (prevType === tt.relational) {
		// `class C<T> { ... }`
		return true;
	}

	return !parser.state.exprAllowed;
}

function updateContext(parser: JSParser, prevType: TokenType) {
	if (match(parser, tt.braceL)) {
		const curContext = getCurContext(parser);
		if (curContext === ct.jsxOpenTag) {
			parser.state.context.push(ct.braceExpression);
		} else if (curContext === ct.jsxInner) {
			parser.state.context.push(ct.templateQuasi);
		} else {
			_updateContext(parser, prevType);
		}

		parser.state.exprAllowed = true;
	} else if (match(parser, tt.slash) && prevType === tt.jsxTagStart) {
		parser.state.context.length -= 2; // do not consider JSX expr -> JSX open tag -> ... anymore
		parser.state.context.push(ct.jsxCloseTag); // reconsider as closing tag context
		parser.state.exprAllowed = false;
	} else {
		_updateContext(parser, prevType);
	}
}

function _updateContext(parser: JSParser, prevType: TokenType): void {
	const type = parser.state.tokenType;

	if (
		type.keyword !== undefined &&
		(prevType === tt.dot || prevType === tt.questionDot)
	) {
		parser.state.exprAllowed = false;
	} else if (type.updateContext !== undefined) {
		type.updateContext(parser, prevType);
	} else {
		parser.state.exprAllowed = type.beforeExpr;
	}
}

// Reads inline JSX contents token.
function readTokenJsx(parser: JSParser): void {
	let out = "";
	let chunkStart = parser.state.index;
	while (true) {
		if (parser.state.index >= parser.length) {
			finishToken(parser, tt.eof);
			break;
		}

		const code = parser.input.charCodeAt(getIndex(parser));

		if (code === charCodes.lessThan || code === charCodes.leftCurlyBrace) {
			if (parser.state.index === parser.state.startIndex) {
				if (code === charCodes.lessThan && parser.state.exprAllowed) {
					bumpIndex(parser);
					return finishToken(parser, tt.jsxTagStart);
				}

				return getTokenFromCode(parser, code);
			}

			out += parser.getRawInput(chunkStart, parser.state.index);
			return finishToken(parser, tt.jsxText, out);
		}

		if (code === charCodes.ampersand) {
			out += parser.getRawInput(chunkStart, parser.state.index);
			out += readTokenJsxEntity(parser);
			chunkStart = parser.state.index;
			continue;
		}

		if (isNewLine(code)) {
			out += parser.getRawInput(chunkStart, parser.state.index);
			out += readTokenJsxNewLine(parser, true);
			chunkStart = parser.state.index;
		} else {
			bumpIndex(parser);
		}
	}
}

function readTokenJsxNewLine(parser: JSParser, normalizeCRLF: boolean): string {
	const ch = parser.input.charCodeAt(getIndex(parser));
	let out;
	bumpIndex(parser);

	if (
		ch === charCodes.carriageReturn &&
		parser.input.charCodeAt(getIndex(parser)) === charCodes.lineFeed
	) {
		bumpIndex(parser);
		out = normalizeCRLF ? "\n" : "\r\n";
	} else {
		out = String.fromCharCode(ch);
	}

	parser.state.curLine = ob1Inc(parser.state.curLine);
	resetTokenizerLine(parser);
	return out;
}

function readTokenJsxString(parser: JSParser, quote: number): void {
	let out = "";
	let chunkStart = bumpIndex(parser);
	while (true) {
		if (parser.state.index >= parser.length) {
			unexpectedDiagnostic(
				parser,
				{
					end: parser.getPositionFromIndex(parser.state.index),
					description: descriptions.JS_PARSER.UNTERMINATED_JSX_STRING,
				},
			);
			break;
		}

		const ch = parser.input.charCodeAt(getIndex(parser));
		if (ch === quote) {
			break;
		}

		if (ch === charCodes.ampersand) {
			out += parser.getRawInput(chunkStart, parser.state.index);
			out += readTokenJsxEntity(parser);
			chunkStart = parser.state.index;
		} else if (isNewLine(ch)) {
			out += parser.getRawInput(chunkStart, parser.state.index);
			out += readTokenJsxNewLine(parser, false);
			chunkStart = parser.state.index;
		} else {
			bumpIndex(parser);
		}
	}

	out += parser.getRawInput(chunkStart, parser.state.index);
	bumpIndex(parser);
	return finishToken(parser, tt.string, out);
}

function readTokenJsxEntity(parser: JSParser): string {
	let str = "";
	let count = 0;
	let entity;
	let ch = parser.input[getIndex(parser)];

	const startIndex = bumpIndex(parser);

	while (parser.state.index < parser.length && count++ < 10) {
		ch = parser.input[getIndex(parser)];
		bumpIndex(parser);
		if (ch === ";") {
			if (str[0] === "#") {
				if (str[1] === "x") {
					str = str.substr(2);
					if (HEX_NUMBER.test(str)) {
						entity = String.fromCodePoint(parseInt(str, 16));
					}
				} else {
					str = str.substr(1);
					if (DECIMAL_NUMBER.test(str)) {
						entity = String.fromCodePoint(parseInt(str, 10));
					}
				}
			} else {
				entity = xhtmlEntityNameToChar[str];
			}
			break;
		}
		str += ch;
	}

	if (entity === undefined) {
		parser.state.index = startIndex;
		return "&";
	} else {
		return entity;
	}
}

// Read a JSX identifier (valid tag or attribute name).
//
// Optimized version since JSX identifiers can't contain
// escape characters and so can be read as single slice.
// Also assumes that first character was already checked
// by isIdentifierStart in readToken.
function readTokenJsxWord(parser: JSParser): void {
	let ch;
	const start = parser.state.index;
	do {
		ch = parser.input.charCodeAt(ob1Get0(bumpIndex(parser)));
	} while (isIdentifierChar(ch) || ch === charCodes.dash);
	return finishToken(
		parser,
		tt.jsxName,
		parser.getRawInput(start, parser.state.index),
	);
}
