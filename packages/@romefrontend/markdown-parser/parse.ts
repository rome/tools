import {
	ParserOptionsWithRequiredPath,
	createParser,
	isDigit,
	readUntilLineBreak,
} from "@romefrontend/parser-core";
import {Tokens} from "@romefrontend/markdown-parser/types";
import {
	MarkdownDividerBlock,
	MarkdownHeadingBlock,
	MarkdownListBlock,
	MarkdownListItem,
	MarkdownParagraph,
	MarkdownRoot,
	MarkdownText,
} from "@romefrontend/ast";
import {Number0, ob1Add} from "@romefrontend/ob1";
import {isEscaped} from "@romefrontend/string-utils";
import {
	AnyMarkdownInlineNode,
	AnyMarkdownNode,
	MarkdownListChildren,
} from "@romefrontend/ast/markdown/unions";
import {hasThematicBreak} from "@romefrontend/markdown-parser/utils";

export const createMarkdownParser = createParser((
	ParserCore,
	ParserWithRequiredPath,
) =>
	class MarkdownParser extends ParserWithRequiredPath<Tokens> {
		constructor(opts: ParserOptionsWithRequiredPath) {
			super(opts, "parse/markdown", {});

			this.ignoreWhitespaceTokens = false;
		}

		consumeBlock(blockChar: string, index: Number0, currentChar: string) {
			const nextChar = this.getInputCharOnly(index, 1);
			const nextNextChar = this.getInputCharOnly(index, 2);
			if (hasThematicBreak([currentChar, nextChar, nextNextChar].join(""))) {
				// by spec, should be at least 3, with an infinite number
				const [, endIndex] = this.readInputFrom(
					index,
					(char) => {
						return char === blockChar;
					},
				);
				return this.finishToken("Break", endIndex);
			}
			return undefined;
		}

		tokenize(index: Number0) {
			const char = this.getInputCharOnly(index);
			const escaped = isEscaped(index, this.input);
			if (!escaped) {
				if (char === "#") {
					return this.finishToken("Hash");
				}

				if (char === "\n") {
					return this.finishToken("NewLine");
				}

				// dividers
				if (char === "-") {
					const block = this.consumeBlock("-", index, char);
					if (block) {
						return block;
					}
				}
				if (char === "_") {
					const block = this.consumeBlock("_", index, char);
					if (block) {
						return block;
					}
				}
				if (char === "*") {
					const block = this.consumeBlock("*", index, char);
					if (block) {
						return block;
					}
				}

				if (isDigit(char)) {
					const nextChar = this.getInputCharOnly(index, 1);
					const nextNextChar = this.getInputCharOnly(index, 2);
					if (nextChar === "." && nextNextChar === " ") {
						return this.finishValueToken(
							"ListItem",
							"numeric-list",
							ob1Add(index, 3),
						);
					}
				}
			}

			const [value, endIndex] = this.readInputFrom(index, readUntilLineBreak);

			return this.finishValueToken("Text", value, endIndex);
		}

		parseHeading(): MarkdownHeadingBlock {
			const start = this.getPosition();
			let level = 1;
			this.expectToken("Hash");
			while (this.matchToken("Hash") && level < 6) {
				level += 1;
				this.nextToken();
			}

			// TODO what if we have more than 6 hashes? Error or what?
			const token = this.expectToken("Text");
			return this.finishNode(
				start,
				{
					type: "MarkdownHeadingBlock",
					level,
					value: token.value.trim(),
				},
			);
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

		parseParagraph(): MarkdownParagraph {
			const start = this.getPosition();
			const children: Array<AnyMarkdownInlineNode> = [];

			while (!this.matchToken("EOF") && !this.matchToken("NewLine")) {
				const token = this.getToken();
				switch (token.type) {
					case "Text": {
						children.push(this.parseText());
						break;
					}
				}
				this.nextToken();
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
			this.expectToken("Break");
			const start = this.getPosition();

			return this.finishNode(
				start,
				{
					type: "MarkdownDividerBlock",
				},
			);
		}

		parseItem(): MarkdownListItem {
			this.expectToken("ListItem");
			const pos = this.getPosition();
			const children: Array<MarkdownListChildren> = [];

			while (!this.matchToken("EOF") && this.matchToken("Text")) {
				children.push(this.parseParagraph());
			}

			return this.finishNode(
				pos,
				{
					// TODO handle check
					checked: null,
					type: "MarkdownListItem",
					children,
				},
			);
		}

		parseListBlock(): MarkdownListBlock {
			const pos = this.getPosition();
			const children: Array<MarkdownListItem> = [];

			while (!this.matchToken("EOF") && this.matchToken("ListItem")) {
				const item = this.parseItem();
				children.push(item);
			}

			return this.finishNode(
				pos,
				{
					type: "MarkdownListBlock",

					// TODO to review the type of list
					kind: "dot-list",
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
				case "Hash":
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
