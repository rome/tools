import {CSSParserOptions, Tokens} from "./types";
import {ParserOptions, createParser, isDigit} from "@romejs/parser-core";
import {DiagnosticCategory, descriptions} from "@romejs/diagnostics";
import {Number0, ob1Add, ob1Get, ob1Get0, ob1Inc} from "@romejs/ob1";

import {
	consumeBadURL,
	consumeChar,
	consumeEscaped,
	consumeName,
	consumeNumber,
	getChar,
	getNewlineLength,
	isIdentifierStart,
	isName,
	isNameStart,
	isNewline,
	isNumberStart,
	isValidEscape,
	isWhitespace,
} from "./utils";

export const createCSSParser = createParser((ParserCore) =>
	class CSSParser extends ParserCore<Tokens> {
		consumeDiagnosticCategory: DiagnosticCategory;
		options: ParserOptions;

		constructor(opts: CSSParserOptions) {
			super(opts, "parse/css", {});

			this.consumeDiagnosticCategory =
				opts.consumeDiagnosticCategory || "parse/json";
			this.ignoreWhitespaceTokens = false;
			this.options = opts;
		}

		consumeIdentLikeToken(
			index: Number0,
			input: string,
		): Tokens["Function"] | Tokens["Ident"] | Tokens["URL"] | Tokens["BadURL"] {
			const [newIndex, name] = consumeName(index, input);
			index = newIndex;
			let value = name;

			if (/url/gi.test(value) && getChar(index, input) === "(") {
				index = ob1Inc(index);

				while (
					isWhitespace(getChar(index, input)) &&
					isWhitespace(getChar(index, input, 1))
				) {
					index = ob1Inc(index);
				}

				if (getChar(index, input) === '"' || getChar(index, input) === "'") {
					return this.finishValueToken("Function", value, index);
				}

				if (
					isWhitespace(getChar(index, input)) &&
					(getChar(index, input, 1) === '"' || getChar(index, input, 1) === "'")
				) {
					return this.finishValueToken("Function", value, ob1Add(index, 1));
				}

				return this.consumeURLToken(index, input);
			}

			if (getChar(index, input) === "(") {
				return this.finishValueToken("Function", value, ob1Inc(index));
			}

			return this.finishValueToken("Ident", value, index);
		}

		consumeStringToken(
			index: Number0,
			input: string,
			endChar?: string,
		): Tokens["String"] | Tokens["BadString"] {
			let value = "";

			if (!endChar) {
				[index, endChar] = consumeChar(index, input);
			}

			while (ob1Get(index) < input.length) {
				const char = getChar(index, input);
				const nextChar = getChar(index, input, 1);

				if (char === endChar) {
					return this.finishValueToken("String", value, ob1Inc(index));
				} else if (this.isEOF(index)) {
					this.createDiagnostic({
						description: descriptions.CSS_PARSER.UNTERMINATED_STRING,
					});
					return this.finishValueToken("String", value, ob1Inc(index));
				} else if (isNewline(char)) {
					this.createDiagnostic({
						description: descriptions.CSS_PARSER.UNTERMINATED_STRING,
					});
					return this.finishToken("BadString", index);
				} else if (char === "\\") {
					if (this.isEOF(ob1Inc(index))) {
						continue;
					}
					if (isNewline(nextChar)) {
						index = ob1Add(index, getNewlineLength(ob1Inc(index), input));
					} else if (isValidEscape(char, nextChar)) {
						const [newIndex, newValue] = consumeEscaped(index, input);
						value += newValue;
						index = newIndex;
					}
				} else {
					value += char;
					index = ob1Inc(index);
				}
			}

			return this.finishValueToken("String", value, index);
		}

		consumeNumberToken(
			index: Number0,
			input: string,
		): Tokens["Percentage"] | Tokens["Dimension"] | Tokens["Number"] {
			const [newIndex, numberValue, numberType] = consumeNumber(index, input);
			index = newIndex;

			if (
				isIdentifierStart(
					getChar(index, input),
					getChar(index, input, 1),
					getChar(index, input, 2),
				)
			) {
				const [endIndex, unit] = consumeName(index, input);
				return this.finishComplexToken(
					"Dimension",
					{
						numberType,
						unit,
						value: numberValue,
					},
					endIndex,
				);
			}

			if (getChar(index, input) === "%") {
				return this.finishValueToken("Percentage", numberValue, index);
			}

			return this.finishComplexToken(
				"Number",
				{
					numberType,
					value: numberValue,
				},
				index,
			);
		}

		consumeURLToken(
			index: Number0,
			input: string,
		): Tokens["URL"] | Tokens["BadURL"] {
			let value = "";

			while (isWhitespace(getChar(index, input))) {
				index = ob1Inc(index);
			}

			while (ob1Get(index) < input.length) {
				if (getChar(index, input) === ")") {
					return this.finishValueToken("URL", value, ob1Inc(index));
				}

				if (this.isEOF(index)) {
					this.createDiagnostic({
						description: descriptions.CSS_PARSER.UNTERMINATED_URL,
					});
					return this.finishValueToken("URL", value);
				}

				if (isWhitespace(getChar(index, input))) {
					while (isWhitespace(getChar(index, input))) {
						index = ob1Inc(index);
					}

					if (getChar(index, input) === ")") {
						return this.finishValueToken("URL", value, ob1Inc(index));
					}

					if (this.isEOF(index)) {
						this.createDiagnostic({
							description: descriptions.CSS_PARSER.UNTERMINATED_URL,
						});
						return this.finishValueToken("URL", value);
					}

					[index] = consumeBadURL(index, input);
					return this.finishToken("BadURL", index);
				}

				if (
					getChar(index, input) === '"' ||
					getChar(index, input) === "'" ||
					getChar(index, input) === "("
				) {
					this.createDiagnostic({
						description: descriptions.CSS_PARSER.UNTERMINATED_URL,
					});
					[index] = consumeBadURL(index, input);
					return this.finishToken("BadURL", index);
				}

				if (getChar(index, input) === "\\") {
					if (isValidEscape(getChar(index, input), getChar(index, input))) {
						const [newIndex, newValue] = consumeEscaped(index, input);
						index = newIndex;
						value += newValue;
						continue;
					}

					this.createDiagnostic({
						description: descriptions.CSS_PARSER.UNTERMINATED_URL,
					});
					[index] = consumeBadURL(index, input);
					return this.finishToken("BadURL", index);
				}

				value += getChar(index, input);
				index = ob1Inc(index);
			}

			throw new Error("Unrecoverable state due to bad URL");
		}

		tokenize(index: Number0, input: string) {
			function getChar(offset = 0) {
				return input[ob1Get0(index) + offset];
			}

			if (getChar() === "/" && getChar(1) === "*") {
				index = ob1Add(index, 2);
				while (getChar() !== "*" && getChar(1) !== "/") {
					index = ob1Add(index, 1);
				}
			}

			if (isWhitespace(getChar())) {
				const endIndex = this.readInputFrom(index, isWhitespace)[1];
				return this.finishToken("Whitespace", endIndex);
			}

			if (getChar() === '"') {
				return this.consumeStringToken(index, input);
			}

			if (getChar() === "#") {
				if (isName(getChar(1)) || isValidEscape(getChar(1), getChar(2))) {
					const [endIndex, value] = consumeName(ob1Inc(index), input);
					const isIdent = isIdentifierStart(getChar(1), getChar(2), getChar(3));
					return this.finishComplexToken(
						"Hash",
						{
							hashType: isIdent ? "id" : undefined,
							value,
						},
						endIndex,
					);
				}
				return this.finishValueToken("Delim", getChar());
			}

			if (getChar() === "'") {
				return this.consumeStringToken(index, input);
			}

			if (getChar() === "(") {
				return this.finishToken("LeftParen");
			}

			if (getChar() === ")") {
				return this.finishToken("RightParen");
			}

			if (getChar() === "+") {
				if (isNumberStart(getChar(), getChar(1), getChar(2))) {
					return this.consumeNumberToken(index, input);
				}
				return this.finishValueToken("Delim", getChar());
			}

			if (getChar() === ",") {
				return this.finishToken("Comma");
			}

			if (getChar() === "-") {
				if (isNumberStart(getChar(), getChar(1), getChar(2))) {
					return this.consumeNumberToken(index, input);
				}

				if (getChar(1) === "-" && getChar(2) === ">") {
					return this.finishToken("CDC", ob1Add(index, 3));
				}

				if (isIdentifierStart(getChar(), getChar(1), getChar(2))) {
					return this.consumeIdentLikeToken(index, input);
				}

				return this.finishValueToken("Delim", getChar());
			}

			if (getChar() === ".") {
				if (isNumberStart(getChar(), getChar(1), getChar(2))) {
					return this.consumeNumberToken(index, input);
				}
				return this.finishValueToken("Delim", getChar());
			}

			if (getChar() === ":") {
				return this.finishToken("Colon");
			}

			if (getChar() === ";") {
				return this.finishToken("Semi");
			}

			if (getChar() === "@") {
				if (isIdentifierStart(getChar(1), getChar(2), getChar(3))) {
					const [endIndex, value] = consumeName(ob1Inc(index), input);
					return this.finishValueToken("AtKeyword", value, endIndex);
				}
				return this.finishValueToken("Delim", getChar());
			}

			if (getChar() === "[") {
				return this.finishToken("LeftSquareBracket");
			}

			if (getChar() === "\\") {
				if (isValidEscape(getChar(), getChar(1))) {
					return this.consumeIdentLikeToken(index, input);
				}
				this.createDiagnostic({
					description: descriptions.CSS_PARSER.INVALID_ESCAPE,
				});
				return this.finishValueToken("Delim", getChar());
			}

			if (getChar() === "]") {
				return this.finishToken("RightSquareBracket");
			}

			if (getChar() === "{") {
				return this.finishToken("LeftCurlyBracket");
			}

			if (getChar() === "}") {
				return this.finishToken("RightCurlyBracket");
			}

			if (isDigit(getChar())) {
				return this.consumeNumberToken(index, input);
			}

			if (isNameStart(getChar())) {
				return this.consumeIdentLikeToken(index, input);
			}

			return this.finishValueToken("Delim", getChar());
		}
	}
);
