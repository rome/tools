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

export const cssParser = createParser<CSSParserTypes>({
	diagnosticLanguage: "css",
	ignoreWhitespaceTokens: false,
	tokenize(parser: CSSParser, index: ZeroIndexed): AnyCSSToken {
		const char = parser.getInputCharOnly(index);

		if (char === "/" && parser.getInputCharOnly(index.increment()) === "*") {
			index = index.add(2);
			let value = "";
			while (
				parser.getInputCharOnly(index) !== "*" &&
				parser.getInputCharOnly(index.increment()) !== "/" &&
				!parser.isEOF(index)
			) {
				value += parser.getInputCharOnly(index);
				index = index.add(1);
			}
			return parser.finishValueToken("Comment", value, index.add(2));
		}

		if (isWhitespace(char)) {
			const endIndex = parser.readInputFrom(index, isWhitespace)[1];
			return parser.finishToken("Whitespace", endIndex);
		}

		if (char === '"') {
			return consumeStringToken(parser, index);
		}

		if (char === "#") {
			const nextChar = parser.getInputCharOnly(index.increment());
			if (
				isName(nextChar) ||
				isValidEscape(nextChar, parser.getInputCharOnly(index.add(2)))
			) {
				const [value, endIndex] = consumeName(parser, index.increment());
				const isIdent = isIdentifierStart(
					parser.getInputCharOnly(index.increment()),
					parser.getInputCharOnly(index.add(2)),
					parser.getInputCharOnly(index.add(3)),
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
					parser.getInputCharOnly(index.increment()),
					parser.getInputCharOnly(index.add(2)),
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
					parser.getInputCharOnly(index.increment()),
					parser.getInputCharOnly(index.add(2)),
				)
			) {
				return consumeNumberToken(parser, index);
			}

			if (
				parser.getInputCharOnly(index.increment()) === "-" &&
				parser.getInputCharOnly(index.add(2)) === ">"
			) {
				return parser.finishToken("CDC", index.add(3));
			}

			if (
				isIdentifierStart(
					char,
					parser.getInputCharOnly(index.increment()),
					parser.getInputCharOnly(index.add(2)),
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
					parser.getInputCharOnly(index.increment()),
					parser.getInputCharOnly(index.add(2)),
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
				parser.getInputCharOnly(index.increment()) === "!" &&
				parser.getInputCharOnly(index.add(2)) === "-" &&
				parser.getInputCharOnly(index.add(3)) === "-"
			) {
				return parser.finishToken("CDO", index.add(4));
			}
			return parser.finishValueToken("Delim", char);
		}

		if (char === "@") {
			if (
				isIdentifierStart(
					parser.getInputCharOnly(index.increment()),
					parser.getInputCharOnly(index.add(2)),
					parser.getInputCharOnly(index.add(3)),
				)
			) {
				const [value, endIndex] = consumeName(parser, index.increment());
				return parser.finishValueToken("AtKeyword", value, endIndex);
			}
			return parser.finishValueToken("Delim", char);
		}

		if (char === "[") {
			return parser.finishToken("LeftSquareBracket");
		}

		if (char === "\\") {
			if (isValidEscape(char, parser.getInputCharOnly(index.increment()))) {
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

function getNewlineLength(parser: CSSParser, index: ZeroIndexed): number {
	if (
		parser.getInputCharOnly(index) === Symbols.CarriageReturn &&
		parser.getInputCharOnly(index.increment()) === Symbols.LineFeed
	) {
		return 2;
	}

	return 1;
}

function consumeBadURL(parser: CSSParser, index: ZeroIndexed): ZeroIndexed {
	while (!parser.isEOF(index)) {
		if (parser.getInputCharOnly(index) === ")") {
			return index.increment();
		}

		if (
			isValidEscape(
				parser.getInputCharOnly(index),
				parser.getInputCharOnly(index.increment()),
			)
		) {
			index = consumeEscaped(parser, index)[1];
		} else {
			index = index.increment();
		}
	}
	return index;
}

function consumeEscaped(
	parser: CSSParser,
	index: ZeroIndexed,
): [string, ZeroIndexed] {
	let value = "";
	index = index.add(2);
	const lastChar = parser.getInputCharOnly(index.decrement());

	if (isHexDigit(lastChar)) {
		let hexString = lastChar;
		let utf8Value = "";

		const possibleHex = parser.getRawInput(index, index.add(5));
		for (const char of possibleHex) {
			if (!isHexDigit(char)) {
				break;
			}

			hexString += char;
			index = index.increment();
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
			index = index.add(getNewlineLength(parser, index));
		}
	}

	return [value, index];
}

function consumeName(
	parser: CSSParser,
	index: ZeroIndexed,
): [string, ZeroIndexed] {
	let value = "";

	while (!parser.isEOF(index)) {
		const char1 = parser.getInputCharOnly(index);
		const char2 = parser.getInputCharOnly(index.increment());

		if (isName(char1)) {
			value += char1;
			index = index.increment();
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
	index: ZeroIndexed,
): [ZeroIndexed, number, string, string] {
	const char = parser.getInputCharOnly(index);
	let value = "";
	let type = "integer";

	if (char === "+" || char === "-") {
		value += char;
		index = index.increment();
	}

	while (isDigit(parser.getInputCharOnly(index))) {
		value += parser.getInputCharOnly(index);
		index = index.increment();
	}

	if (
		parser.getInputCharOnly(index) === "." &&
		isDigit(parser.getInputCharOnly(index.increment()))
	) {
		value += parser.getInputCharOnly(index);
		index = index.increment();

		value += parser.getInputCharOnly(index);
		index = index.increment();

		type = "number";

		while (isDigit(parser.getInputCharOnly(index))) {
			value += parser.getInputCharOnly(index);
			index = index.increment();
		}
	}

	const char1 = parser.getInputCharOnly(index);
	const char2 = parser.getInputCharOnly(index.increment());
	const char3 = parser.getInputCharOnly(index.add(2));

	if (char1 === "E" || char1 === "e") {
		if (isDigit(char2)) {
			value += parser.getInputCharOnly(index);
			index = index.increment();

			value += parser.getInputCharOnly(index);
			index = index.increment();
		} else if ((char2 === "-" || char2 === "+") && isDigit(char3)) {
			value += parser.getInputCharOnly(index);
			index = index.increment();

			value += parser.getInputCharOnly(index);
			index = index.increment();

			value += parser.getInputCharOnly(index);
			index = index.increment();

			while (isDigit(parser.getInputCharOnly(index))) {
				value += parser.getInputCharOnly(index);
				index = index.increment();
			}
		}
	}

	return [index, parseFloat(value), type, value];
}

function consumeIdentLikeToken(
	parser: CSSParser,
	index: ZeroIndexed,
): Tokens["Function"] | Tokens["Ident"] | Tokens["URL"] | Tokens["BadURL"] {
	const [name, newIndex] = consumeName(parser, index);
	index = newIndex;
	let value = name;

	if (/url/gi.test(value) && parser.getInputCharOnly(index) === "(") {
		index = index.increment();

		while (
			isWhitespace(parser.getInputCharOnly(index)) &&
			isWhitespace(parser.getInputCharOnly(index.increment()))
		) {
			index = index.increment();
		}

		if (
			parser.getInputCharOnly(index) === '"' ||
			parser.getInputCharOnly(index) === "'"
		) {
			return parser.finishValueToken("Function", value, index);
		}

		if (
			isWhitespace(parser.getInputCharOnly(index)) &&
			(parser.getInputCharOnly(index.increment()) === '"' ||
			parser.getInputCharOnly(index.increment()) === "'")
		) {
			return parser.finishValueToken("Function", value, index.add(1));
		}

		return consumeURLToken(parser, index);
	}

	if (parser.getInputCharOnly(index) === "(") {
		return parser.finishValueToken("Function", value, index.increment());
	}

	return parser.finishValueToken("Ident", value, index);
}

function consumeStringToken(
	parser: CSSParser,
	index: ZeroIndexed,
	endChar?: string,
): Tokens["String"] | Tokens["BadString"] {
	let value = "";

	if (!endChar) {
		[endChar, index] = parser.getInputChar(index);
	}

	while (!parser.isEOF(index)) {
		const char = parser.getInputCharOnly(index);
		const nextChar = parser.getInputCharOnly(index.increment());

		if (char === endChar) {
			return parser.finishValueToken("String", value, index.increment());
		} else if (parser.isEOF(index)) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_STRING,
			});
			return parser.finishValueToken("String", value, index.increment());
		} else if (isNewline(char)) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_STRING,
			});
			return parser.finishToken("BadString", index);
		} else if (char === "\\") {
			if (parser.isEOF(index.increment())) {
				continue;
			}
			if (isNewline(nextChar)) {
				index = index.add(getNewlineLength(parser, index.increment()));
			} else if (isValidEscape(char, nextChar)) {
				const [newValue, newIndex] = consumeEscaped(parser, index);
				value += newValue;
				index = newIndex;
			}
		} else {
			value += char;
			index = index.increment();
		}
	}

	return parser.finishValueToken("String", value, index);
}

function consumeNumberToken(
	parser: CSSParser,
	index: ZeroIndexed,
): Tokens["Percentage"] | Tokens["Dimension"] | Tokens["Number"] {
	const [newIndex, numberValue, numberType, raw] = consumeNumber(parser, index);
	index = newIndex;

	if (
		isIdentifierStart(
			parser.getInputCharOnly(index),
			parser.getInputCharOnly(index.increment()),
			parser.getInputCharOnly(index.add(2)),
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
		return parser.finishValueToken("Percentage", numberValue, index.add(1));
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
	index: ZeroIndexed,
): Tokens["URL"] | Tokens["BadURL"] {
	let value = "";

	while (isWhitespace(parser.getInputCharOnly(index))) {
		index = index.increment();
	}

	while (!parser.isEOF(index)) {
		if (parser.getInputCharOnly(index) === ")") {
			return parser.finishValueToken("URL", value, index.increment());
		}

		if (parser.isEOF(index)) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_URL,
			});
			return parser.finishValueToken("URL", value);
		}

		if (isWhitespace(parser.getInputCharOnly(index))) {
			while (isWhitespace(parser.getInputCharOnly(index))) {
				index = index.increment();
			}

			if (parser.getInputCharOnly(index) === ")") {
				return parser.finishValueToken("URL", value, index.increment());
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
		index = index.increment();
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
