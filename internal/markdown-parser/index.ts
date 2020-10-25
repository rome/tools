import {
	ParserCore,
	ParserOptionsWithRequiredPath,
	createParser,
	isDigit,
	readUntilLineBreak,
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
import {CodeProperties, MarkdownParserState, Tokens} from "./types";
import {hasThematicBreak, isntInlineCharacter} from "./utils";
import {descriptions} from "@internal/diagnostics";
import {createMarkdownInitialState} from "@internal/markdown-parser/State";
import {
	parseListItem,
	tokenizeListItem,
} from "@internal/markdown-parser/parser/listItem";
import {tokenizeTextWrapping} from "@internal/markdown-parser/parser/textwrapping";
import {parseParagraph} from "@internal/markdown-parser/parser/paragraph";
import {parseText} from "@internal/markdown-parser/parser/text";
import {parseReference} from "@internal/markdown-parser/parser/reference";
import {parseTable, tokenizeTable} from "./parser/table";

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
	getInitialState: () => createMarkdownInitialState(),

	tokenizeWithState(parser, index, state) {
		const char = parser.getInputCharOnly(index);
		const escaped = isEscaped(index, parser.input);
		if (!escaped) {
			if (char === "[") {
				return {
					token: parser.finishToken("OpenSquareBracket"),
					state,
				};
			}
			if (char === "]") {
				return {
					token: parser.finishToken("CloseSquareBracket"),
					state,
				};
			}

			if (char === "(") {
				return {
					token: parser.finishToken("OpenBracket"),
					state,
				};
			}

			if (char === ")") {
				return {
					token: parser.finishToken("CloseBracket"),
					state,
				};
			}

			if (char === "|") {
				const token = tokenizeTable(parser, index);
				if (token) {
					return {token, state};
				}
			}
		}

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
								isListItem: true,
							},
						};
					}
				}

				return {
					token: parser.finishToken("NewLine"),
					state: {
						...state,
						isParagraph: false,
						isListItem: false,
					},
				};
			}
			if (char === "-") {
				const block = tokenizeBlock(parser, "-", index, char);
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
				const block = tokenizeBlock(parser, "_", index, char);
				if (block) {
					return {token: block, state};
				}

				const result = tokenizeTextWrapping(parser, state, "_", index);
				if (result) {
					return result;
				}
			}
			if (char === "*") {
				const block = tokenizeBlock(parser, "*", index, char);
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

				const result = tokenizeTextWrapping(parser, state, "*", index);
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
				const result = tokenizeTextWrapping(parser, state, "*", index);
				if (result) {
					return result;
				}
			}
			if (char === "_") {
				const result = tokenizeTextWrapping(parser, state, "_", index);
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
	const children: Array<MarkdownListItem> = [];
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

function parseBlock(
	parser: MarkdownParser,
): undefined | AnyMarkdownNode | Array<AnyMarkdownNode> {
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

		case "TablePipe":
			return parseTable(parser);

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
