import {CSSParserOptions, Tokens} from "./types";
import {
	ParserOptions,
	createParser,
	isDigit,
	isHexDigit,
} from "@romejs/parser-core";
import {DiagnosticCategory, descriptions} from "@romejs/diagnostics";
import {Number0, ob1Add, ob1Inc} from "@romejs/ob1";
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

		getNewlineLength(index: Number0): number {
			if (
				this.getInputCharOnly(index) === Symbols.CarriageReturn &&
				this.getInputCharOnly(index, 1) === Symbols.LineFeed
			) {
				return 2;
			}

			return 1;
		}

		consumeBadURL(index: Number0): Number0 {
			while (!this.isEOF(index)) {
				if (this.getInputCharOnly(index) === ")") {
					return ob1Inc(index);
				}

				if (
					isValidEscape(
						this.getInputCharOnly(index),
						this.getInputCharOnly(index, 1),
					)
				) {
					index = this.consumeEscaped(index)[1];
				} else {
					index = ob1Inc(index);
				}
			}
			return index;
		}

		consumeEscaped(index: Number0): [string, Number0] {
			let value = "";
			index = ob1Add(index, 2);
			const lastChar = this.getInputCharOnly(index, -1);

			if (isHexDigit(lastChar)) {
				let hexString = lastChar;
				let utf8Value = "";

				const [possibleHex] = this.getInputRange(index, 5);
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
					(hexNumber >= Symbols.SurrogateMin &&
					hexNumber <= Symbols.SurrogateMax)
				) {
					utf8Value = Symbols.Replace;
				} else {
					utf8Value = hexToUtf8(hexString);
				}
				value += utf8Value;

				if (isWhitespace(this.getInputCharOnly(index))) {
					index = ob1Add(index, this.getNewlineLength(index));
				}
			}

			return [value, index];
		}

		consumeName(index: Number0): [string, Number0] {
			let value = "";

			while (!this.isEOF(index)) {
				const char1 = this.getInputCharOnly(index);
				const char2 = this.getInputCharOnly(index, 1);

				if (isName(char1)) {
					value += char1;
					index = ob1Inc(index);
					continue;
				}

				if (isValidEscape(char1, char2)) {
					const [newValue, newIndex] = this.consumeEscaped(index);
					value += newValue;
					index = newIndex;
					continue;
				}

				break;
			}

			return [value, index];
		}

		consumeNumber(index: Number0): [Number0, number, string] {
			const char = this.getInputCharOnly(index);
			let value = "";
			let type = "integer";

			if (char === "+" || char === "-") {
				value += char;
				index = ob1Inc(index);
			}

			while (isDigit(this.getInputCharOnly(index))) {
				value += this.getInputCharOnly(index);
				index = ob1Inc(index);
			}

			if (
				this.getInputCharOnly(index) === "." &&
				isDigit(this.getInputCharOnly(index, 1))
			) {
				value += this.getInputCharOnly(index);
				index = ob1Inc(index);

				value += this.getInputCharOnly(index);
				index = ob1Inc(index);

				type = "number";

				while (isDigit(this.getInputCharOnly(index))) {
					value += this.getInputCharOnly(index);
					index = ob1Inc(index);
				}
			}

			const char1 = this.getInputCharOnly(index);
			const char2 = this.getInputCharOnly(index, 1);
			const char3 = this.getInputCharOnly(index, 2);

			if (char1 === "E" || char1 === "e") {
				if (isDigit(char2)) {
					value += this.getInputCharOnly(index);
					index = ob1Inc(index);

					value += this.getInputCharOnly(index);
					index = ob1Inc(index);
				} else if ((char2 === "-" || char2 === "+") && isDigit(char3)) {
					value += this.getInputCharOnly(index);
					index = ob1Inc(index);

					value += this.getInputCharOnly(index);
					index = ob1Inc(index);

					value += this.getInputCharOnly(index);
					index = ob1Inc(index);

					while (isDigit(this.getInputCharOnly(index))) {
						value += this.getInputCharOnly(index);
						index = ob1Inc(index);
					}
				}
			}

			return [index, parseFloat(value), type];
		}

		consumeIdentLikeToken(
			index: Number0,
		): Tokens["Function"] | Tokens["Ident"] | Tokens["URL"] | Tokens["BadURL"] {
			const [name, newIndex] = this.consumeName(index);
			index = newIndex;
			let value = name;

			if (/url/gi.test(value) && this.getInputCharOnly(index) === "(") {
				index = ob1Inc(index);

				while (
					isWhitespace(this.getInputCharOnly(index)) &&
					isWhitespace(this.getInputCharOnly(index, 1))
				) {
					index = ob1Inc(index);
				}

				if (
					this.getInputCharOnly(index) === '"' ||
					this.getInputCharOnly(index) === "'"
				) {
					return this.finishValueToken("Function", value, index);
				}

				if (
					isWhitespace(this.getInputCharOnly(index)) &&
					(this.getInputCharOnly(index, 1) === '"' ||
					this.getInputCharOnly(index, 1) === "'")
				) {
					return this.finishValueToken("Function", value, ob1Add(index, 1));
				}

				return this.consumeURLToken(index);
			}

			if (this.getInputCharOnly(index) === "(") {
				return this.finishValueToken("Function", value, ob1Inc(index));
			}

			return this.finishValueToken("Ident", value, index);
		}

		consumeStringToken(
			index: Number0,
			endChar?: string,
		): Tokens["String"] | Tokens["BadString"] {
			let value = "";

			if (!endChar) {
				[endChar, index] = this.getInputChar(index);
			}

			while (!this.isEOF(index)) {
				const char = this.getInputCharOnly(index);
				const nextChar = this.getInputCharOnly(index, 1);

				if (char === endChar) {
					return this.finishValueToken("String", value, ob1Inc(index));
				} else if (this.isEOF(index)) {
					this.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.UNTERMINATED_STRING,
					});
					return this.finishValueToken("String", value, ob1Inc(index));
				} else if (isNewline(char)) {
					this.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.UNTERMINATED_STRING,
					});
					return this.finishToken("BadString", index);
				} else if (char === "\\") {
					if (this.isEOF(ob1Inc(index))) {
						continue;
					}
					if (isNewline(nextChar)) {
						index = ob1Add(index, this.getNewlineLength(ob1Inc(index)));
					} else if (isValidEscape(char, nextChar)) {
						const [newValue, newIndex] = this.consumeEscaped(index);
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
		): Tokens["Percentage"] | Tokens["Dimension"] | Tokens["Number"] {
			const [newIndex, numberValue, numberType] = this.consumeNumber(index);
			index = newIndex;

			if (
				isIdentifierStart(
					this.getInputCharOnly(index),
					this.getInputCharOnly(index, 1),
					this.getInputCharOnly(index, 2),
				)
			) {
				const [unit, endIndex] = this.consumeName(index);
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

			if (this.getInputCharOnly(index) === "%") {
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

		consumeURLToken(index: Number0): Tokens["URL"] | Tokens["BadURL"] {
			let value = "";

			while (isWhitespace(this.getInputCharOnly(index))) {
				index = ob1Inc(index);
			}

			while (!this.isEOF(index)) {
				if (this.getInputCharOnly(index) === ")") {
					return this.finishValueToken("URL", value, ob1Inc(index));
				}

				if (this.isEOF(index)) {
					this.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.UNTERMINATED_URL,
					});
					return this.finishValueToken("URL", value);
				}

				if (isWhitespace(this.getInputCharOnly(index))) {
					while (isWhitespace(this.getInputCharOnly(index))) {
						index = ob1Inc(index);
					}

					if (this.getInputCharOnly(index) === ")") {
						return this.finishValueToken("URL", value, ob1Inc(index));
					}

					if (this.isEOF(index)) {
						this.unexpectedDiagnostic({
							description: descriptions.CSS_PARSER.UNTERMINATED_URL,
						});
						return this.finishValueToken("URL", value);
					}

					index = this.consumeBadURL(index);
					return this.finishToken("BadURL", index);
				}

				if (
					this.getInputCharOnly(index) === '"' ||
					this.getInputCharOnly(index) === "'" ||
					this.getInputCharOnly(index) === "("
				) {
					this.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.UNTERMINATED_URL,
					});
					index = this.consumeBadURL(index);
					return this.finishToken("BadURL", index);
				}

				if (this.getInputCharOnly(index) === "\\") {
					if (
						isValidEscape(
							this.getInputCharOnly(index),
							this.getInputCharOnly(index),
						)
					) {
						const [newValue, newIndex] = this.consumeEscaped(index);
						index = newIndex;
						value += newValue;
						continue;
					}

					this.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.UNTERMINATED_URL,
					});
					index = this.consumeBadURL(index);
					return this.finishToken("BadURL", index);
				}

				value += this.getInputCharOnly(index);
				index = ob1Inc(index);
			}

			throw new Error("Unrecoverable state due to bad URL");
		}

		tokenize(index: Number0) {
			const char = this.getInputCharOnly(index);

			// Skip over comments
			if (char === "/" && this.getInputCharOnly(index, 1) === "*") {
				index = ob1Add(index, 2);
				let value = "";
				while (
					this.getInputCharOnly(index) !== "*" &&
					this.getInputCharOnly(index, 1) !== "/" &&
					!this.isEOF(index)
				) {
					value += this.getInputCharOnly(index);
					index = ob1Add(index, 1);
				}
				this.registerComment(
					this.comments.addComment({
						type: "CommentBlock",
						value,
					}),
				);
			}

			if (isWhitespace(char)) {
				const endIndex = this.readInputFrom(index, isWhitespace)[1];
				return this.finishToken("Whitespace", endIndex);
			}

			if (char === '"') {
				return this.consumeStringToken(index);
			}

			if (char === "#") {
				const nextChar = this.getInputCharOnly(index, 1);
				if (
					isName(nextChar) ||
					isValidEscape(nextChar, this.getInputCharOnly(index, 2))
				) {
					const [value, endIndex] = this.consumeName(ob1Inc(index));
					const isIdent = isIdentifierStart(
						this.getInputCharOnly(index, 1),
						this.getInputCharOnly(index, 2),
						this.getInputCharOnly(index, 3),
					);
					return this.finishComplexToken(
						"Hash",
						{
							hashType: isIdent ? "id" : undefined,
							value,
						},
						endIndex,
					);
				}
				return this.finishValueToken("Delim", char);
			}

			if (char === "'") {
				return this.consumeStringToken(index);
			}

			if (char === "(") {
				return this.finishToken("LeftParen");
			}

			if (char === ")") {
				return this.finishToken("RightParen");
			}

			if (char === "+") {
				if (
					isNumberStart(
						char,
						this.getInputCharOnly(index, 1),
						this.getInputCharOnly(index, 2),
					)
				) {
					return this.consumeNumberToken(index);
				}

				return this.finishValueToken("Delim", char);
			}

			if (char === ",") {
				return this.finishToken("Comma");
			}

			if (char === "-") {
				if (
					isNumberStart(
						char,
						this.getInputCharOnly(index, 1),
						this.getInputCharOnly(index, 2),
					)
				) {
					return this.consumeNumberToken(index);
				}

				if (
					this.getInputCharOnly(index, 1) === "-" &&
					this.getInputCharOnly(index, 2) === ">"
				) {
					return this.finishToken("CDC", ob1Add(index, 3));
				}

				if (
					isIdentifierStart(
						char,
						this.getInputCharOnly(index, 1),
						this.getInputCharOnly(index, 2),
					)
				) {
					return this.consumeIdentLikeToken(index);
				}

				return this.finishValueToken("Delim", char);
			}

			if (char === ".") {
				if (
					isNumberStart(
						char,
						this.getInputCharOnly(index, 1),
						this.getInputCharOnly(index, 2),
					)
				) {
					return this.consumeNumberToken(index);
				}

				return this.finishValueToken("Delim", char);
			}

			if (char === ":") {
				return this.finishToken("Colon");
			}

			if (char === ";") {
				return this.finishToken("Semi");
			}

			if (char === "@") {
				if (
					isIdentifierStart(
						this.getInputCharOnly(index, 1),
						this.getInputCharOnly(index, 2),
						this.getInputCharOnly(index, 3),
					)
				) {
					const [value, endIndex] = this.consumeName(ob1Inc(index));
					return this.finishValueToken("AtKeyword", value, endIndex);
				}
				return this.finishValueToken("Delim", char);
			}

			if (char === "[") {
				return this.finishToken("LeftSquareBracket");
			}

			if (char === "\\") {
				if (isValidEscape(char, this.getInputCharOnly(index, 1))) {
					return this.consumeIdentLikeToken(index);
				}

				this.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.INVALID_ESCAPE,
				});
				return this.finishValueToken("Delim", char);
			}

			if (char === "]") {
				return this.finishToken("RightSquareBracket");
			}

			if (char === "{") {
				return this.finishToken("LeftCurlyBracket");
			}

			if (char === "}") {
				return this.finishToken("RightCurlyBracket");
			}

			if (isDigit(char)) {
				return this.consumeNumberToken(index);
			}

			if (isNameStart(char)) {
				return this.consumeIdentLikeToken(index);
			}

			return this.finishValueToken("Delim", char);
		}
	}
);
