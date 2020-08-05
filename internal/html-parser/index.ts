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
} from "@internal/parser-core";
import {
	AnyHTMLChildNode,
	HTMLAttribute,
	HTMLElement,
	HTMLIdentifier,
	HTMLRoot,
	HTMLString,
	HTMLText,
} from "@internal/ast";
import {Number0, ob1Add, ob1Get0, ob1Inc} from "@internal/ob1";
import {isEscaped} from "@internal/string-utils";
import {isSelfClosingTagName} from "./tags";
import {descriptions} from "@internal/diagnostics";

type Tokens = BaseTokens & {
	Text: ValueToken<"Text", string>;
	// <
	TagStartOpen: SimpleToken<"TagStartOpen">;
	// />
	TagSelfClosing: SimpleToken<"TagSelfClosing">;
	// >
	TagEnd: SimpleToken<"TagEnd">;
	// </
	TagEndOpen: SimpleToken<"TagEndOpen">;
	Equals: SimpleToken<"Equals">;
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

function isntCommentEnd(char: string, index: Number0, input: string): boolean {
	const isCommentEnd =
		char === "-" &&
		!isEscaped(index, input) &&
		input[ob1Get0(index) + 1] === "-" &&
		input[ob1Get0(index) + 2] === ">";
	return !isCommentEnd;
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

		protected tokenizeWithState(
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
				if (char === "=") {
					return {
						state,
						token: this.finishToken("Equals"),
					};
				}

				if (char === "/" && this.getInputCharOnly(index, 1)) {
					return {
						state,
						token: this.finishToken("TagSelfClosing", ob1Add(index, 2)),
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
						token: this.finishToken("TagEnd"),
					};
				}
			}

			if (
				this.getInputCharOnly(index) === "<" &&
				this.getInputCharOnly(index, 1) === "!" &&
				this.getInputCharOnly(index, 2) === "-" &&
				this.getInputCharOnly(index, 3) === "-"
			) {
				// Skip <!--
				const start = ob1Add(index, 4);
				const [value, valueEnd, overflow] = this.readInputFrom(
					start,
					isntCommentEnd,
				);

				// Check for unclosed comment
				if (overflow) {
					// TODO
				}

				// Skip -->
				const end = ob1Add(valueEnd, 3);

				return {
					state: {
						...state,
						inTagHead: false,
					},
					token: this.finishValueToken("Comment", value, end),
				};
			}

			if (isTagStartChar(index, this.input)) {
				let token;

				if (this.getInputCharOnly(index, 1) === "/") {
					token = this.finishToken("TagEndOpen", ob1Add(index, 2));
				} else {
					token = this.finishToken("TagStartOpen");
				}

				return {
					state: {
						...state,
						inTagHead: true,
					},
					token,
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

		private parseIdentifier(): HTMLIdentifier {
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

		private parseString(): HTMLString {
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

		private parseAttribute(): HTMLAttribute {
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

		private parseTag(): HTMLElement {
			const headStart = this.getPosition();
			this.expectToken("TagStartOpen");

			const attributes: HTMLElement["attributes"] = [];
			const children: HTMLElement["children"] = [];

			const name = this.parseIdentifier();
			const tagName = name.name;
			let selfClosing = isSelfClosingTagName(tagName);

			// Parse attributes
			while (
				!this.matchToken("EOF") &&
				!this.matchToken("TagSelfClosing") &&
				!this.matchToken("TagEnd")
			) {
				const keyToken = this.getToken();

				if (keyToken.type === "Identifier") {
					attributes.push(this.parseAttribute());
				} else {
					throw this.unexpected({
						description: descriptions.HTML_PARSER.EXPECTED_ATTRIBUTE_NAME,
					});
				}
			}

			if (this.eatToken("TagSelfClosing")) {
				selfClosing = true;
			} else {
				this.expectToken("TagEnd");
			}

			const headEnd = this.getPosition();

			// Verify closing tag
			if (!selfClosing) {
				while (
					// Build children
					!this.matchToken("EOF") &&
					!this.matchToken("TagEndOpen")
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
					this.expectToken("TagEndOpen");

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

					this.expectToken("TagEnd");
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

		private parseComment(): undefined {
			const start = this.getPosition();
			const token = this.expectToken("Comment");

			this.registerComment(
				this.comments.createComment({
					value: token.value,
					type: "CommentBlock",
					loc: this.finishLoc(start),
				}),
			);
			return undefined;
		}

		private parseText(): HTMLText {
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

		private parseChild(): undefined | AnyHTMLChildNode {
			const token = this.getToken();

			switch (token.type) {
				case "TagStartOpen":
					return this.parseTag();

				case "Text":
					return this.parseText();

				case "Comment":
					return this.parseComment();

				case "TagEndOpen": {
					this.unexpectedDiagnostic({
						description: descriptions.HTML_PARSER.UNOPENED_TAG,
					});
					this.nextToken();
					return undefined;
				}

				default: {
					this.unexpectedDiagnostic();
					this.nextToken();
					return undefined;
				}
			}
		}

		public parse(): HTMLRoot {
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
