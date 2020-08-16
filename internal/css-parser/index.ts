import {AnyCSSToken, AnyCSSValue, CSSParserOptions, Tokens} from "./types";
import {
	ParserCore,
	TokenValues,
	ValueToken,
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
import {
	CSSAtRule,
	CSSBlock,
	CSSDeclaration,
	CSSFunction,
	CSSRoot,
	CSSRule,
} from "@internal/ast";

type CSSParserTypes = {
	tokens: Tokens;
	state: {};
	options: CSSParserOptions;
	meta: void;
};

type CSSParser = ParserCore<CSSParserTypes>;

export const createCSSParser = createParser<CSSParserTypes>({
	diagnosticCategory: "parse/css",
	ignoreWhitespaceTokens: true,
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
): [Number0, number, string] {
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

	return [index, parseFloat(value), type];
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
	const [newIndex, numberValue, numberType] = consumeNumber(parser, index);
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

export function tokenizeCSS(opts: CSSParserOptions): Array<TokenValues<Tokens>> {
	return createCSSParser(opts).tokenizeAll();
}

export function parseCSS(opts: CSSParserOptions): CSSRoot {
	const parser = createCSSParser(opts);
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

function parseRules(
	parser: CSSParser,
	topLevel = false,
	endingTokenType?: keyof Tokens,
): Array<CSSAtRule | CSSRule> {
	const rules: Array<CSSAtRule | CSSRule> = [];
	while (!parser.matchToken("EOF")) {
		if (endingTokenType && parser.matchToken(endingTokenType)) {
			parser.nextToken();
			break;
		}

		if (parser.matchToken("Comment")) {
			parser.registerComment(
				parser.comments.createComment({
					type: "CommentBlock",
					loc: parser.finishLoc(parser.getPosition()),
					value: (parser.getToken() as Tokens["Comment"]).value,
				}),
			);
			parser.eatToken("Comment");
			continue;
		}

		if (parser.matchToken("Whitespace")) {
			parser.eatToken("Whitespace");
			continue;
		}

		if (parser.matchToken("CDO") || parser.matchToken("CDC")) {
			if (topLevel) {
				parser.nextToken();
				continue;
			}
			const rule = parseRule(parser);
			rule && rules.push(rule);
			continue;
		}

		if (parser.matchToken("AtKeyword")) {
			rules.push(parseAtRule(parser));
			continue;
		}

		const rule = parseRule(parser);
		if (rule !== undefined) {
			rules.push(rule);
		}
	}

	return rules;
}

function parseRule(parser: CSSParser): CSSRule | undefined {
	const start = parser.getPosition();
	const prelude: Array<AnyCSSValue> = [];
	while (!parser.matchToken("EOF")) {
		if (parser.matchToken("LeftCurlyBracket")) {
			return parser.finishNode(
				start,
				{
					type: "CSSRule",
					// TODO: Parse prelude according to selector grammar
					// https://www.w3.org/TR/css-syntax-3/#style-rules
					prelude,
					block: parseDeclartionBlock(parser),
				},
			);
		}
		const parsedValue = parseComponentValue(parser);
		parsedValue && prelude.push(parsedValue);
	}
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.UNEXPECTED_TOKEN,
	});
	return undefined;
}

function parseAtRule(parser: CSSParser): CSSAtRule {
	const start = parser.getPosition();
	const token = parser.expectToken("AtKeyword");
	const prelude: Array<AnyCSSValue> = [];
	const name = token.value;
	let block = undefined;
	while (true) {
		if (parser.matchToken("Semi")) {
			break;
		}
		if (parser.matchToken("EOF")) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_AT_RULE,
			});
			break;
		}
		if (parser.matchToken("LeftCurlyBracket")) {
			block = parseComplexBlock(parser);
			break;
		}
		const parsedValue = parseComponentValue(parser);
		parsedValue && prelude.push(parsedValue);
	}
	return parser.finishNode(
		start,
		{
			type: "CSSAtRule",
			name,
			prelude,
			block,
		},
	);
}

function parseSimpleBlock(parser: CSSParser): CSSBlock | undefined {
	const start = parser.getPosition();
	const startingToken = parser.getToken();
	const startingTokenValue = getBlockStartTokenValue(parser, startingToken);
	const endingTokenType = getBlockEndTokenType(parser, startingToken);
	let value: Array<AnyCSSValue | CSSAtRule | CSSDeclaration | undefined> = [];

	if (!endingTokenType) {
		return undefined;
	}

	parser.nextToken();

	while (true) {
		if (parser.matchToken(endingTokenType)) {
			parser.nextToken();
			break;
		}
		if (parser.matchToken("EOF")) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_BLOCK,
			});
			break;
		}
		const parsedValue = parseComponentValue(parser);
		parsedValue && value.push(parsedValue);
	}

	return parser.finishNode(
		start,
		{
			type: "CSSBlock",
			startingTokenValue,
			value,
		},
	);
}

function parseDeclartionBlock(parser: CSSParser): CSSBlock | undefined {
	const start = parser.getPosition();
	const startingToken = parser.getToken();
	const startingTokenValue = getBlockStartTokenValue(parser, startingToken);
	const endingTokenType = getBlockEndTokenType(parser, startingToken);
	let value: Array<AnyCSSValue | CSSAtRule | CSSDeclaration | undefined> = [];

	if (!endingTokenType) {
		return undefined;
	}

	parser.nextToken();

	value = parseDeclarations(parser, endingTokenType);

	return parser.finishNode(
		start,
		{
			type: "CSSBlock",
			startingTokenValue,
			value,
		},
	);
}

function parseComplexBlock(parser: CSSParser): CSSBlock | undefined {
	const start = parser.getPosition();
	const startingToken = parser.getToken();
	const startingTokenValue = getBlockStartTokenValue(parser, startingToken);
	const endingTokenType = getBlockEndTokenType(parser, startingToken);
	let value: Array<CSSAtRule | CSSRule> = [];

	if (!endingTokenType) {
		return undefined;
	}

	parser.nextToken();

	value = parseRules(parser, false, endingTokenType);

	return parser.finishNode(
		start,
		{
			type: "CSSBlock",
			startingTokenValue,
			value,
		},
	);
}

function parseComponentValue(parser: CSSParser): AnyCSSValue | undefined {
	if (parser.matchToken("Whitespace")) {
		parser.nextToken();
		return undefined;
	}

	if (
		parser.matchToken("LeftCurlyBracket") ||
		parser.matchToken("LeftParen") ||
		parser.matchToken("LeftSquareBracket")
	) {
		return parseSimpleBlock(parser);
	}

	if (parser.matchToken("Function")) {
		return parseFunction(parser);
	}

	const start = parser.getPosition();

	if (parser.matchToken("Dimension")) {
		const unit = (parser.getToken() as Tokens["Dimension"]).unit;
		const value = (parser.getToken() as Tokens["Dimension"]).value;
		parser.nextToken();
		return parser.finishNode(
			start,
			{
				type: "CSSDimension",
				unit,
				value,
			},
		);
	}

	if (parser.matchToken("Percentage")) {
		const value = (parser.getToken() as Tokens["Percentage"]).value;
		parser.nextToken();
		return parser.finishNode(
			start,
			{
				type: "CSSPercentage",
				value,
			},
		);
	}

	if (parser.matchToken("Ident")) {
		const value = (parser.getToken() as Tokens["Ident"]).value;
		parser.nextToken();
		return parser.finishNode(
			start,
			{
				type: "CSSIdentifier",
				value,
			},
		);
	}

	if (parser.matchToken("Number")) {
		const value = (parser.getToken() as Tokens["Number"]).value;
		parser.nextToken();
		return parser.finishNode(
			start,
			{
				type: "CSSNumber",
				value,
			},
		);
	}

	if (parser.matchToken("Colon")) {
		parser.nextToken();
		return parser.finishNode(
			start,
			{
				type: "CSSRaw",
				value: ":",
			},
		);
	}

	const value = (parser.getToken() as ValueToken<string, string>).value;
	parser.nextToken();
	return parser.finishNode(
		start,
		{
			type: "CSSRaw",
			value,
		},
	);
}

function parseFunction(parser: CSSParser): CSSFunction {
	const start = parser.getPosition();
	const token = parser.expectToken("Function");
	const name = token.value;
	const value = [];

	while (true) {
		if (parser.matchToken("RightParen")) {
			parser.nextToken();
			break;
		}
		if (parser.matchToken("EOF")) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_FUNCTION,
			});
			break;
		}
		const parsedValue = parseComponentValue(parser);
		parsedValue && value.push(parsedValue);
	}

	return parser.finishNode(
		start,
		{
			type: "CSSFunction",
			name,
			value,
		},
	);
}

function parseDeclarations(
	parser: CSSParser,
	endingTokenType?: keyof Tokens,
): Array<CSSAtRule | CSSDeclaration> {
	const declarations: Array<CSSAtRule | CSSDeclaration> = [];

	while (!parser.matchToken("EOF")) {
		if (parser.eatToken("Whitespace") || parser.eatToken("Semi")) {
			continue;
		}
		if (endingTokenType && parser.matchToken(endingTokenType)) {
			parser.nextToken();
			break;
		}
		if (parser.matchToken("AtKeyword")) {
			declarations.push(parseAtRule(parser));
			continue;
		}
		if (parser.matchToken("Ident")) {
			const declaration = parseDeclaration(parser);
			declaration && declarations.push(declaration);
			while (!parser.matchToken("Semi") && !parser.matchToken("EOF")) {
				const declaration = parseDeclaration(parser);
				declaration && declarations.push(declaration);
			}
			continue;
		}
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.INVALID_DECLARATION,
		});
		while (!parser.matchToken("Semi") && !parser.matchToken("EOF")) {
			parseComponentValue(parser);
		}
	}

	return declarations;
}

function parseDeclaration(parser: CSSParser): CSSDeclaration | undefined {
	while (!parser.matchToken("Semi")) {
		const currentToken = parser.getToken();
		if (currentToken.type !== "Ident") {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.INVALID_DECLARATION,
			});
			return undefined;
		}

		const name = currentToken.value;
		const start = parser.getPosition();
		let important = false;
		let value: Array<AnyCSSValue | undefined> = [];
		parser.nextToken();

		while (parser.matchToken("Whitespace")) {
			parser.eatToken("Whitespace");
		}
		if (!parser.matchToken("Colon")) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.INVALID_DECLARATION,
			});
			return undefined;
		}
		parser.nextToken();
		while (parser.matchToken("Whitespace")) {
			parser.eatToken("Whitespace");
		}
		while (!parser.matchToken("EOF")) {
			if (parser.matchToken("Semi")) {
				const lastTwoTokens = [...value].slice(-2);
				if (
					lastTwoTokens[0]?.type === "CSSRaw" &&
					lastTwoTokens[0].value === "!" &&
					lastTwoTokens[1]?.type === "CSSIdentifier" &&
					/^important$/i.test(lastTwoTokens[1].value)
				) {
					value = value.slice(0, -2);
					important = true;
				}
				return parser.finishNode(
					start,
					{
						type: "CSSDeclaration",
						important,
						name,
						value,
					},
				);
			}
			const parsedValue = parseComponentValue(parser);
			parsedValue && value.push(parsedValue);
		}
	}
	return undefined;
}

function getBlockStartTokenValue(
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

function getBlockEndTokenType(
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
