import {
	ParserCore,
	ParserOptionsWithRequiredPath,
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
import {Number0, ob1Add} from "@internal/ob1";
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
import {parseText} from "@internal/markdown-parser/parser/text";
import {parseReference} from "@internal/markdown-parser/parser/reference";

export type MarkdownParser = ParserCore<MarkdownParserTypes>;

const createMarkdownParser = createParser<MarkdownParserTypes>({
	diagnosticLanguage: "markdown",
	ignoreWhitespaceTokens: false,
	getInitialState: () => createMarkdownInitialState(),

	tokenizeWithState(parser, index, state) {
		const char = parser.getInputCharOnly(index);
		const escaped = isEscaped(index, parser.input);

		if (!escaped) {
			if (char === "[") {
				return [state, parser.finishToken("OpenSquareBracket")];
			}

			if (char === "]") {
				return [state, parser.finishToken("CloseSquareBracket")];
			}

			if (char === "(") {
				return [state, parser.finishToken("OpenBracket")];
			}

			if (char === ")") {
				return [state, parser.finishToken("CloseBracket")];
			}
		}

		if (!(escaped || state.isParagraph)) {
			if (char === "#") {
				return [state, consumeHeading(parser, index)];
			}
			if (char === "\n") {
				const nextChar = parser.getInputCharOnly(index, 1);
				if (nextChar === "#") {
					return [state, consumeHeading(parser, ob1Add(index, 1))];
				}

				if (nextChar === "`") {
					const token = consumeCode(parser, ob1Add(index, 1));
					if (token) {
						return [state, token];
					}
				}

				if (isDigit(nextChar)) {
					const token = tokenizeListItem(parser, index);
					if (token) {
						return [
							{
								isParagraph: true,
								isListItem: true,
							},
							token,
						];
					}
				}

				return [
					{
						isParagraph: false,
						isListItem: false,
					},
					parser.finishToken("NewLine"),
				];
			}

			if (char === "-") {
				const block = tokenizeBlock(parser, "-", index, char);
				if (block) {
					return [state, block];
				}

				const listItemToken = tokenizeListItem(parser, index, "-");
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
				const block = tokenizeBlock(parser, "_", index, char);
				if (block) {
					return [state, block];
				}

				const result = tokenizeInline(parser, state, "_", index);
				if (result) {
					return result;
				}
			}

			if (char === "*") {
				const block = tokenizeBlock(parser, "*", index, char);
				if (block) {
					return [state, block];
				}

				const listItemToken = tokenizeListItem(parser, index, "*");
				if (listItemToken) {
					return [
						{
							isParagraph: true,
						},
						listItemToken,
					];
				}

				const result = tokenizeInline(parser, state, "*", index);
				if (result) {
					return result;
				}
			}

			if (char === "`") {
				const token = consumeCode(parser, index);
				if (token) {
					return [state, token];
				}
			}

			if (isDigit(char)) {
				const listItemToken = tokenizeListItem(parser, index);

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
				const result = tokenizeInline(parser, state, "*", index);
				if (result) {
					return result;
				}
			}

			if (char === "_") {
				const result = tokenizeInline(parser, state, "_", index);
				if (result) {
					return result;
				}
			}

			if (char === "\n") {
				return [
					{
						isParagraph: false,
					},
					parser.finishToken("NewLine"),
				];
			}

			const [value, endIndex] = parser.readInputFrom(index, isntInlineCharacter);

			return [
				{
					isParagraph: parser.getInputCharOnly(endIndex) !== "\n",
				},
				parser.finishValueToken("Text", value, endIndex),
			];
		}

		const [value, endIndex] = parser.readInputFrom(index, isntInlineCharacter);

		return [
			{
				isParagraph: parser.getInputCharOnly(endIndex) !== "\n",
			},
			parser.finishValueToken("Text", value, endIndex),
		];
	},
});

function consumeHeading(parser: MarkdownParser, index: Number0) {
	const [value, end] = parser.readInputFrom(
		index,
		(char1) => {
			return char1 === "#";
		},
	);
	if (value.length > 6) {
		const [textValue, endText] = parser.readInputFrom(end, isntLineBreak);
		return parser.finishValueToken("Text", value + textValue, endText);
	}
	return parser.finishValueToken("HeadingLevel", value.length, end);
}

function tokenizeBlock(
	parser: MarkdownParser,
	blockChar: string,
	index: Number0,
	currentChar: string,
) {
	const nextChar = parser.getInputCharOnly(index, 1);
	const nextNextChar = parser.getInputCharOnly(index, 2);
	if (hasThematicBreak([currentChar, nextChar, nextNextChar].join(""))) {
		// by spec, should be at least 3, with an infinite number
		const [value, endIndex] = parser.readInputFrom(
			index,
			(char) => {
				return char === blockChar;
			},
		);
		return parser.finishValueToken("Break", value, endIndex);
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
	let value;

	while (!(parser.matchToken("EOF") || parser.matchToken("Code"))) {
		const token = parser.getToken();
		if (token.type === "Text") {
			value = parseText(parser);
		}

		parser.nextToken();
	}
	parser.nextToken();

	return parser.finishNode(
		start,
		{
			type: "MarkdownCodeBlock",
			language: token.language,
			value,
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

		case "Text":
			return parseParagraph(parser);

		case "Code":
			return parseCode(parser);

		default: {
			throw parser.unexpected();
		}
	}
}

export function parseMarkdown(opts: ParserOptionsWithRequiredPath): MarkdownRoot {
	const parser = createMarkdownParser(opts);
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

	parser.finalize();

	return parser.finishNode(
		start,
		parser.finishRoot({
			type: "MarkdownRoot",
			body,
		}),
	);
}

function consumeCode(parser: MarkdownParser, index: Number0) {
	const nextChar = parser.getInputCharOnly(index, 1);
	const nextNextChar = parser.getInputCharOnly(index, 2);
	if (nextChar === "`" && nextNextChar === "`") {
		const [languageValue, endIndex] = parser.readInputFrom(
			ob1Add(index, 3),
			isntLineBreak,
		);

		return parser.finishComplexToken<"Code", CodeProperties>(
			"Code",
			{
				language: languageValue || "unknown",
			},
			endIndex,
		);
	}

	return undefined;
}

export function tokenizeMarkdown(opts: ParserOptionsWithRequiredPath) {
	return createMarkdownParser(opts).getAllTokens();
}

export * from "./types";
export * from "./utils";
