import {isAlpha, isDigit} from "@internal/parser-core";
import {AnyCSSToken, CSSParser, Tokens} from "@internal/css-parser/types";
import {descriptions} from "@internal/diagnostics";
import {matchToken} from "@internal/css-parser/tokenizer";

export const Symbols = {
	CarriageReturn: "\r",
	Control: 0x80,
	FormFeed: "\f",
	LineFeed: "\n",
	MaxValue: 0x10ffff,
	Replace: "\ufffd",
	Space: " ",
	SurrogateMax: 0xdfff,
	SurrogateMin: 0xd800,
	Tab: "\t",
};

export function getCodePoint(char: string): number {
	if (!char) {
		return -1;
	}

	if (char.length === 1) {
		const point = char.codePointAt(0);
		if (point !== undefined) {
			return point;
		}
	}

	throw new Error("Input was not 1 character long");
}

export function hexToUtf8(hex: string): string {
	const match = hex.match(/.{1,2}/g);
	return match ? decodeURIComponent(`%${match.join("%")}`) : Symbols.Replace;
}

export function isIdentifierStart(
	char1: string,
	char2: string,
	char3: string,
): boolean {
	if (char1 === "-") {
		return isNameStart(char2) || char2 === "-" || isValidEscape(char2, char3);
	}

	if (isNameStart(char1)) {
		return true;
	}

	if (char1 === "\\") {
		return isValidEscape(char1, char2);
	}

	return false;
}

export function isName(char: string): boolean {
	return isNameStart(char) || isDigit(char) || char === "-";
}

export function isNameStart(char: string): boolean {
	return isAlpha(char) || isNonAscii(char) || char === "_";
}

export function isNewline(char: string): boolean {
	return (
		char === Symbols.LineFeed ||
		char === Symbols.CarriageReturn ||
		char === Symbols.FormFeed
	);
}

export function isNonAscii(char: string): boolean {
	return getCodePoint(char) >= Symbols.Control;
}

export function isNumberStart(
	char1: string,
	char2: string,
	char3: string,
): boolean {
	if (char1 === "+" || char1 === "-") {
		if (isDigit(char2)) {
			return true;
		}
		return char2 === "." && isDigit(char3);
	}

	if (char1 === ".") {
		return isDigit(char2);
	}

	return isDigit(char1);
}

export function isValidEscape(char1: string, char2?: string): boolean {
	if (char1 !== "\\" || !char2 || isNewline(char2)) {
		return false;
	}

	return true;
}

export function isWhitespace(char: string): boolean {
	return isNewline(char) || char === Symbols.Space || char === Symbols.Tab;
}

export function isCustomProperty(value: string): boolean {
	return value.startsWith("--");
}

export function getBlockStartTokenValue(
	parser: CSSParser,
	token: AnyCSSToken,
): string | undefined {
	switch (token.type) {
		case "LeftCurlyBracket":
			return "{";
		case "LeftParen":
			return "(";
		case "LeftSquareBracket":
			return "[";
		default: {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.INVALID_BLOCK_START,
			});
			return undefined;
		}
	}
}

export function getBlockEndTokenType(
	parser: CSSParser,
	token: AnyCSSToken,
): keyof Tokens | undefined {
	switch (token.type) {
		case "LeftCurlyBracket":
			return "RightCurlyBracket";
		case "LeftParen":
			return "RightParen";
		case "LeftSquareBracket":
			return "RightSquareBracket";
		default: {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.INVALID_BLOCK_START,
			});
			return undefined;
		}
	}
}

export function matchEndOfDeclaration(
	parser: CSSParser,
	endingTokenType: keyof Tokens,
): boolean {
	return (
		matchToken(parser, "EOF") ||
		matchToken(parser, "Semi") ||
		matchToken(parser, endingTokenType)
	);
}

// https://www.w3.org/TR/css-values-4/#css-wide-keywords
export const CSS_WIDE_KEYWORDS = ["unset", "initial", "inherit"];

// Given an Ident, tells if it's a valid <custom-ident>
// Source: https://www.w3.org/TR/css-values-4/#custom-idents
export function isCustomIdent(token: Tokens["Ident"]) {
	if (CSS_WIDE_KEYWORDS.includes(token.value)) {
		return false;
	}
	return true;
}

export const NOT = "not";
export const AND = "and";
export const OR = "or";

export type NOT = typeof NOT;
export type AND = typeof AND;
export type OR = typeof OR;

export const CONDITIONS = [NOT, AND, OR];
