import {
	BaseTokens,
	ParserCore,
	ParserOptions,
	ParserOptionsWithRequiredPath,
	SimpleToken,
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

type State = {
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
	getInitialState: () => ({inTagHead: false}),

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

function parseIdentifier(parser: HTMLParser): HTMLIdentifier {
	const start = parser.getPosition();
	const token = parser.expectToken("Identifier");
	return parser.finishNode(
		start,
		{
			type: "HTMLIdentifier",
			name: token.value,
		},
	);
}

function parseString(parser: HTMLParser): HTMLString {
	const start = parser.getPosition();
	const value = parser.expectToken("String").value;
	return parser.finishNode(
		start,
		{
			type: "HTMLString",
			value,
		},
	);
}

function parseAttribute(parser: HTMLParser): HTMLAttribute {
	const start = parser.getPosition();
	const name = parseIdentifier(parser);
	parser.expectToken("Equals");
	const value = parseString(parser);
	return parser.finishNode(
		start,
		{
			type: "HTMLAttribute",
			name,
			value,
		},
	);
}

function parseTag(parser: HTMLParser): HTMLElement {
	const headStart = parser.getPosition();
	parser.expectToken("TagStartOpen");

	const attributes: HTMLElement["attributes"] = [];
	const children: HTMLElement["children"] = [];

	const name = parseIdentifier(parser);
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
			attributes.push(parseAttribute(parser));
		} else {
			throw parser.unexpected({
				description: descriptions.HTML_PARSER.EXPECTED_ATTRIBUTE_NAME,
			});
		}
	}

	if (parser.eatToken("TagSelfClosing")) {
		selfClosing = true;
	} else {
		parser.expectToken("TagEnd");
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
			throw parser.unexpected({
				description: descriptions.HTML_PARSER.UNCLOSED_TAG(
					tagName,
					parser.finishLocAt(headStart, headEnd),
				),
			});
		} else {
			parser.expectToken("TagEndOpen");

			const name = parser.getToken();
			if (name.type === "Identifier") {
				if (name.value !== tagName) {
					throw parser.unexpected({
						description: descriptions.HTML_PARSER.INCORRECT_CLOSING_TAG_NAME(
							tagName,
							name.value,
						),
					});
				}

				parser.nextToken();
			} else {
				throw parser.unexpected({
					description: descriptions.HTML_PARSER.EXPECTED_CLOSING_TAG_NAME,
				});
			}

			parser.expectToken("TagEnd");
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

function parseComment(parser: HTMLParser): undefined {
	const start = parser.getPosition();
	const token = parser.expectToken("Comment");

	parser.registerComment(
		parser.comments.createComment({
			value: token.value,
			type: "CommentBlock",
			loc: parser.finishLoc(start),
		}),
	);
	return undefined;
}

function parseText(parser: HTMLParser): HTMLText {
	const start = parser.getPosition();
	const token = parser.expectToken("Text");

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

	return parser.finishNode(
		start,
		{
			type: "HTMLText",
			value,
		},
	);
}

function parseChild(parser: HTMLParser): undefined | AnyHTMLChildNode {
	const token = parser.getToken();

	switch (token.type) {
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

export function parseHTML(opts: ParserOptionsWithRequiredPath): HTMLRoot {
	const parser = createHTMLParser(opts);
	const start = parser.getPosition();
	const body: Array<AnyHTMLChildNode> = [];

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
