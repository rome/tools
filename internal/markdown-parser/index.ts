import {
	ParserCore,
	ParserOptions,
	createParser,
	isDigit,
	isntLineBreak,
} from "@internal/parser-core";
import {
	AnyMarkdownNode,
	MarkdownCodeBlock,
	MarkdownDividerBlock,
	MarkdownHeadingBlock,
	MarkdownListBlock,
	MarkdownListItem,
	MarkdownRoot,
} from "@internal/ast";
import {isEscaped} from "@internal/string-utils";
import {CodeProperties, MarkdownParserTypes} from "./types";
import {hasThematicBreak, isntInlineCharacter} from "./utils";
import {descriptions} from "@internal/diagnostics";
import {createMarkdownInitialState} from "@internal/markdown-parser/State";
import {
	parseListItem,
	tokenizeListItem,
} from "@internal/markdown-parser/parser/listItem";
import {tokenizeInline} from "@internal/markdown-parser/parser/inline";
import {parseParagraph} from "@internal/markdown-parser/parser/paragraph";
import {parseReference} from "@internal/markdown-parser/parser/reference";

export type MarkdownParser = ParserCore<MarkdownParserTypes>;

const markdownParser = createParser<MarkdownParserTypes>({
	diagnosticLanguage: "markdown",
	ignoreWhitespaceTokens: false,
	getInitialState: () => createMarkdownInitialState(),

	tokenizeWithState(parser, tokenizer, state) {
		const char = tokenizer.get();
		const escaped = isEscaped(tokenizer.index, parser.input);

		if (!escaped) {
			if (char === "\n") {
				return [
					{
						isParagraph: false,
					},
					tokenizer.finishToken("NewLine"),
				];
			}

			if (tokenizer.consume("[")) {
				return tokenizer.finishToken("OpenSquareBracket");
			}

			if (tokenizer.consume("]")) {
				return tokenizer.finishToken("CloseSquareBracket");
			}

			if (tokenizer.consume("(")) {
				return tokenizer.finishToken("OpenBracket");
			}

			if (tokenizer.consume(")")) {
				return tokenizer.finishToken("CloseBracket");
			}
		}

		if (!(escaped || state.isParagraph)) {
			if (char === "#") {
				return consumeHeading(parser, tokenizer);
			}

			if (char === "-") {
				const block = tokenizeBlock(parser, "-", tokenizer, char);
				if (block) {
					return block;
				}

				const listItemToken = tokenizeListItem(parser, tokenizer, "-");
				if (listItemToken) {
					return [
						{
							isParagraph: true,
						},
						listItemToken,
					];
				}
			}

			if (char === "_") {
				const block = tokenizeBlock(parser, "_", tokenizer, char);
				if (block) {
					return block;
				}

				const result = tokenizeInline(parser, state, "_", tokenizer);
				if (result) {
					return result;
				}
			}

			if (char === "*") {
				const block = tokenizeBlock(parser, "*", tokenizer, char);
				if (block) {
					return block;
				}

				const listItemToken = tokenizeListItem(parser, tokenizer, "*");
				if (listItemToken) {
					return [
						{
							isParagraph: true,
						},
						listItemToken,
					];
				}

				const result = tokenizeInline(parser, state, "*", tokenizer);
				if (result) {
					return result;
				}
			}

			if (char === "`") {
				const token = consumeCode(parser, tokenizer);
				if (token) {
					return token;
				}
			}

			if (isDigit(char)) {
				const listItemToken = tokenizeListItem(parser, tokenizer);

				if (listItemToken) {
					return [
						{
							isParagraph: true,
						},
						listItemToken,
					];
				}
			}
		}

		if (!escaped && state.isParagraph) {
			if (char === "*") {
				const result = tokenizeInline(parser, state, "*", tokenizer);
				if (result) {
					return result;
				}
			}

			if (char === "_") {
				const result = tokenizeInline(parser, state, "_", tokenizer);
				if (result) {
					return result;
				}
			}

			if (char === "\n") {
				return [
					{
						isParagraph: false,
					},
					tokenizer.finishToken("NewLine"),
				];
			}

			const value = tokenizer.read(isntInlineCharacter);

			return [
				{
					isParagraph: tokenizer.get() !== "\n",
				},
				tokenizer.finishValueToken("Text", value),
			];
		}

		const value = tokenizer.read(isntInlineCharacter);

		return [
			{
				isParagraph: tokenizer.get() !== "\n",
			},
			tokenizer.finishValueToken("Text", value),
		];
	},
});

function isHash(char: string): boolean {
	return char === "#";
}

function consumeHeading(
	parser: MarkdownParser,
	tokenizer: MarkdownParser["tokenizer"],
) {
	const value = tokenizer.read(isHash);
	if (value.length > 6) {
		const textValue = tokenizer.read(isntLineBreak);
		return tokenizer.finishValueToken("Text", value + textValue);
	}
	return tokenizer.finishValueToken("HeadingLevel", value.length);
}

function tokenizeBlock(
	parser: MarkdownParser,
	blockChar: string,
	tokenizer: MarkdownParser["tokenizer"],
	currentChar: string,
) {
	const nextChar = tokenizer.get(1);
	const nextNextChar = tokenizer.get(2);
	if (hasThematicBreak([currentChar, nextChar, nextNextChar].join(""))) {
		// by spec, should be at least 3, with an infinite number
		const value = tokenizer.read((char) => {
			return char === blockChar;
		});
		return tokenizer.finishValueToken("Break", value);
	}
	return undefined;
}

function parseHeading(parser: MarkdownParser): MarkdownHeadingBlock {
	const start = parser.getPosition();
	const token = parser.getToken();
	if (token.type === "HeadingLevel") {
		const nextToken = parser.nextToken();
		if (nextToken.type === "Text") {
			parser.nextToken();
			return parser.finishNode(
				start,
				{
					type: "MarkdownHeadingBlock",
					level: token.value,
					value: nextToken.value.trim(),
				},
			);
		}
	}
	throw parser.unexpected({
		description: descriptions.MARKDOWN_PARSER.INVALID_SEQUENCE,
	});
}

function parseBreak(parser: MarkdownParser): MarkdownDividerBlock {
	const token = parser.expectToken("Break");
	const start = parser.getPosition();

	return parser.finishNode(
		start,
		{
			type: "MarkdownDividerBlock",
			value: token.value,
		},
	);
}

function parseListBlock(parser: MarkdownParser): MarkdownListBlock {
	const pos = parser.getPosition();
	const children: MarkdownListItem[] = [];
	const currentToken = parser.getToken();
	let ordered = false;
	if (currentToken.type === "ListItem") {
		if (currentToken.numeric) {
			ordered = true;
		}
	}
	while (!parser.matchToken("EOF") && parser.matchToken("ListItem")) {
		const item = parseListItem(parser);
		children.push(item);
	}

	return parser.finishNode(
		pos,
		{
			type: "MarkdownListBlock",
			ordered,
			children,
		},
	);
}

function parseCode(parser: MarkdownParser): MarkdownCodeBlock {
	const token = parser.expectToken("Code");
	const start = parser.getPosition();

	return parser.finishNode(
		start,
		{
			type: "MarkdownCodeBlock",
			language: token.language,
			value: token.value,
		},
	);
}

function parseBlock(
	parser: MarkdownParser,
): undefined | AnyMarkdownNode | (AnyMarkdownNode[]) {
	const token = parser.getToken();
	switch (token.type) {
		case "NewLine": {
			parser.nextToken();
			return undefined;
		}

		case "OpenSquareBracket": {
			return parseReference(parser);
		}

		case "HeadingLevel":
			return parseHeading(parser);

		case "ListItem":
			return parseListBlock(parser);

		case "Break":
			return parseBreak(parser);

		case "Strong":
		case "Emphasis":
		case "Text":
			return parseParagraph(parser);

		case "Code":
			return parseCode(parser);

		default: {
			throw parser.unexpected();
		}
	}
}

export function parseMarkdown(opts: ParserOptions): MarkdownRoot {
	const parser = markdownParser.create(opts);
	const start = parser.getPosition();
	const body: AnyMarkdownNode[] = [];

	while (!parser.matchToken("EOF")) {
		const child = parseBlock(parser);
		if (child !== undefined) {
			if (Array.isArray(child)) {
				body.push(...child);
			} else {
				body.push(child);
			}
		}
	}

	parser.finalize(false);

	return parser.finishNode(
		start,
		parser.finishRoot({
			type: "MarkdownRoot",
			body,
		}),
	);
}

function consumeCode(
	parser: MarkdownParser,
	tokenizer: MarkdownParser["tokenizer"],
) {
	if (tokenizer.consume("```")) {
		const languageValue = tokenizer.read(isntLineBreak);

		const value = tokenizer.read((_, index) => {
			const firstChar = tokenizer.get(1);
			const secondChar = tokenizer.get(2);
			const thirdChar = tokenizer.get(3);
			return !(firstChar === "`" && secondChar === "`" && thirdChar === "`");
		});

		tokenizer.assert("```");

		return tokenizer.finishComplexToken<"Code", CodeProperties>(
			"Code",
			{
				language: languageValue || "unknown",
				value,
			},
		);
	}

	return undefined;
}

export function tokenizeMarkdown(opts: ParserOptions) {
	return markdownParser.create(opts).getAllTokens();
}

export * from "./types";
export * from "./utils";
