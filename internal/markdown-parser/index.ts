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
import {ListProperties, Tokens} from "./types";
import {hasThematicBreak} from "./utils";
import {descriptions} from "@internal/diagnostics";

type MarkdownParserTypes = {
	tokens: Tokens;
	state: {};
	options: ParserOptionsWithRequiredPath;
	meta: void;
};

type MarkdownParser = ParserCore<MarkdownParserTypes>;

const createMarkdownParser = createParser<MarkdownParserTypes>({
	diagnosticCategory: "parse/markdown",
	ignoreWhitespaceTokens: true,
	getInitialState: () => ({isBlockHead: false}),

	tokenizeWithState(parser, index, state) {
		const char = parser.getInputCharOnly(index);
		const escaped = isEscaped(index, parser.input);
		if (!escaped) {
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
				if (nextChar === "\n") {
					return {
						token: parser.finishToken("EndParagraph", ob1Add(index, 1)),
						state,
					};
				}

				if (isDigit(nextChar)) {
					const [, endIndex] = parser.readInputFrom(index, isDigit);
					const nextChar = parser.getInputCharOnly(endIndex);
					const nextNextChar = parser.getInputCharOnly(endIndex, 1);

					if (nextChar === "." && nextNextChar === " ") {
						return {
							token: parser.finishComplexToken<"ListItem", ListProperties>(
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
					token: parser.finishToken("NewLine"),
					state,
				};
			}

			// dividers
			if (char === "-") {
				const block = consumeBlock(parser, "-", index, char);
				if (block) {
					return {token: block, state};
				}
				const nextChar = parser.getInputCharOnly(index, 1);
				if (nextChar === " ") {
					return {
						token: parser.finishComplexToken<"ListItem", ListProperties>(
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
				const block = consumeBlock(parser, "_", index, char);
				if (block) {
					return {token: block, state};
				}
			}
			if (char === "*") {
				const block = consumeBlock(parser, "*", index, char);
				if (block) {
					return {token: block, state};
				}
				const nextChar = parser.getInputCharOnly(index, 1);
				if (nextChar === " ") {
					return {
						token: parser.finishComplexToken<"ListItem", ListProperties>(
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
				const [, endIndex] = parser.readInputFrom(index, isDigit);
				const nextChar = parser.getInputCharOnly(endIndex);
				const nextNextChar = parser.getInputCharOnly(endIndex, 1);
				if (nextChar === "." && nextNextChar === " ") {
					return {
						token: parser.finishComplexToken<"ListItem", ListProperties>(
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

		const [value, endIndex] = parser.readInputFrom(index, readUntilLineBreak);

		return {
			token: parser.finishValueToken("Text", value, endIndex),
			state,
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
	while (
		!parser.matchToken("EOF") &&
		!parser.matchToken("EndParagraph") &&
		!parser.matchToken("Break")
	) {
		const token = parser.getToken();
		if (isList && token.type === "NewLine") {
			parser.nextToken();
			break;
		}
		switch (token.type) {
			case "Break": {
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

function parseBlock(parser: MarkdownParser): undefined | AnyMarkdownNode {
	const token = parser.getToken();
	switch (token.type) {
		case "NewLine": {
			parser.nextToken();
			return undefined;
		}

		case "EndParagraph": {
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

		default:
			throw parser.unexpected();
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

export function tokenizeMarkdown(opts: ParserOptionsWithRequiredPath) {
	return createMarkdownParser(opts).tokenizeAll();
}

export * from "./types";
export * from "./utils";
