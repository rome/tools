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
import {Number0, ob1Add, ob1Inc} from "@internal/ob1";
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

export const cssParser = createParser<CSSParserTypes>({
	diagnosticLanguage: "css",
	ignoreWhitespaceTokens: false,
	tokenize(parser: CSSParser, index: Number0): AnyCSSToken {
		const char = parser.getInputCharOnly(index);

		if (char === "/" && parser.getInputCharOnly(index, 1) === "*") {
			index = ob1Add(index, 2);
			let value = "";
			while (
				parser.getInputCharOnly(index) !== "*" &&
				parser.getInputCharOnly(index, 1) !== "/" &&
				!parser.isEOF(index)
			) {
				value += parser.getInputCharOnly(index);
				index = ob1Add(index, 1);
			}
			return parser.finishValueToken("Comment", value, ob1Add(index, 2));
		}

		if (isWhitespace(char)) {
			const endIndex = parser.readInputFrom(index, isWhitespace)[1];
			return parser.finishToken("Whitespace", endIndex);
		}

		if (char === '"') {
			return consumeStringToken(parser, index);
		}

		if (char === "#") {
			const nextChar = parser.getInputCharOnly(index, 1);
			if (
				isName(nextChar) ||
				isValidEscape(nextChar, parser.getInputCharOnly(index, 2))
			) {
				const [value, endIndex] = consumeName(parser, ob1Inc(index));
				const isIdent = isIdentifierStart(
					parser.getInputCharOnly(index, 1),
					parser.getInputCharOnly(index, 2),
					parser.getInputCharOnly(index, 3),
				);
				return parser.finishComplexToken(
					"Hash",
					{
						hashType: isIdent ? "id" : undefined,
						value,
					},
					endIndex,
				);
			}
			return parser.finishValueToken("Delim", char);
		}

		if (char === "'") {
			return consumeStringToken(parser, index);
		}

		if (char === "(") {
			return parser.finishToken("LeftParen");
		}

		if (char === ")") {
			return parser.finishToken("RightParen");
		}

		if (char === "+") {
			if (
				isNumberStart(
					char,
					parser.getInputCharOnly(index, 1),
					parser.getInputCharOnly(index, 2),
				)
			) {
				return consumeNumberToken(parser, index);
			}

			return parser.finishValueToken("Delim", char);
		}

		if (char === ",") {
			return parser.finishToken("Comma");
		}

		if (char === "-") {
			if (
				isNumberStart(
					char,
					parser.getInputCharOnly(index, 1),
					parser.getInputCharOnly(index, 2),
				)
			) {
				return consumeNumberToken(parser, index);
			}

			if (
				parser.getInputCharOnly(index, 1) === "-" &&
				parser.getInputCharOnly(index, 2) === ">"
			) {
				return parser.finishToken("CDC", ob1Add(index, 3));
			}

			if (
				isIdentifierStart(
					char,
					parser.getInputCharOnly(index, 1),
					parser.getInputCharOnly(index, 2),
				)
			) {
				return consumeIdentLikeToken(parser, index);
			}

			return parser.finishValueToken("Delim", char);
		}

		if (char === ".") {
			if (
				isNumberStart(
					char,
					parser.getInputCharOnly(index, 1),
					parser.getInputCharOnly(index, 2),
				)
			) {
				return consumeNumberToken(parser, index);
			}

			return parser.finishValueToken("Delim", char);
		}

		if (char === ":") {
			return parser.finishToken("Colon");
		}

		if (char === ";") {
			return parser.finishToken("Semi");
		}

		if (char === "<") {
			if (
				parser.getInputCharOnly(index, 1) === "!" &&
				parser.getInputCharOnly(index, 2) === "-" &&
				parser.getInputCharOnly(index, 3) === "-"
			) {
				return parser.finishToken("CDO", ob1Add(index, 4));
			}
			return parser.finishValueToken("Delim", char);
		}

		if (char === "@") {
			if (
				isIdentifierStart(
					parser.getInputCharOnly(index, 1),
					parser.getInputCharOnly(index, 2),
					parser.getInputCharOnly(index, 3),
				)
			) {
				const [value, endIndex] = consumeName(parser, ob1Inc(index));
				return parser.finishValueToken("AtKeyword", value, endIndex);
			}
			return parser.finishValueToken("Delim", char);
		}

		if (char === "[") {
			return parser.finishToken("LeftSquareBracket");
		}

		if (char === "\\") {
			if (isValidEscape(char, parser.getInputCharOnly(index, 1))) {
				return consumeIdentLikeToken(parser, index);
			}

			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.INVALID_ESCAPE,
			});
			return parser.finishValueToken("Delim", char);
		}

		if (char === "]") {
			return parser.finishToken("RightSquareBracket");
		}

		if (char === "{") {
			return parser.finishToken("LeftCurlyBracket");
		}

		if (char === "}") {
			return parser.finishToken("RightCurlyBracket");
		}

		if (isDigit(char)) {
			return consumeNumberToken(parser, index);
		}

		if (isNameStart(char)) {
			return consumeIdentLikeToken(parser, index);
		}

		return parser.finishValueToken("Delim", char);
	},
});

function getNewlineLength(parser: CSSParser, index: Number0): number {
	if (
		parser.getInputCharOnly(index) === Symbols.CarriageReturn &&
		parser.getInputCharOnly(index, 1) === Symbols.LineFeed
	) {
		return 2;
	}

	return 1;
}

function consumeBadURL(parser: CSSParser, index: Number0): Number0 {
	while (!parser.isEOF(index)) {
		if (parser.getInputCharOnly(index) === ")") {
			return ob1Inc(index);
		}

		if (
			isValidEscape(
				parser.getInputCharOnly(index),
				parser.getInputCharOnly(index, 1),
			)
		) {
			index = consumeEscaped(parser, index)[1];
		} else {
			index = ob1Inc(index);
		}
	}
	return index;
}

function consumeEscaped(parser: CSSParser, index: Number0): [string, Number0] {
	let value = "";
	index = ob1Add(index, 2);
	const lastChar = parser.getInputCharOnly(index, -1);

	if (isHexDigit(lastChar)) {
		let hexString = lastChar;
		let utf8Value = "";

		const [possibleHex] = parser.getInputRange(index, 5);
		for (const char of possibleHex) {
			if (!isHexDigit(char)) {
				break;
			}

			hexString += char;
			index = ob1Inc(index);
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

		if (isWhitespace(parser.getInputCharOnly(index))) {
			index = ob1Add(index, getNewlineLength(parser, index));
		}
	}

	return [value, index];
}

function consumeName(parser: CSSParser, index: Number0): [string, Number0] {
	let value = "";

	while (!parser.isEOF(index)) {
		const char1 = parser.getInputCharOnly(index);
		const char2 = parser.getInputCharOnly(index, 1);

		if (isName(char1)) {
			value += char1;
			index = ob1Inc(index);
			continue;
		}

		if (isValidEscape(char1, char2)) {
			const [newValue, newIndex] = consumeEscaped(parser, index);
			value += newValue;
			index = newIndex;
			continue;
		}

		break;
	}

	return [value, index];
}

function consumeNumber(
	parser: CSSParser,
	index: Number0,
): [Number0, number, string, string] {
	const char = parser.getInputCharOnly(index);
	let value = "";
	let type = "integer";

	if (char === "+" || char === "-") {
		value += char;
		index = ob1Inc(index);
	}

	while (isDigit(parser.getInputCharOnly(index))) {
		value += parser.getInputCharOnly(index);
		index = ob1Inc(index);
	}

	if (
		parser.getInputCharOnly(index) === "." &&
		isDigit(parser.getInputCharOnly(index, 1))
	) {
		value += parser.getInputCharOnly(index);
		index = ob1Inc(index);

		value += parser.getInputCharOnly(index);
		index = ob1Inc(index);

		type = "number";

		while (isDigit(parser.getInputCharOnly(index))) {
			value += parser.getInputCharOnly(index);
			index = ob1Inc(index);
		}
	}

	const char1 = parser.getInputCharOnly(index);
	const char2 = parser.getInputCharOnly(index, 1);
	const char3 = parser.getInputCharOnly(index, 2);

	if (char1 === "E" || char1 === "e") {
		if (isDigit(char2)) {
			value += parser.getInputCharOnly(index);
			index = ob1Inc(index);

			value += parser.getInputCharOnly(index);
			index = ob1Inc(index);
		} else if ((char2 === "-" || char2 === "+") && isDigit(char3)) {
			value += parser.getInputCharOnly(index);
			index = ob1Inc(index);

			value += parser.getInputCharOnly(index);
			index = ob1Inc(index);

			value += parser.getInputCharOnly(index);
			index = ob1Inc(index);

			while (isDigit(parser.getInputCharOnly(index))) {
				value += parser.getInputCharOnly(index);
				index = ob1Inc(index);
			}
		}
	}

	return [index, parseFloat(value), type, value];
}

function consumeIdentLikeToken(
	parser: CSSParser,
	index: Number0,
): Tokens["Function"] | Tokens["Ident"] | Tokens["URL"] | Tokens["BadURL"] {
	const [name, newIndex] = consumeName(parser, index);
	index = newIndex;
	let value = name;

	if (/url/gi.test(value) && parser.getInputCharOnly(index) === "(") {
		index = ob1Inc(index);

		while (
			isWhitespace(parser.getInputCharOnly(index)) &&
			isWhitespace(parser.getInputCharOnly(index, 1))
		) {
			index = ob1Inc(index);
		}

		if (
			parser.getInputCharOnly(index) === '"' ||
			parser.getInputCharOnly(index) === "'"
		) {
			return parser.finishValueToken("Function", value, index);
		}

		if (
			isWhitespace(parser.getInputCharOnly(index)) &&
			(parser.getInputCharOnly(index, 1) === '"' ||
			parser.getInputCharOnly(index, 1) === "'")
		) {
			return parser.finishValueToken("Function", value, ob1Add(index, 1));
		}

		return consumeURLToken(parser, index);
	}

	if (parser.getInputCharOnly(index) === "(") {
		return parser.finishValueToken("Function", value, ob1Inc(index));
	}

	return parser.finishValueToken("Ident", value, index);
}

function consumeStringToken(
	parser: CSSParser,
	index: Number0,
	endChar?: string,
): Tokens["String"] | Tokens["BadString"] {
	let value = "";

	if (!endChar) {
		[endChar, index] = parser.getInputChar(index);
	}

	while (!parser.isEOF(index)) {
		const char = parser.getInputCharOnly(index);
		const nextChar = parser.getInputCharOnly(index, 1);

		if (char === endChar) {
			return parser.finishValueToken("String", value, ob1Inc(index));
		} else if (parser.isEOF(index)) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_STRING,
			});
			return parser.finishValueToken("String", value, ob1Inc(index));
		} else if (isNewline(char)) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_STRING,
			});
			return parser.finishToken("BadString", index);
		} else if (char === "\\") {
			if (parser.isEOF(ob1Inc(index))) {
				continue;
			}
			if (isNewline(nextChar)) {
				index = ob1Add(index, getNewlineLength(parser, ob1Inc(index)));
			} else if (isValidEscape(char, nextChar)) {
				const [newValue, newIndex] = consumeEscaped(parser, index);
				value += newValue;
				index = newIndex;
			}
		} else {
			value += char;
			index = ob1Inc(index);
		}
	}

	return parser.finishValueToken("String", value, index);
}

function consumeNumberToken(
	parser: CSSParser,
	index: Number0,
): Tokens["Percentage"] | Tokens["Dimension"] | Tokens["Number"] {
	const [newIndex, numberValue, numberType, raw] = consumeNumber(parser, index);
	index = newIndex;

	if (
		isIdentifierStart(
			parser.getInputCharOnly(index),
			parser.getInputCharOnly(index, 1),
			parser.getInputCharOnly(index, 2),
		)
	) {
		const [unit, endIndex] = consumeName(parser, index);
		return parser.finishComplexToken(
			"Dimension",
			{
				numberType,
				unit,
				value: numberValue,
			},
			endIndex,
		);
	}

	if (parser.getInputCharOnly(index) === "%") {
		return parser.finishValueToken("Percentage", numberValue, ob1Add(index, 1));
	}

	return parser.finishComplexToken(
		"Number",
		{
			numberType,
			value: numberValue,
			raw,
		},
		index,
	);
}

function consumeURLToken(
	parser: CSSParser,
	index: Number0,
): Tokens["URL"] | Tokens["BadURL"] {
	let value = "";

	while (isWhitespace(parser.getInputCharOnly(index))) {
		index = ob1Inc(index);
	}

	while (!parser.isEOF(index)) {
		if (parser.getInputCharOnly(index) === ")") {
			return parser.finishValueToken("URL", value, ob1Inc(index));
		}

		if (parser.isEOF(index)) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_URL,
			});
			return parser.finishValueToken("URL", value);
		}

		if (isWhitespace(parser.getInputCharOnly(index))) {
			while (isWhitespace(parser.getInputCharOnly(index))) {
				index = ob1Inc(index);
			}

			if (parser.getInputCharOnly(index) === ")") {
				return parser.finishValueToken("URL", value, ob1Inc(index));
			}

			if (parser.isEOF(index)) {
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.UNTERMINATED_URL,
				});
				return parser.finishValueToken("URL", value);
			}

			index = consumeBadURL(parser, index);
			return parser.finishToken("BadURL", index);
		}

		if (
			parser.getInputCharOnly(index) === '"' ||
			parser.getInputCharOnly(index) === "'" ||
			parser.getInputCharOnly(index) === "("
		) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_URL,
			});
			index = consumeBadURL(parser, index);
			return parser.finishToken("BadURL", index);
		}

		if (parser.getInputCharOnly(index) === "\\") {
			if (
				isValidEscape(
					parser.getInputCharOnly(index),
					parser.getInputCharOnly(index),
				)
			) {
				const [newValue, newIndex] = consumeEscaped(parser, index);
				index = newIndex;
				value += newValue;
				continue;
			}

			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_URL,
			});
			index = consumeBadURL(parser, index);
			return parser.finishToken("BadURL", index);
		}

		value += parser.getInputCharOnly(index);
		index = ob1Inc(index);
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

	parser.finalize();

	return parser.finishNode(
		start,
		parser.finishRoot({
			type: "CSSRoot",
			body: rules,
		}),
	);
}
