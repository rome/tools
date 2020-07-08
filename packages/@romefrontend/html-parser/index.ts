import {
	BaseTokens,
	ParserCoreState,
	ParserOptionsWithRequiredPath,
	SimpleToken,
	TokenValues,
	ValueToken,
	createParser,
	isAlpha,
	isDigit,
} from "@romefrontend/parser-core";
import {
	AnyHTMLChildNode,
	HTMLAttribute,
	HTMLElement,
	HTMLIdentifier,
	HTMLRoot,
	HTMLString,
	HTMLText,
} from "@romefrontend/ast";
import {Number0, ob1Add, ob1Get0, ob1Inc} from "@romefrontend/ob1";
import {isEscaped} from "@romefrontend/string-utils";
import {isSelfClosingTagName} from "./tags";
import {descriptions} from "@romefrontend/diagnostics";
import {consumeComment} from "@romefrontend/html-parser/utils.ts";

type Tokens = BaseTokens & {
	Text: ValueToken<"Text", string>;
	Slash: SimpleToken<"Slash">;
	Less: SimpleToken<"Less">;
	Equals: SimpleToken<"Equals">;
	Greater: SimpleToken<"Greater">;
	Identifier: ValueToken<"Identifier", string>;
	String: ValueToken<"String", string>;
	Comment: ValueToken<"Comment", string>;
};

type State = ParserCoreState & {
	inTagHead: boolean;
};

function isTagStartChar(index: Number0, input: string): boolean {
	const i = ob1Get0(index);
	return input[i] === "<" && !isEscaped(index, input);
}

function isStringValueChar(char: string, index: Number0, input: string): boolean {
	if (char === '"' && !isEscaped(index, input)) {
		return false;
	}

	return true;
}

function isIdentifierChar(char: string): boolean {
	return isDigit(char) || isAlpha(char) || char === "-";
}

function isTextChar(char: string, index: Number0, input: string): boolean {
	return !isTagStartChar(index, input);
}

function isTagComment(index: Number0, input: string): boolean {
	const first = input[ob1Get0(index)];
	const second = input[ob1Get0(index) + 1];
	const third = input[ob1Get0(index) + 2];
	const fourth = input[ob1Get0(index) + 3];
	return first === "<" && second === "!" && third === "-" && fourth === "-";
}

const createHTMLParser = createParser((ParserCore, ParserWithRequiredPath) =>
	class HTMLParser extends ParserWithRequiredPath<Tokens, State> {
		constructor(opts: ParserOptionsWithRequiredPath) {
			super(
				opts,
				"parse/html",
				{
					inTagHead: false,
				},
			);
			this.ignoreWhitespaceTokens = true;
		}

		tokenizeWithState(
			index: Number0,
			state: State,
		):
			| undefined
			| {
					token: TokenValues<Tokens>;
					state: State;
				} {
			const escaped = isEscaped(index, this.input);
			const char = this.getInputCharOnly(index);

			if (!escaped && state.inTagHead) {
				if (char === " ") {
					return this.lookahead(ob1Inc(index));
				}

				if (char === "=") {
					return {
						state,
						token: this.finishToken("Equals"),
					};
				}

				if (char === "/") {
					return {
						state,
						token: this.finishToken("Slash"),
					};
				}

				if (isIdentifierChar(char)) {
					const [value, end] = this.readInputFrom(index, isIdentifierChar);
					return {
						state,
						token: this.finishValueToken("Identifier", value, end),
					};
				}

				if (char === '"') {
					const [value, stringValueEnd, unclosed] = this.readInputFrom(
						ob1Inc(index),
						isStringValueChar,
					);

					if (unclosed) {
						// TODO
					}

					const end = ob1Add(stringValueEnd, 1);
					return {
						state,
						token: this.finishValueToken("String", value, end),
					};
				}

				if (char === ">") {
					return {
						state: {
							...state,
							inTagHead: false,
						},
						token: this.finishToken("Greater"),
					};
				}
			}

			if (isTagComment(index, this.input)) {
				const startingIndex = ob1Add(index, 2);
				const [endIndex, value] = consumeComment(startingIndex, this.input);
				return {
					state: {
						...state,
						inTagHead: false,
					},
					token: this.finishValueToken("Comment", value, endIndex),
				};
			}

			if (isTagStartChar(index, this.input)) {
				return {
					state: {
						...state,
						inTagHead: true,
					},
					token: this.finishToken("Less"),
				};
			}

			// Keep eating text until we hit a <
			const [value, end] = this.readInputFrom(index, isTextChar);
			return {
				state,
				token: {
					type: "Text",
					value,
					start: index,
					end,
				},
			};
		}

		parseIdentifier(): HTMLIdentifier {
			const start = this.getPosition();
			const token = this.expectToken("Identifier");
			return this.finishNode(
				start,
				{
					type: "HTMLIdentifier",
					name: token.value,
				},
			);
		}

		parseString(): HTMLString {
			const start = this.getPosition();
			const value = this.expectToken("String").value;
			return this.finishNode(
				start,
				{
					type: "HTMLString",
					value,
				},
			);
		}

		parseAttribute(): HTMLAttribute {
			const start = this.getPosition();
			const name = this.parseIdentifier();
			this.expectToken("Equals");
			const value = this.parseString();
			return this.finishNode(
				start,
				{
					type: "HTMLAttribute",
					name,
					value,
				},
			);
		}

		atTagEnd(): boolean {
			return this.matchToken("Less") && this.lookahead().token.type === "Slash";
		}

		atComment(): boolean {
			return this.lookahead().token.type === "Comment";
		}

		parseTag(): HTMLElement {
			const headStart = this.getPosition();
			if (this.atComment()) {
				this.parseComment();
			}
			this.expectToken("Less");

			const attributes: HTMLElement["attributes"] = [];
			const children: HTMLElement["children"] = [];

			const name = this.parseIdentifier();
			const tagName = name.name;
			let selfClosing = isSelfClosingTagName(tagName);

			// Parse attributes
			while (!this.matchToken("EOF") && !this.matchToken("Greater")) {
				const keyToken = this.getToken();

				if (keyToken.type === "Identifier") {
					attributes.push(this.parseAttribute());
				} else if (keyToken.type === "Slash") {
					this.nextToken();
					selfClosing = true;
				} else {
					throw this.unexpected({
						description: descriptions.HTML_PARSER.EXPECTED_ATTRIBUTE_NAME,
					});
				}
			}

			this.expectToken("Greater");
			const headEnd = this.getPosition();

			// Verify closing tag
			if (!selfClosing) {
				while (
					// Build children
					!this.matchToken("EOF") &&
					!this.atTagEnd()
				) {
					const child = this.parseChild();
					if (child !== undefined) {
						children.push(child);
					}
				}

				if (this.matchToken("EOF")) {
					throw this.unexpected({
						description: descriptions.HTML_PARSER.UNCLOSED_TAG(
							tagName,
							this.finishLocAt(headStart, headEnd),
						),
					});
				} else {
					this.expectToken("Less");
					this.expectToken("Slash");

					const name = this.getToken();
					if (name.type === "Identifier") {
						if (name.value !== tagName) {
							throw this.unexpected({
								description: descriptions.HTML_PARSER.INCORRECT_CLOSING_TAG_NAME(
									tagName,
									name.value,
								),
							});
						}

						this.nextToken();
					} else {
						throw this.unexpected({
							description: descriptions.HTML_PARSER.EXPECTED_CLOSING_TAG_NAME,
						});
					}

					this.expectToken("Greater");
				}
			}

			return this.finishNode(
				headStart,
				{
					type: "HTMLElement",
					selfClosing,
					name,
					attributes,
					children,
				},
			);
		}

		parseComment(): undefined {
			const start = this.getPosition();
			const token = this.expectToken("Comment");

			this.comments.addComment({
				value: token.value,
				type: "CommentBlock",
				loc: this.finishLoc(start),
			});
			return undefined;
		}

		parseText(): HTMLText {
			const start = this.getPosition();
			const token = this.expectToken("Text");

			const lines: Array<string> = [];
			let line = "";

			function pushLine() {
				line = line.trim();

				if (line !== "") {
					lines.push(line);
					line = "";
				}
			}

			let lineStart = true;

			for (const char of token.value) {
				switch (char) {
					case "\n": {
						lineStart = true;
						break;
					}

					case "\t":
					case " ": {
						if (!lineStart) {
							line += " ";
						}
						break;
					}

					default: {
						lineStart = false;
						line += char;
						break;
					}
				}
			}

			pushLine();

			const value = lines.join(" ").replace(/\s+/g, " ");

			return this.finishNode(
				start,
				{
					type: "HTMLText",
					value,
				},
			);
		}

		parseChild(): undefined | AnyHTMLChildNode {
			const token = this.getToken();

			switch (token.type) {
				case "Less":
					return this.parseTag();

				case "Text":
					return this.parseText();

				case "Comment":
					return this.parseComment();

				default:
					throw this.unexpected();
			}
		}

		parse(): HTMLRoot {
			const start = this.getPosition();
			const body: Array<AnyHTMLChildNode> = [];

			while (!this.matchToken("EOF")) {
				const child = this.parseChild();
				if (child !== undefined) {
					body.push(child);
				}
			}

			this.finalize();

			return this.finishNode(
				start,
				this.finishRoot({
					type: "HTMLRoot",
					body,
				}),
			);
		}
	}
);

export function parseHTML(opts: ParserOptionsWithRequiredPath) {
	return createHTMLParser(opts).parse();
}

export function tokenizeHTML(opts: ParserOptionsWithRequiredPath) {
	return createHTMLParser(opts).tokenizeAll();
}

export * from "./xhtmlEntities";
