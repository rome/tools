import {
	ParserOptionsWithRequiredPath,
	createParser,
	isDigit,
	readUntilLineBreak,
} from "@internal/parser-core";
import {ListProperties, State, Tokens} from "@internal/markdown-parser/types";
import {
	MarkdownDividerBlock,
	MarkdownHeadingBlock,
	MarkdownListBlock,
	MarkdownListItem,
	MarkdownParagraph,
	MarkdownRoot,
	MarkdownText,
} from "@internal/ast";
import {Number0, ob1Add} from "@internal/ob1";
import {isEscaped} from "@internal/string-utils";
import {
	AnyMarkdownInlineNode,
	AnyMarkdownNode,
	MarkdownListChildren,
} from "@internal/ast/markdown/unions";
import {hasThematicBreak} from "@internal/markdown-parser/utils";
import {descriptions} from "@internal/diagnostics";

export const createMarkdownParser = createParser((
	ParserCore,
	ParserWithRequiredPath,
) =>
	class MarkdownParser extends ParserWithRequiredPath<Tokens, State> {
		constructor(opts: ParserOptionsWithRequiredPath) {
			super(
				opts,
				"parse/markdown",
				{
					isBlockHead: false,
				},
			);

			this.ignoreWhitespaceTokens = false;
		}

		consumeHeading(index: Number0) {
			const [value, end] = this.readInputFrom(
				index,
				(char1) => {
					return char1 === "#";
				},
			);
			if (value.length > 6) {
				const [textValue, endText] = this.readInputFrom(end, readUntilLineBreak);
				return this.finishValueToken("Text", value + textValue, endText);
			}
			return this.finishValueToken("HeadingLevel", value.length, end);
		}

		consumeBlock(blockChar: string, index: Number0, currentChar: string) {
			const nextChar = this.getInputCharOnly(index, 1);
			const nextNextChar = this.getInputCharOnly(index, 2);
			if (hasThematicBreak([currentChar, nextChar, nextNextChar].join(""))) {
				// by spec, should be at least 3, with an infinite number
				const [value, endIndex] = this.readInputFrom(
					index,
					(char) => {
						return char === blockChar;
					},
				);
				return this.finishValueToken("Break", value, endIndex);
			}
			return undefined;
		}

		tokenizeWithState(index: Number0, state: State) {
			const char = this.getInputCharOnly(index);
			const escaped = isEscaped(index, this.input);
			if (!escaped) {
				if (char === "#") {
					return {
						token: this.consumeHeading(index),
						state,
					};
				}

				if (char === "\n") {
					const nextChar = this.getInputCharOnly(index, 1);
					if (nextChar === "#") {
						return {
							token: this.consumeHeading(ob1Add(index, 1)),
							state,
						};
					}
					if (nextChar === "\n") {
						return {
							token: this.finishToken("EndParagraph", ob1Add(index, 1)),
							state,
						};
					}

					if (isDigit(nextChar)) {
						const [, endIndex] = this.readInputFrom(index, isDigit);
						const nextChar = this.getInputCharOnly(endIndex);
						const nextNextChar = this.getInputCharOnly(endIndex, 1);

						if (nextChar === "." && nextNextChar === " ") {
							return {
								token: this.finishComplexToken<"ListItem", ListProperties>(
									"ListItem",
									{
										numeric: true,
										checked: undefined,
									},
									ob1Add(endIndex, 2),
								),
								state,
							};
						}
					}

					return {
						token: this.finishToken("NewLine"),
						state,
					};
				}

				// dividers
				if (char === "-") {
					const block = this.consumeBlock("-", index, char);
					if (block) {
						return {token: block, state};
					}
					const nextChar = this.getInputCharOnly(index, 1);
					if (nextChar === " ") {
						return {
							token: this.finishComplexToken<"ListItem", ListProperties>(
								"ListItem",
								{
									numeric: false,
									checked: undefined,
									value: "-",
								},
								ob1Add(index, 2),
							),
							state,
						};
					}
				}
				if (char === "_") {
					const block = this.consumeBlock("_", index, char);
					if (block) {
						return {token: block, state};
					}
				}
				if (char === "*") {
					const block = this.consumeBlock("*", index, char);
					if (block) {
						return {token: block, state};
					}
					const nextChar = this.getInputCharOnly(index, 1);
					if (nextChar === " ") {
						return {
							token: this.finishComplexToken<"ListItem", ListProperties>(
								"ListItem",
								{
									numeric: false,
									checked: undefined,
									value: "*",
								},
								ob1Add(index, 2),
							),
							state,
						};
					}
				}

				if (isDigit(char)) {
					const [, endIndex] = this.readInputFrom(index, isDigit);
					const nextChar = this.getInputCharOnly(endIndex);
					const nextNextChar = this.getInputCharOnly(endIndex, 1);
					if (nextChar === "." && nextNextChar === " ") {
						return {
							token: this.finishComplexToken<"ListItem", ListProperties>(
								"ListItem",
								{
									numeric: true,
									checked: undefined,
								},
								ob1Add(endIndex, 2),
							),
							state,
						};
					}
				}
			}

			const [value, endIndex] = this.readInputFrom(index, readUntilLineBreak);

			return {
				token: this.finishValueToken("Text", value, endIndex),
				state,
			};
		}

		parseHeading(): MarkdownHeadingBlock {
			const start = this.getPosition();
			const token = this.getToken();
			if (token.type === "HeadingLevel") {
				const nextToken = this.nextToken();
				if (nextToken.type === "Text") {
					this.nextToken();
					return this.finishNode(
						start,
						{
							type: "MarkdownHeadingBlock",
							level: token.value,
							value: nextToken.value.trim(),
						},
					);
				}
			}
			throw this.unexpected({
				description: descriptions.MARKDOWN_PARSER.INVALID_SEQUENCE,
			});
		}

		parseText(): MarkdownText {
			const token = this.expectToken("Text");
			const pos = this.getPosition();
			return this.finishNode(
				pos,
				{
					type: "MarkdownText",
					value: token.value,
				},
			);
		}

		parseParagraph(isList?: boolean): MarkdownParagraph {
			const start = this.getPosition();
			const children: Array<AnyMarkdownInlineNode> = [];
			while (
				!this.matchToken("EOF") &&
				!this.matchToken("EndParagraph") &&
				!this.matchToken("Break")
			) {
				const token = this.getToken();
				if (isList && token.type === "NewLine") {
					this.nextToken();
					break;
				}
				switch (token.type) {
					case "Break": {
						break;
					}
					case "Text": {
						children.push(this.parseText());
						break;
					}
					case "NewLine": {
						const pos = this.getPosition();
						children.push(
							this.finishNode(
								pos,
								{
									type: "MarkdownText",
									value: "\n",
								},
							),
						);
						this.nextToken();
						break;
					}
					default: {
						// TODO: to remove once all cases are handled
						this.unexpectedDiagnostic({
							description: descriptions.MARKDOWN_PARSER.INVALID_SEQUENCE,
						});
						this.nextToken();
					}
				}
			}

			return this.finishNode(
				start,
				{
					type: "MarkdownParagraph",
					children,
				},
			);
		}

		parseBreak(): MarkdownDividerBlock {
			const token = this.expectToken("Break");
			const start = this.getPosition();

			return this.finishNode(
				start,
				{
					type: "MarkdownDividerBlock",
					value: token.value,
				},
			);
		}

		parseItem(): MarkdownListItem {
			const token = this.expectToken("ListItem");
			const pos = this.getPosition();
			const children: Array<MarkdownListChildren> = [];

			while (
				!this.matchToken("EOF") &&
				!this.matchToken("NewLine") &&
				this.matchToken("Text")
			) {
				children.push(this.parseParagraph(true));
			}

			return this.finishNode(
				pos,
				{
					// TODO handle check
					checked: token.checked,
					type: "MarkdownListItem",
					children,
					value: token.value,
				},
			);
		}

		parseListBlock(): MarkdownListBlock {
			const pos = this.getPosition();
			const children: Array<MarkdownListItem> = [];
			const currentToken = this.getToken();
			let ordered = false;
			if (currentToken.type === "ListItem") {
				if (currentToken.numeric === true) {
					ordered = true;
				}
			}
			while (!this.matchToken("EOF") && this.matchToken("ListItem")) {
				const item = this.parseItem();
				children.push(item);
			}

			return this.finishNode(
				pos,
				{
					type: "MarkdownListBlock",
					ordered,
					children,
				},
			);
		}
		parseBlock(): undefined | AnyMarkdownNode {
			const token = this.getToken();
			switch (token.type) {
				case "NewLine": {
					this.nextToken();
					return undefined;
				}
				case "EndParagraph": {
					this.nextToken();
					return undefined;
				}
				case "HeadingLevel":
					return this.parseHeading();

				case "ListItem":
					return this.parseListBlock();

				case "Break":
					return this.parseBreak();

				case "Text":
					return this.parseParagraph();

				default:
					throw this.unexpected();
			}
		}

		parse(): MarkdownRoot {
			const start = this.getPosition();
			const body: Array<AnyMarkdownNode> = [];

			while (!this.matchToken("EOF")) {
				const child = this.parseBlock();
				if (child !== undefined) {
					body.push(child);
				}
			}

			this.finalize();

			return this.finishNode(
				start,
				this.finishRoot({
					type: "MarkdownRoot",
					body,
				}),
			);
		}
	}
);
