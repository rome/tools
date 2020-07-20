import {AnyCSSToken, AnyCSSValue, CSSParserOptions, Tokens} from "./types";
import {
	ParserOptions,
	ValueToken,
	createParser,
	isDigit,
	isHexDigit,
} from "@romefrontend/parser-core";
import {DiagnosticCategory, descriptions} from "@romefrontend/diagnostics";
import {Number0, ob1Add, ob1Inc} from "@romefrontend/ob1";
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
	CSSRule,
	CSSStylesheet,
} from "@romefrontend/ast";

export const createCSSParser = createParser((_, ParserWithRequiredPath) =>
	class CSSParser extends ParserWithRequiredPath<Tokens> {
		consumeDiagnosticCategory: DiagnosticCategory;
		options: ParserOptions;

		constructor(opts: CSSParserOptions) {
			super(opts, "parse/css", {});

			this.consumeDiagnosticCategory =
				opts.consumeDiagnosticCategory || "parse/css";
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
				return this.finishValueToken(
					"Percentage",
					numberValue,
					ob1Add(index, 1),
				);
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

		tokenize(index: Number0): AnyCSSToken {
			const char = this.getInputCharOnly(index);

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
				return this.finishValueToken("Comment", value, ob1Add(index, 2));
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

			if (char === "<") {
				if (
					this.getInputCharOnly(index, 1) === "!" &&
					this.getInputCharOnly(index, 2) === "-" &&
					this.getInputCharOnly(index, 3) === "-"
				) {
					return this.finishToken("CDO", ob1Add(index, 4));
				}
				return this.finishValueToken("Delim", char);
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

		parse(): CSSStylesheet {
			const start = this.getPosition();
			const rules = this.parseRules(true);

			this.finalize();

			return this.finishNode(
				start,
				this.finishRoot({
					type: "CSSStylesheet",
					body: rules,
				}),
			);
		}

		parseRules(
			topLevel = false,
			endingTokenType?: keyof Tokens,
		): Array<CSSAtRule | CSSRule> {
			const rules: Array<CSSAtRule | CSSRule> = [];
			while (!this.matchToken("EOF")) {
				if (endingTokenType && this.matchToken(endingTokenType)) {
					this.nextToken();
					break;
				}
				if (this.matchToken("Comment")) {
					this.registerComment(
						this.comments.createComment({
							type: "CommentBlock",
							loc: this.finishLoc(this.getPosition()),
							value: (this.getToken() as Tokens["Comment"]).value,
						}),
					);
					this.eatToken("Comment");
					continue;
				}
				if (this.matchToken("Whitespace")) {
					this.eatToken("Whitespace");
					continue;
				}
				if (this.matchToken("CDO") || this.matchToken("CDC")) {
					if (topLevel) {
						this.nextToken();
						continue;
					}
					const rule = this.parseRule();
					rule && rules.push(rule);
					continue;
				}
				if (this.matchToken("AtKeyword")) {
					rules.push(this.parseAtRule());
					continue;
				}
				const rule = this.parseRule();
				rule && rules.push(rule);
			}

			return rules;
		}

		parseRule(): CSSRule | null {
			const start = this.getPosition();
			const prelude: Array<AnyCSSValue> = [];
			while (!this.matchToken("EOF")) {
				if (this.matchToken("LeftCurlyBracket")) {
					return this.finishNode(
						start,
						{
							type: "CSSRule",
							// TODO: Parse prelude according to selector grammar
							// https://www.w3.org/TR/css-syntax-3/#style-rules
							prelude,
							block: this.parseDeclartionBlock(),
						},
					);
				}
				const parsedValue = this.parseComponentValue();
				parsedValue && prelude.push(parsedValue);
			}
			this.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNEXPECTED_TOKEN,
			});
			return null;
		}

		parseAtRule(): CSSAtRule {
			const start = this.getPosition();
			const token = this.expectToken("AtKeyword");
			const prelude: Array<AnyCSSValue> = [];
			const name = token.value;
			let block = null;
			while (true) {
				if (this.matchToken("Semi")) {
					break;
				}
				if (this.matchToken("EOF")) {
					this.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.UNTERMINATED_AT_RULE,
					});
					break;
				}
				if (this.matchToken("LeftCurlyBracket")) {
					block = this.parseComplexBlock();
					break;
				}
				const parsedValue = this.parseComponentValue();
				parsedValue && prelude.push(parsedValue);
			}
			return this.finishNode(
				start,
				{
					type: "CSSAtRule",
					name,
					prelude,
					block,
				},
			);
		}

		parseSimpleBlock(): CSSBlock | null {
			const start = this.getPosition();
			const startingToken = this.getToken();
			const startingTokenValue = this.getBlockStartTokenValue(startingToken);
			const endingTokenType = this.getBlockEndTokenType(startingToken);
			let value: Array<AnyCSSValue | CSSAtRule | CSSDeclaration | null> = [];

			if (!endingTokenType) {
				return null;
			}

			this.nextToken();

			while (true) {
				if (this.matchToken(endingTokenType)) {
					this.nextToken();
					break;
				}
				if (this.matchToken("EOF")) {
					this.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.UNTERMINATED_BLOCK,
					});
					break;
				}
				const parsedValue = this.parseComponentValue();
				parsedValue && value.push(parsedValue);
			}

			return this.finishNode(
				start,
				{
					type: "CSSBlock",
					startingTokenValue,
					value,
				},
			);
		}

		parseDeclartionBlock(): CSSBlock | null {
			const start = this.getPosition();
			const startingToken = this.getToken();
			const startingTokenValue = this.getBlockStartTokenValue(startingToken);
			const endingTokenType = this.getBlockEndTokenType(startingToken);
			let value: Array<AnyCSSValue | CSSAtRule | CSSDeclaration | null> = [];

			if (!endingTokenType) {
				return null;
			}

			this.nextToken();

			value = this.parseDeclarations(endingTokenType);

			return this.finishNode(
				start,
				{
					type: "CSSBlock",
					startingTokenValue,
					value,
				},
			);
		}

		parseComplexBlock(): CSSBlock | null {
			const start = this.getPosition();
			const startingToken = this.getToken();
			const startingTokenValue = this.getBlockStartTokenValue(startingToken);
			const endingTokenType = this.getBlockEndTokenType(startingToken);
			let value: Array<CSSAtRule | CSSRule> = [];

			if (!endingTokenType) {
				return null;
			}

			this.nextToken();

			value = this.parseRules(false, endingTokenType);

			return this.finishNode(
				start,
				{
					type: "CSSBlock",
					startingTokenValue,
					value,
				},
			);
		}

		parseComponentValue(): AnyCSSValue | null {
			if (this.matchToken("Whitespace")) {
				this.nextToken();
				return null;
			}
			if (
				this.matchToken("LeftCurlyBracket") ||
				this.matchToken("LeftParen") ||
				this.matchToken("LeftSquareBracket")
			) {
				return this.parseSimpleBlock();
			}
			if (this.matchToken("Function")) {
				return this.parseFunction();
			}

			const start = this.getPosition();

			if (this.matchToken("Dimension")) {
				const unit = (this.getToken() as Tokens["Dimension"]).unit;
				const value = (this.getToken() as Tokens["Dimension"]).value;
				this.nextToken();
				return this.finishNode(
					start,
					{
						type: "CSSDimension",
						unit,
						value,
					},
				);
			}
			if (this.matchToken("Percentage")) {
				const value = (this.getToken() as Tokens["Percentage"]).value;
				this.nextToken();
				return this.finishNode(
					start,
					{
						type: "CSSPercentage",
						value,
					},
				);
			}
			if (this.matchToken("Ident")) {
				const value = (this.getToken() as Tokens["Ident"]).value;
				this.nextToken();
				return this.finishNode(
					start,
					{
						type: "CSSIdent",
						value,
					},
				);
			}
			if (this.matchToken("Number")) {
				const value = (this.getToken() as Tokens["Number"]).value;
				this.nextToken();
				return this.finishNode(
					start,
					{
						type: "CSSNumber",
						value,
					},
				);
			}
			if (this.matchToken("Colon")) {
				this.nextToken();
				return this.finishNode(
					start,
					{
						type: "CSSRaw",
						value: ":",
					},
				);
			}
			const value = (this.getToken() as ValueToken<string, string>).value;
			this.nextToken();
			return this.finishNode(
				start,
				{
					type: "CSSRaw",
					value,
				},
			);
		}

		parseFunction(): CSSFunction {
			const start = this.getPosition();
			const token = this.expectToken("Function");
			const name = token.value;
			const value = [];

			while (true) {
				if (this.matchToken("RightParen")) {
					this.nextToken();
					break;
				}
				if (this.matchToken("EOF")) {
					this.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.UNTERMINATED_FUNCTION,
					});
					break;
				}
				const parsedValue = this.parseComponentValue();
				parsedValue && value.push(parsedValue);
			}

			return this.finishNode(
				start,
				{
					type: "CSSFunction",
					name,
					value,
				},
			);
		}

		parseDeclarations(
			endingTokenType?: keyof Tokens,
		): Array<CSSAtRule | CSSDeclaration> {
			const declarations: Array<CSSAtRule | CSSDeclaration> = [];

			while (!this.matchToken("EOF")) {
				if (this.eatToken("Whitespace") || this.eatToken("Semi")) {
					continue;
				}
				if (endingTokenType && this.matchToken(endingTokenType)) {
					this.nextToken();
					break;
				}
				if (this.matchToken("AtKeyword")) {
					declarations.push(this.parseAtRule());
					continue;
				}
				if (this.matchToken("Ident")) {
					const declaration = this.parseDeclaration();
					declaration && declarations.push(declaration);
					while (!this.matchToken("Semi") && !this.matchToken("EOF")) {
						const declaration = this.parseDeclaration();
						declaration && declarations.push(declaration);
					}
					continue;
				}
				this.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.INVALID_DECLARATION,
				});
				while (!this.matchToken("Semi") && !this.matchToken("EOF")) {
					this.parseComponentValue();
				}
			}

			return declarations;
		}

		parseDeclaration(): CSSDeclaration | null {
			while (!this.matchToken("Semi")) {
				const currentToken = this.getToken();
				if (currentToken.type !== "Ident") {
					this.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.INVALID_DECLARATION,
					});
					return null;
				}

				const name = currentToken.value;
				const start = this.getPosition();
				let important = false;
				let value: Array<AnyCSSValue | null> = [];
				this.nextToken();

				while (this.matchToken("Whitespace")) {
					this.eatToken("Whitespace");
				}
				if (!this.matchToken("Colon")) {
					this.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.INVALID_DECLARATION,
					});
					return null;
				}
				this.nextToken();
				while (this.matchToken("Whitespace")) {
					this.eatToken("Whitespace");
				}
				while (!this.matchToken("EOF")) {
					if (this.matchToken("Semi")) {
						const lastTwoTokens = [...value].slice(-2);
						if (
							lastTwoTokens[0]?.type === "CSSRaw" &&
							lastTwoTokens[0].value === "!" &&
							lastTwoTokens[1]?.type === "CSSIdent" &&
							/^important$/i.test(lastTwoTokens[1].value)
						) {
							value = value.slice(0, -2);
							important = true;
						}
						return this.finishNode(
							start,
							{
								type: "CSSDeclaration",
								important,
								name,
								value,
							},
						);
					}
					const parsedValue = this.parseComponentValue();
					parsedValue && value.push(parsedValue);
				}
			}
			return null;
		}

		getBlockStartTokenValue(token: AnyCSSToken): string | null {
			switch (token.type) {
				case "LeftCurlyBracket":
					return "{";
				case "LeftParen":
					return "(";
				case "LeftSquareBracket":
					return "[";
				default: {
					this.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.INVALID_BLOCK_START,
					});
					return null;
				}
			}
		}

		getBlockEndTokenType(token: AnyCSSToken): keyof Tokens | null {
			switch (token.type) {
				case "LeftCurlyBracket":
					return "RightCurlyBracket";
				case "LeftParen":
					return "RightParen";
				case "LeftSquareBracket":
					return "RightSquareBracket";
				default: {
					this.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.INVALID_BLOCK_START,
					});
					return null;
				}
			}
		}
	}
);
