import {
	AnyCSSToken,
	CSSParser,
	CSSParserOptions,
	CSSParserTypes,
	Tokens,
} from "./types";
import {
	TokenValues,
	createParser,
	isDigit,
	isHexDigit,
} from "@internal/parser-core";
import {descriptions} from "@internal/diagnostics";
import {ZeroIndexed} from "@internal/numbers";
import {
	Symbols,
	hexToUtf8,
	isIdentifierStart,
	isName,
	isNameStart,
	isNewline,
	isNumberStart,
	isValidEscape,
	isWhitespace,
} from "./utils";
import {CSSRoot} from "@internal/ast";
import {parseRules} from "@internal/css-parser/parser/rules";

function isntBlockCommentEnd(
	char: string,
	index: ZeroIndexed,
	input: string,
): boolean {
	const nextChar = input[index.valueOf() + 1];
	return !(char === "*" && nextChar === "/");
}

export const cssParser = createParser<CSSParserTypes>({
	diagnosticLanguage: "css",
	ignoreWhitespaceTokens: false,
	tokenize(parser: CSSParser, tokenizer): AnyCSSToken {
		const char = tokenizer.get();

		if (tokenizer.consume("/*")) {
			const value = tokenizer.read(isntBlockCommentEnd);
			tokenizer.assert("*/");
			return tokenizer.finishValueToken("Comment", value);
		}

		if (isWhitespace(char)) {
			tokenizer.read(isWhitespace);
			return tokenizer.finishToken("Whitespace");
		}

		if (tokenizer.consume('"')) {
			return consumeStringToken(parser, tokenizer, '"');
		}

		if (tokenizer.consume("'")) {
			return consumeStringToken(parser, tokenizer, "'");
		}

		if (tokenizer.startsWith("#")) {
			const nextChar = tokenizer.get(1);
			if (isName(nextChar) || isValidEscape(nextChar, tokenizer.get(2))) {
				tokenizer.assert("#");

				const isIdent = isIdentifierStart(
					tokenizer.get(),
					tokenizer.get(1),
					tokenizer.get(2),
				);
				const value = consumeName(parser, tokenizer);
				return tokenizer.finishComplexToken(
					"Hash",
					{
						hashType: isIdent ? "id" : undefined,
						value,
					},
				);
			}
		}

		if (tokenizer.consume("(")) {
			return tokenizer.finishToken("LeftParen");
		}

		if (tokenizer.consume(")")) {
			return tokenizer.finishToken("RightParen");
		}

		if (
			tokenizer.startsWith("+") &&
			isNumberStart("+", tokenizer.get(1), tokenizer.get(2))
		) {
			return consumeNumberToken(parser, tokenizer);
		}

		if (tokenizer.consume(",")) {
			return tokenizer.finishToken("Comma");
		}

		if (tokenizer.consume("-->")) {
			return tokenizer.finishToken("CDC");
		}

		if (tokenizer.startsWith("-")) {
			if (isNumberStart("-", tokenizer.get(1), tokenizer.get(2))) {
				return consumeNumberToken(parser, tokenizer);
			}

			if (isIdentifierStart("-", tokenizer.get(1), tokenizer.get(2))) {
				return consumeIdentLikeToken(parser, tokenizer);
			}
		}

		if (
			tokenizer.startsWith(".") &&
			isNumberStart(".", tokenizer.get(1), tokenizer.get(2))
		) {
			return consumeNumberToken(parser, tokenizer);
		}

		if (tokenizer.consume(":")) {
			return tokenizer.finishToken("Colon");
		}

		if (tokenizer.consume(";")) {
			return tokenizer.finishToken("Semi");
		}

		if (tokenizer.consume("<!--")) {
			return tokenizer.finishToken("CDO");
		}

		if (
			tokenizer.startsWith("@") &&
			isIdentifierStart(tokenizer.get(1), tokenizer.get(2), tokenizer.get(3))
		) {
			tokenizer.assert("@");
			const value = consumeName(parser, tokenizer);
			return tokenizer.finishValueToken("AtKeyword", value);
		}

		if (tokenizer.consume("[")) {
			return tokenizer.finishToken("LeftSquareBracket");
		}

		if (tokenizer.consume("\\")) {
			if (isValidEscape("\\", tokenizer.get())) {
				return consumeIdentLikeToken(parser, tokenizer);
			}

			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.INVALID_ESCAPE,
			});
			return tokenizer.finishValueToken("Delim", "\\");
		}

		if (tokenizer.consume("]")) {
			return tokenizer.finishToken("RightSquareBracket");
		}

		if (tokenizer.consume("{")) {
			return tokenizer.finishToken("LeftCurlyBracket");
		}

		if (tokenizer.consume("}")) {
			return tokenizer.finishToken("RightCurlyBracket");
		}

		if (isDigit(char)) {
			return consumeNumberToken(parser, tokenizer);
		}

		if (isNameStart(char)) {
			return consumeIdentLikeToken(parser, tokenizer);
		}

		tokenizer.take(1);
		return tokenizer.finishValueToken("Delim", char);
	},
});

function getNewlineLength(
	parser: CSSParser,
	tokenizer: CSSParser["tokenizer"],
): number {
	if (tokenizer.startsWith(`${Symbols.CarriageReturn}${Symbols.LineFeed}`)) {
		return 2;
	}

	return 1;
}

function consumeBadURL(parser: CSSParser, tokenizer: CSSParser["tokenizer"]) {
	while (!tokenizer.isEOF()) {
		if (tokenizer.consume(")")) {
			return;
		}

		if (isValidEscape(tokenizer.get(), tokenizer.get(1))) {
			consumeEscaped(parser, tokenizer);
		} else {
			tokenizer.take(1);
		}
	}
}

function consumeEscaped(
	parser: CSSParser,
	tokenizer: CSSParser["tokenizer"],
): string {
	tokenizer.assert("\\");
	const lastChar = tokenizer.take(1);
	let value = "";

	if (isHexDigit(lastChar)) {
		let hexString = lastChar;
		let utf8Value = "";

		const possibleHex = tokenizer.getRange(5);
		for (const char of possibleHex) {
			if (!isHexDigit(char)) {
				break;
			}

			hexString += char;
			tokenizer.take(1);
		}

		const hexNumber = parseInt(hexString, 16);
		if (
			hexNumber === 0 ||
			hexNumber > Symbols.MaxValue ||
			(hexNumber >= Symbols.SurrogateMin && hexNumber <= Symbols.SurrogateMax)
		) {
			utf8Value = Symbols.Replace;
		} else {
			utf8Value = hexToUtf8(hexString);
		}
		value += utf8Value;

		if (isWhitespace(tokenizer.get())) {
			tokenizer.take(getNewlineLength(parser, tokenizer));
		}
	}

	return value;
}

function consumeName(
	parser: CSSParser,
	tokenizer: CSSParser["tokenizer"],
): string {
	let value = "";

	while (!tokenizer.isEOF()) {
		const char1 = tokenizer.get();
		const char2 = tokenizer.get(1);

		if (isName(char1)) {
			value += char1;
			tokenizer.take(1);
			continue;
		}

		if (isValidEscape(char1, char2)) {
			const newValue = consumeEscaped(parser, tokenizer);
			value += newValue;
			continue;
		}

		break;
	}

	return value;
}

function consumeNumber(
	parser: CSSParser,
	tokenizer: CSSParser["tokenizer"],
): [number, string, string] {
	const char = tokenizer.get();
	let value = "";
	let type = "integer";

	if (tokenizer.consume("+") || tokenizer.consume("-")) {
		value += char;
	}

	value += tokenizer.read(isDigit);

	if (tokenizer.startsWith(".") && isDigit(tokenizer.get(1))) {
		value += tokenizer.take(2);
		type = "number";
		value += tokenizer.read(isDigit);
	}

	const char1 = tokenizer.get();
	const char2 = tokenizer.get(1);
	const char3 = tokenizer.get(2);

	if (char1 === "E" || char1 === "e") {
		if (isDigit(char2)) {
			value += tokenizer.take(2);
		} else if ((char2 === "-" || char2 === "+") && isDigit(char3)) {
			value += tokenizer.take(3);

			const digits = tokenizer.read(isDigit);
			value += digits;
		}
	}

	return [parseFloat(value), type, value];
}

function consumeIdentLikeToken(
	parser: CSSParser,
	tokenizer: CSSParser["tokenizer"],
): Tokens["Function"] | Tokens["Ident"] | Tokens["URL"] | Tokens["BadURL"] {
	const name = consumeName(parser, tokenizer);
	let value = name;

	if (/url/gi.test(value) && tokenizer.consume("(")) {
		while (isWhitespace(tokenizer.get()) && isWhitespace(tokenizer.get(1))) {
			tokenizer.take(1);
		}

		if (tokenizer.startsWith("'") || tokenizer.startsWith('"')) {
			return tokenizer.finishValueToken("Function", value);
		}

		if (
			isWhitespace(tokenizer.get()) &&
			(tokenizer.startsWith("'") || tokenizer.startsWith('"'))
		) {
			return tokenizer.finishValueToken("Function", value);
		}

		return consumeURLToken(parser, tokenizer);
	}

	if (tokenizer.consume("(")) {
		return tokenizer.finishValueToken("Function", value);
	}

	return tokenizer.finishValueToken("Ident", value);
}

function consumeStringToken(
	parser: CSSParser,
	tokenizer: CSSParser["tokenizer"],
	endChar: string,
): Tokens["String"] | Tokens["BadString"] {
	let value = "";

	while (!(tokenizer.isEOF() || tokenizer.startsWith(endChar))) {
		const char = tokenizer.get();
		const nextChar = tokenizer.get(1);

		if (isNewline(char)) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_STRING,
			});
			return tokenizer.finishToken("BadString");
		}

		if (char === "\\") {
			if (parser.isEOF(tokenizer.index.increment())) {
				continue;
			}

			if (isNewline(nextChar)) {
				tokenizer.take(getNewlineLength(parser, tokenizer));
			} else if (isValidEscape(char, nextChar)) {
				const newValue = consumeEscaped(parser, tokenizer);
				value += newValue;
			}
		} else {
			value += char;
			tokenizer.take(1);
		}
	}

	if (tokenizer.isEOF()) {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.UNTERMINATED_STRING,
		});
	} else {
		tokenizer.assert(endChar);
	}

	return tokenizer.finishValueToken("String", value);
}

function consumeNumberToken(
	parser: CSSParser,
	tokenizer: CSSParser["tokenizer"],
): Tokens["Percentage"] | Tokens["Dimension"] | Tokens["Number"] {
	const [numberValue, numberType, raw] = consumeNumber(parser, tokenizer);

	if (isIdentifierStart(tokenizer.get(), tokenizer.get(1), tokenizer.get(2))) {
		const unit = consumeName(parser, tokenizer);
		return tokenizer.finishComplexToken(
			"Dimension",
			{
				numberType,
				unit,
				value: numberValue,
			},
		);
	}

	if (tokenizer.consume("%")) {
		return tokenizer.finishValueToken("Percentage", numberValue);
	}

	return tokenizer.finishComplexToken(
		"Number",
		{
			numberType,
			value: numberValue,
			raw,
		},
	);
}

function consumeURLToken(
	parser: CSSParser,
	tokenizer: CSSParser["tokenizer"],
): Tokens["URL"] | Tokens["BadURL"] {
	let value = "";

	while (isWhitespace(tokenizer.get())) {
		tokenizer.take(1);
	}

	while (!tokenizer.isEOF()) {
		if (tokenizer.consume(")")) {
			return tokenizer.finishValueToken("URL", value);
		}

		if (tokenizer.isEOF()) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_URL,
			});
			return tokenizer.finishValueToken("URL", value);
		}

		if (isWhitespace(tokenizer.get())) {
			while (isWhitespace(tokenizer.get())) {
				tokenizer.take(1);
			}

			if (tokenizer.consume(")")) {
				return tokenizer.finishValueToken("URL", value);
			}

			if (tokenizer.isEOF()) {
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.UNTERMINATED_URL,
				});
				return tokenizer.finishValueToken("URL", value);
			}

			consumeBadURL(parser, tokenizer);
			return tokenizer.finishToken("BadURL");
		}

		if (
			tokenizer.startsWith('"') ||
			tokenizer.startsWith("'") ||
			tokenizer.startsWith("(")
		) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_URL,
			});
			consumeBadURL(parser, tokenizer);
			return tokenizer.finishToken("BadURL");
		}

		if (tokenizer.consume("\\")) {
			if (isValidEscape(tokenizer.get(), tokenizer.get())) {
				const newValue = consumeEscaped(parser, tokenizer);
				value += newValue;
				continue;
			}

			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_URL,
			});
			consumeBadURL(parser, tokenizer);
			return tokenizer.finishToken("BadURL");
		}

		value += tokenizer.take(1);
	}

	throw new Error("Unrecoverable state due to bad URL");
}

export function tokenizeCSS(opts: CSSParserOptions): TokenValues<Tokens>[] {
	return cssParser.create(opts).getAllTokens();
}

export function parseCSS(opts: CSSParserOptions): CSSRoot {
	const parser = cssParser.create(opts);
	const start = parser.getPosition();
	const rules = parseRules(parser, true);

	parser.finalize(false);

	return parser.finishNode(
		start,
		parser.finishRoot({
			type: "CSSRoot",
			body: rules,
		}),
	);
}
