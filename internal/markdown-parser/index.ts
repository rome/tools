import {
	ParserCore,
	ParserOptionsWithRequiredPath,
	createParser,
	isDigit,
	readUntilLineBreak,
} from "@internal/parser-core";
import {
	AnyMarkdownInlineNode,
	AnyMarkdownNode,
	MarkdownCodeBlock,
	MarkdownDividerBlock,
	MarkdownHeadingBlock,
	MarkdownListBlock,
	MarkdownListChildren,
	MarkdownListItem,
	MarkdownParagraph,
	MarkdownRoot,
	MarkdownText,
} from "@internal/ast";
import {Number0, ob1Add} from "@internal/ob1";
import {isEscaped} from "@internal/string-utils";
import {CodeProperties, MarkdownParserState, Tokens} from "./types";
import {hasThematicBreak, isBlockToken, isntInlineCharacter} from "./utils";
import {descriptions} from "@internal/diagnostics";
import {InlineState} from "@internal/markdown-parser/State";
import {tokenizeListItem} from "@internal/markdown-parser/parser/listItem";
import {
	parseInline,
	tokenizeInline,
} from "@internal/markdown-parser/parser/inline";

type MarkdownParserTypes = {
	tokens: Tokens;
	state: MarkdownParserState;
	options: ParserOptionsWithRequiredPath;
	meta: void;
};

export type MarkdownParser = ParserCore<MarkdownParserTypes>;

const createMarkdownParser = createParser<MarkdownParserTypes>({
	diagnosticCategory: "parse/markdown",
	ignoreWhitespaceTokens: false,
	getInitialState: () => ({
		isBlockHead: false,
		isParagraph: false,
		inlineState: new InlineState(),
	}),

	tokenizeWithState(parser, index, state) {
		const char = parser.getInputCharOnly(index);
		const escaped = isEscaped(index, parser.input);

		if (!escaped && !state.isParagraph) {
			if (char === "#") {
				return {
					token: consumeHeading(parser, index),
					state,
				};
			}
			if (char === "\n") {
				const nextChar = parser.getInputCharOnly(index, 1);
				if (nextChar === "#") {
					return {
						token: consumeHeading(parser, ob1Add(index, 1)),
						state,
					};
				}

				if (nextChar === "`") {
					const token = consumeCode(parser, ob1Add(index, 1));
					if (token) {
						return {state, token};
					}
				}

				if (isDigit(nextChar)) {
					const token = tokenizeListItem(parser, index);
					if (token) {
						return {
							token,
							state: {
								...state,
								isParagraph: true,
							},
						};
					}
				}

				return {
					token: parser.finishToken("NewLine"),
					state: {
						...state,
						isParagraph: false,
					},
				};
			}
			if (char === "-") {
				const block = consumeBlock(parser, "-", index, char);
				if (block) {
					return {token: block, state};
				}
				const listItemToken = tokenizeListItem(parser, index, "-");
				if (listItemToken) {
					return {
						token: listItemToken,
						state: {
							...state,
							isParagraph: true,
						},
					};
				}
			}
			if (char === "_") {
				const block = consumeBlock(parser, "_", index, char);
				if (block) {
					return {token: block, state};
				}

				const result = tokenizeInline(parser, state, "_", index);
				if (result) {
					return result;
				}
			}
			if (char === "*") {
				const block = consumeBlock(parser, "*", index, char);
				if (block) {
					return {token: block, state};
				}
				const listItemToken = tokenizeListItem(parser, index, "*");
				if (listItemToken) {
					return {
						token: listItemToken,
						state: {
							...state,
							isParagraph: true,
						},
					};
				}

				const result = tokenizeInline(parser, state, "*", index);
				if (result) {
					return result;
				}
			}
			if (char === "`") {
				const token = consumeCode(parser, index);
				if (token) {
					return {state, token};
				}
			}

			if (isDigit(char)) {
				const listItemToken = tokenizeListItem(parser, index);

				if (listItemToken) {
					return {
						token: listItemToken,
						state: {
							...state,
							isParagraph: true,
						},
					};
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
				return {
					token: parser.finishToken("NewLine"),
					state: {
						...state,
						isParagraph: false,
					},
				};
			}

			const [value, endIndex] = parser.readInputFrom(index, isntInlineCharacter);

			return {
				token: parser.finishValueToken("Text", value, endIndex),
				state: {
					...state,
					isParagraph: parser.getInputCharOnly(endIndex) !== "\n",
				},
			};
		}

		const [value, endIndex] = parser.readInputFrom(index, isntInlineCharacter);

		return {
			token: parser.finishValueToken("Text", value, endIndex),
			state: {
				...state,
				isParagraph: parser.getInputCharOnly(endIndex) !== "\n",
			},
		};
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
		const [textValue, endText] = parser.readInputFrom(end, readUntilLineBreak);
		return parser.finishValueToken("Text", value + textValue, endText);
	}
	return parser.finishValueToken("HeadingLevel", value.length, end);
}

function consumeBlock(
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

function parseText(parser: MarkdownParser): MarkdownText {
	const token = parser.expectToken("Text");
	const pos = parser.getPosition();
	return parser.finishNode(
		pos,
		{
			type: "MarkdownText",
			value: token.value,
		},
	);
}

function parseParagraph(
	parser: MarkdownParser,
	isList?: boolean,
): MarkdownParagraph {
	const start = parser.getPosition();
	const children: Array<AnyMarkdownInlineNode> = [];
	while (!parser.matchToken("EOF") && !isBlockToken(parser)) {
		const token = parser.getToken();

		if (isList && token.type === "NewLine") {
			parser.nextToken();
			break;
		}
		switch (token.type) {
			case "Strong":
			case "Emphasis": {
				const nodes = parseInline(
					parser,
					token,
					// TODO: to add support for more inline tokens: link, code inline block
					() => {
						return parseText(parser);
					},
				);
				if (nodes) {
					children.push(nodes);
				}

				parser.nextToken();
				break;
			}
			case "Text": {
				children.push(parseText(parser));
				break;
			}
			case "NewLine": {
				const pos = parser.getPosition();
				children.push(
					parser.finishNode(
						pos,
						{
							type: "MarkdownText",
							value: "\n",
						},
					),
				);
				parser.nextToken();
				break;
			}
			default: {
				// TODO: to remove once all cases are handled
				parser.unexpectedDiagnostic({
					description: descriptions.MARKDOWN_PARSER.INVALID_SEQUENCE,
				});
				parser.nextToken();
			}
		}
	}

	return parser.finishNode(
		start,
		{
			type: "MarkdownParagraph",
			children,
		},
	);
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

function parseItem(parser: MarkdownParser): MarkdownListItem {
	const token = parser.expectToken("ListItem");
	const pos = parser.getPosition();
	const children: Array<MarkdownListChildren> = [];

	while (
		!parser.matchToken("EOF") &&
		!parser.matchToken("NewLine") &&
		!parser.matchToken("ListItem") &&
		!parser.matchToken("Break") &&
		parser.matchToken("Text")
	) {
		children.push(parseParagraph(parser, true));
	}

	return parser.finishNode(
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

function parseListBlock(parser: MarkdownParser): MarkdownListBlock {
	const pos = parser.getPosition();
	const children: Array<MarkdownListItem> = [];
	const currentToken = parser.getToken();
	let ordered = false;
	if (currentToken.type === "ListItem") {
		if (currentToken.numeric) {
			ordered = true;
		}
	}
	while (!parser.matchToken("EOF") && parser.matchToken("ListItem")) {
		const item = parseItem(parser);
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

	while (!parser.matchToken("EOF") && !parser.matchToken("Code")) {
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

function parseBlock(parser: MarkdownParser): undefined | AnyMarkdownNode {
	const token = parser.getToken();
	switch (token.type) {
		case "NewLine": {
			parser.nextToken();
			return undefined;
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
	const body: Array<AnyMarkdownNode> = [];

	while (!parser.matchToken("EOF")) {
		const child = parseBlock(parser);
		if (child !== undefined) {
			body.push(child);
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
			readUntilLineBreak,
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
	return createMarkdownParser(opts).tokenizeAll();
}

export * from "./types";
export * from "./utils";
