import {
	ParserCore,
	ParserCoreTokenizeState,
	ParserOptions,
	createParser,
	isAlpha,
	isDigit,
} from "@internal/parser-core";
import {
	AnyHTMLChildNode,
	HTMLAttribute,
	HTMLCdataTag,
	HTMLDoctypeTag,
	HTMLElement,
	HTMLIdentifier,
	HTMLRoot,
	HTMLString,
	HTMLText,
} from "@internal/ast";
import {ZeroIndexed} from "@internal/numbers";
import {isEscaped} from "@internal/string-utils";
import {isSelfClosingTagName} from "./tags";
import {descriptions} from "@internal/diagnostics";
import {State, Tokens} from "@internal/html-parser/types";

function isTagStartChar(index: ZeroIndexed, input: string): boolean {
	const i = index.valueOf();
	return input[i] === "<" && !isEscaped(index, input);
}

function isStringValueChar(
	char: string,
	index: ZeroIndexed,
	input: string,
): boolean {
	if (char === '"' && !isEscaped(index, input)) {
		return false;
	}

	return true;
}

function isIdentifierChar(char: string): boolean {
	return isDigit(char) || isAlpha(char) || char === "-";
}

function isTextChar(char: string, index: ZeroIndexed, input: string): boolean {
	return !isTagStartChar(index, input);
}

function isntCommentEnd(
	char: string,
	index: ZeroIndexed,
	input: string,
): boolean {
	const isCommentEnd =
		char === "-" &&
		!isEscaped(index, input) &&
		input[index.valueOf() + 1] === "-" &&
		input[index.valueOf() + 2] === ">";
	return !isCommentEnd;
}

type HTMLParserTypes = {
	tokens: Tokens;
	state: State;
	options: ParserOptions;
	meta: void;
};

type HTMLParser = ParserCore<HTMLParserTypes>;

const htmlParser = createParser<HTMLParserTypes>({
	ignoreWhitespaceTokens: true,
	diagnosticLanguage: "html",
	getInitialState: () => ({inTagHead: false}),

	tokenizeWithState(
		parser,
		index,
		state,
	): ParserCoreTokenizeState<HTMLParserTypes> {
		const escaped = isEscaped(index, parser.input);
		const char = parser.getInputCharOnly(index);

		if (!escaped && state.inTagHead) {
			if (char === "=") {
				return [state, parser.finishToken("Equals")];
			}

			if (char === "/" && parser.getInputCharOnly(index.increment())) {
				return [state, parser.finishToken("TagSelfClosing", index.add(2))];
			}

			if (isIdentifierChar(char)) {
				const [value, end] = parser.readInputFrom(index, isIdentifierChar);
				return [state, parser.finishValueToken("Identifier", value, end)];
			}

			if (char === '"') {
				const [value, stringValueEnd, unclosed] = parser.readInputFrom(
					index.increment(),
					isStringValueChar,
				);

				if (unclosed) {
					// TODO
				}

				const end = stringValueEnd.add(1);
				return [state, parser.finishValueToken("String", value, end)];
			}

			if (char === ">") {
				return [
					{
						inTagHead: false,
					},
					parser.finishToken("TagEnd"),
				];
			}
		}

		if (parser.getInputCharOnly(index) === "!") {
			const [isDoctype, value, endIndex] = consumeDOCTYPE(parser, index);
			if (isDoctype && value && endIndex) {
				return [state, parser.finishValueToken("Doctype", value, endIndex)];
			} else {
				const [isCdata, value, endIndex] = consumeCDATA(parser, index);
				if (isCdata && value && endIndex) {
					return [state, parser.finishValueToken("Cdata", value, endIndex)];
				}
			}
		}

		if (
			parser.getInputCharOnly(index) === "<" &&
			parser.getInputCharOnly(index.increment()) === "!" &&
			parser.getInputCharOnly(index.add(2)) === "-" &&
			parser.getInputCharOnly(index.add(3)) === "-"
		) {
			// Skip <!--
			const start = index.add(4);
			const [value, valueEnd, overflow] = parser.readInputFrom(
				start,
				isntCommentEnd,
			);

			// Check for unclosed comment
			if (overflow) {
				// TODO
			}

			// Skip -->
			const end = valueEnd.add(3);

			return [
				{
					inTagHead: false,
				},
				parser.finishValueToken("Comment", value, end),
			];
		}

		if (isTagStartChar(index, parser.input)) {
			let token;

			if (parser.getInputCharOnly(index.increment()) === "/") {
				token = parser.finishToken("TagEndOpen", index.add(2));
			} else {
				token = parser.finishToken("TagStartOpen");
			}

			return [
				{
					inTagHead: true,
				},
				token,
			];
		}

		// Keep eating text until we hit a <
		const [value, end] = parser.readInputFrom(index, isTextChar);
		return [
			state,
			{
				type: "Text",
				value,
				start: index,
				end,
			},
		];
	},
});

function parseIdentifier(parser: HTMLParser): HTMLIdentifier | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();
	if (token.type === "Identifier") {
		parser.nextToken();
		return parser.finishNode(
			start,
			{
				type: "HTMLIdentifier",
				name: token.value,
			},
		);
	}

	parser.nextToken();
	return undefined;
}

function parseString(parser: HTMLParser): HTMLString | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();
	if (token.type === "String") {
		parser.nextToken();
		return parser.finishNode(
			start,
			{
				type: "HTMLString",
				value: token.value.trim(),
			},
		);
	}
	parser.unexpectedDiagnostic({
		description: descriptions.HTML_PARSER.INVALID_ATTRIBUTE_NAME,
		token,
	});
	parser.nextToken();
	return undefined;
}

function parseAttribute(parser: HTMLParser): HTMLAttribute | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();
	if (token.type === "Identifier") {
		const name = parseIdentifier(parser);
		const valueToken = parser.getToken();
		let value: HTMLString | undefined;
		if (valueToken.type === "Equals") {
			parser.nextToken();
			value = parseString(parser);
		}
		if (name) {
			return parser.finishNode(
				start,
				{
					type: "HTMLAttribute",
					name,
					value,
				},
			);
		}
		parser.unexpectedDiagnostic({
			description: descriptions.HTML_PARSER.EXPECTED_ATTRIBUTE_NAME,
			token,
		});
	}
	return undefined;
}

function parseTag(
	parser: HTMLParser,
): HTMLElement | HTMLDoctypeTag | HTMLCdataTag | undefined {
	const headStart = parser.getPosition();
	if (!parser.eatToken("TagStartOpen")) {
		parser.unexpectedDiagnostic({
			description: descriptions.HTML_PARSER.UNKNOWN_START,
		});
		return undefined;
	}
	if (parser.matchToken("Doctype")) {
		return parseDoctype(parser);
	}
	if (parser.matchToken("Cdata")) {
		return parseCdata(parser);
	}

	const attributes: HTMLElement["attributes"] = [];
	const children: HTMLElement["children"] = [];

	const name = parseIdentifier(parser);
	if (!name) {
		parser.unexpectedDiagnostic({
			description: descriptions.HTML_PARSER.TAGNAME_NOT_FOUND,
		});
		parser.nextToken();
		return undefined;
	}
	const tagName = name.name;
	let selfClosing = isSelfClosingTagName(tagName);

	// Parse attributes
	while (
		!(parser.matchToken("EOF") ||
		parser.matchToken("TagSelfClosing") ||
		parser.matchToken("TagEnd"))
	) {
		const keyToken = parser.getToken();

		if (keyToken.type === "Identifier") {
			const attribute = parseAttribute(parser);
			if (attribute) {
				attributes.push(attribute);
			}
		} else {
			parser.unexpectedDiagnostic({
				description: descriptions.HTML_PARSER.EXPECTED_ATTRIBUTE_NAME,
			});
			parser.nextToken();
		}
	}

	if (parser.eatToken("TagSelfClosing")) {
		selfClosing = true;
	} else {
		if (parser.getToken().type !== "TagEnd") {
			parser.unexpectedDiagnostic({
				description: descriptions.HTML_PARSER.TAGEND_NOT_FOUND(tagName),
			});
		}
		parser.nextToken();
	}

	const headEnd = parser.getPosition();

	// Verify closing tag
	if (!selfClosing) {
		while (
			!// Build children
			(parser.matchToken("EOF") || parser.matchToken("TagEndOpen"))
		) {
			const child = parseChild(parser);
			if (child !== undefined) {
				children.push(child);
			}
		}

		if (parser.matchToken("EOF")) {
			parser.unexpectedDiagnostic({
				description: descriptions.HTML_PARSER.UNCLOSED_TAG(
					tagName,
					parser.finishLocAt(headStart, headEnd),
				),
			});
			parser.nextToken();
			return undefined;
		} else {
			if (!parser.matchToken("TagEndOpen")) {
				parser.unexpectedDiagnostic({
					description: descriptions.HTML_PARSER.TAGEND_NOT_FOUND(tagName),
				});
			}
			parser.nextToken();

			const name = parser.getToken();
			if (name.type === "Identifier") {
				if (name.value !== tagName) {
					parser.unexpectedDiagnostic({
						description: descriptions.HTML_PARSER.INCORRECT_CLOSING_TAG_NAME(
							tagName,
							name.value,
						),
					});
					parser.nextToken();
					return undefined;
				}

				parser.nextToken();
			} else {
				parser.unexpectedDiagnostic({
					description: descriptions.HTML_PARSER.EXPECTED_CLOSING_TAG_NAME(
						tagName,
					),
				});
				parser.nextToken();
			}

			if (!parser.matchToken("TagEnd")) {
				parser.unexpectedDiagnostic({
					description: descriptions.HTML_PARSER.TAGEND_NOT_FOUND(tagName),
					token: name,
				});
			}
			parser.nextToken();
		}
	}

	return parser.finishNode(
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

function parseDoctype(parser: HTMLParser): HTMLDoctypeTag | undefined {
	const token = parser.getToken();
	const start = parser.getPosition();
	if (token.type === "Doctype") {
		if (token.value !== "html") {
			parser.unexpectedDiagnostic({
				description: descriptions.HTML_PARSER.UNSUPPORTED_DOCTYPE(token.value),
			});
		} else {
			parser.nextToken();
			return parser.finishNode(
				start,
				{
					type: "HTMLDoctypeTag",
					value: token.value,
				},
			);
		}
	}
	parser.nextToken();
	return undefined;
}

function parseCdata(parser: HTMLParser): HTMLCdataTag | undefined {
	const token = parser.getToken();
	const start = parser.getPosition();
	if (token.type === "Cdata") {
		parser.nextToken();
		return parser.finishNode(
			start,
			{
				type: "HTMLCdataTag",
				value: token.value,
			},
		);
	}
	parser.nextToken();
	return undefined;
}

function parseComment(parser: HTMLParser): undefined {
	const start = parser.getPosition();

	const token = parser.getToken();

	if (token.type === "Comment") {
		parser.nextToken();
		parser.registerComment(
			parser.comments.createComment({
				value: token.value,
				type: "CommentBlock",
				loc: parser.finishLoc(start),
			}),
		);
	}

	return undefined;
}

function parseText(parser: HTMLParser): HTMLText | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();

	if (token.type === "Text") {
		const lines: string[] = [];
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

		parser.nextToken();
		return parser.finishNode(
			start,
			{
				type: "HTMLText",
				value,
			},
		);
	}

	parser.nextToken();
	return undefined;
}

function parseChild(parser: HTMLParser): undefined | AnyHTMLChildNode {
	const token = parser.getToken();

	switch (token.type) {
		case "Doctype":
			return parseDoctype(parser);

		case "TagStartOpen":
			return parseTag(parser);

		case "Text":
			return parseText(parser);

		case "Comment":
			return parseComment(parser);

		case "TagEndOpen": {
			parser.unexpectedDiagnostic({
				description: descriptions.HTML_PARSER.UNOPENED_TAG,
			});
			parser.nextToken();
			return undefined;
		}

		default: {
			parser.unexpectedDiagnostic();
			parser.nextToken();
			return undefined;
		}
	}
}

function consumeDOCTYPE(
	parser: HTMLParser,
	index: ZeroIndexed,
): [boolean, string | undefined, ZeroIndexed | undefined] {
	// doc requires a token like this
	if (
		parser.getInputRangeOnly(index.increment(), 7) === "DOCTYPE" &&
		!isDigit(parser.getInputCharOnly(index.add(8))) &&
		!isAlpha(parser.getInputCharOnly(index.add(8)))
	) {
		const [value, endIndex] = parser.readInputFrom(
			index.add(9),
			(char) => {
				return char !== ">";
			},
		);
		// we skip the greater sign
		return [true, value.trim(), endIndex.add(1)];
	}

	return [false, undefined, undefined];
}

function consumeCDATA(
	parser: HTMLParser,
	index: ZeroIndexed,
): [boolean, string | undefined, ZeroIndexed | undefined] {
	// doc requires a token like this
	if (parser.getInputRangeOnly(index.increment(), 7) === "[CDATA[") {
		const [value, endIndex] = parser.readInputFrom(
			index.add(8),
			(char, index, input) => {
				return !(char === "]" &&
				input[index.valueOf() + 1] === "]" &&
				input[index.valueOf() + 2] === ">");
			},
		);
		// we skip the greater sign
		return [true, value.trim(), endIndex.add(3)];
	}

	return [false, undefined, undefined];
}

export function parseHTML(opts: ParserOptions): HTMLRoot {
	const parser = htmlParser.create(opts);
	const start = parser.getPosition();
	const body: AnyHTMLChildNode[] = [];

	while (!parser.matchToken("EOF")) {
		const child = parseChild(parser);
		if (child !== undefined) {
			body.push(child);
		}
	}

	parser.finalize();

	return parser.finishNode(
		start,
		parser.finishRoot({
			type: "HTMLRoot",
			body,
		}),
	);
}

export function tokenizeHTML(opts: ParserOptions) {
	return htmlParser.create(opts).getAllTokens();
}

export * from "./xhtmlEntities";
