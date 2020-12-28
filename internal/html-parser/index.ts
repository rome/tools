import {
	ParserCore,
	ParserOptions,
	ParserOptionsWithRequiredPath,
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
import {Number0, ob1Add, ob1Get0, ob1Inc} from "@internal/ob1";
import {isEscaped} from "@internal/string-utils";
import {isSelfClosingTagName} from "./tags";
import {descriptions} from "@internal/diagnostics";
import {State, Tokens} from "@internal/html-parser/types";

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

type HTMLParserTypes = {
	tokens: Tokens;
	state: State;
	options: ParserOptions;
	meta: void;
};

type HTMLParser = ParserCore<HTMLParserTypes>;

const createHTMLParser = createParser<HTMLParserTypes>({
	ignoreWhitespaceTokens: true,
	diagnosticCategory: "parse/html",
	getInitialState: () => ({inTagHead: false, insertionMode: "Initial"}),

	tokenizeWithState(parser, index, state) {
		const escaped = isEscaped(index, parser.input);
		const char = parser.getInputCharOnly(index);

		if (!escaped && state.inTagHead) {
			if (char === "=") {
				return {
					state,
					token: parser.finishToken("Equals"),
				};
			}

			if (char === "/" && parser.getInputCharOnly(index, 1)) {
				return {
					state,
					token: parser.finishToken("TagSelfClosing", ob1Add(index, 2)),
				};
			}

			if (isIdentifierChar(char)) {
				const [value, end] = parser.readInputFrom(index, isIdentifierChar);
				return {
					state,
					token: parser.finishValueToken("Identifier", value, end),
				};
			}

			if (char === '"') {
				const [value, stringValueEnd, unclosed] = parser.readInputFrom(
					ob1Inc(index),
					isStringValueChar,
				);

				if (unclosed) {
					// TODO
				}

				const end = ob1Add(stringValueEnd, 1);
				return {
					state,
					token: parser.finishValueToken("String", value, end),
				};
			}

			if (char === ">") {
				return {
					state: {
						...state,
						inTagHead: false,
					},
					token: parser.finishToken("TagEnd"),
				};
			}
		}

		if (parser.getInputCharOnly(index) === "!") {
			const [isDoctype, value, endIndex] = consumeDOCTYPE(parser, index);
			if (isDoctype && value && endIndex) {
				return {
					state,
					token: parser.finishValueToken("Doctype", value, endIndex),
				};
			} else {
				const [isCdata, value, endIndex] = consumeCDATA(parser, index);
				if (isCdata && value && endIndex) {
					return {
						state,
						token: parser.finishValueToken("Cdata", value, endIndex),
					};
				}
			}
		}

		if (
			parser.getInputCharOnly(index) === "<" &&
			parser.getInputCharOnly(index, 1) === "!" &&
			parser.getInputCharOnly(index, 2) === "-" &&
			parser.getInputCharOnly(index, 3) === "-"
		) {
			// Skip <!--
			const start = ob1Add(index, 4);
			const [value, valueEnd, overflow] = parser.readInputFrom(
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
				token: parser.finishValueToken("Comment", value, end),
			};
		}

		if (isTagStartChar(index, parser.input)) {
			let token;

			if (parser.getInputCharOnly(index, 1) === "/") {
				token = parser.finishToken("TagEndOpen", ob1Add(index, 2));
			} else {
				token = parser.finishToken("TagStartOpen");
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
		const [value, end] = parser.readInputFrom(index, isTextChar);
		return {
			state,
			token: {
				type: "Text",
				value,
				start: index,
				end,
			},
		};
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
		if (valueToken.type === "Equals") {
			parser.nextToken();
			const value = parseString(parser);
			if (value && name) {
				return parser.finishNode(
					start,
					{
						type: "HTMLAttribute",
						name,
						value,
					},
				);
			}
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
		!parser.matchToken("EOF") &&
		!parser.matchToken("TagSelfClosing") &&
		!parser.matchToken("TagEnd")
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
			// Build children
			!parser.matchToken("EOF") &&
			!parser.matchToken("TagEndOpen")
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
	index: Number0,
): [boolean, string | undefined, Number0 | undefined] {
	// doc requires a token like this
	if (
		parser.getInputCharOnly(index, 1) === "D" &&
		parser.getInputCharOnly(index, 2) === "O" &&
		parser.getInputCharOnly(index, 3) === "C" &&
		parser.getInputCharOnly(index, 4) === "T" &&
		parser.getInputCharOnly(index, 5) === "Y" &&
		parser.getInputCharOnly(index, 6) === "P" &&
		parser.getInputCharOnly(index, 7) === "E" &&
		!isDigit(parser.getInputCharOnly(index, 8)) &&
		!isAlpha(parser.getInputCharOnly(index, 8))
	) {
		const [value, endIndex] = parser.readInputFrom(
			ob1Add(index, 9),
			(char) => {
				return char !== ">";
			},
		);
		// we skip the greater sign
		return [true, value.trim(), ob1Add(endIndex, 1)];
	}

	return [false, undefined, undefined];
}

function consumeCDATA(
	parser: HTMLParser,
	index: Number0,
): [boolean, string | undefined, Number0 | undefined] {
	// doc requires a token like this
	if (
		parser.getInputCharOnly(index, 1) === "[" &&
		parser.getInputCharOnly(index, 2) === "C" &&
		parser.getInputCharOnly(index, 3) === "D" &&
		parser.getInputCharOnly(index, 4) === "A" &&
		parser.getInputCharOnly(index, 5) === "T" &&
		parser.getInputCharOnly(index, 6) === "A" &&
		parser.getInputCharOnly(index, 7) === "["
	) {
		const [value, endIndex] = parser.readInputFrom(
			ob1Add(index, 8),
			(char, index, input) => {
				return !(char === "]" &&
				input[ob1Get0(index) + 1] === "]" &&
				input[ob1Get0(index) + 2] === ">");
			},
		);
		// we skip the greater sign
		return [true, value.trim(), ob1Add(endIndex, 3)];
	}

	return [false, undefined, undefined];
}

export function parseHTML(opts: ParserOptionsWithRequiredPath): HTMLRoot {
	const parser = createHTMLParser(opts);
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

export function tokenizeHTML(opts: ParserOptionsWithRequiredPath) {
	return createHTMLParser(opts).tokenizeAll();
}

export * from "./xhtmlEntities";
