import {
	ParserOptionsWithRequiredPath,
	createParser,
	isAlpha,
	isDigit,
} from "@internal/parser-core";
import {TomlRoot} from "@internal/ast";
import {isEscaped} from "@internal/string-utils";
import {TomlParser, TomlParserTypes} from "./types";
import {AnyTomlNode} from "@internal/ast/toml/unions";
import {parseKeyValue} from "@internal/toml-parser/parser/keyValue";

const createTomlParser = createParser<TomlParserTypes>({
	diagnosticCategory: "parse/toml",
	ignoreWhitespaceTokens: false,
	getInitialState: () => ({
		inValue: false,
	}),
	tokenizeWithState(parser, index, state) {
		const char = parser.getInputCharOnly(index);
		const escaped = isEscaped(index, parser.input);

		if (!escaped && !state.inValue) {
			if (char === '"') {
				return {
					state: {
						...state,
						inValue: true,
					},
					token: parser.finishToken("Quote"),
				};
			}

			if (char === "[") {
				return {
					state,
					token: parser.finishToken("OpenSquareBracket"),
				};
			}
			if (char === "]") {
				return {
					state,
					token: parser.finishToken("CloseSquareBracket"),
				};
			}

			if (char === "=") {
				return {
					state,
					token: parser.finishToken("Equals"),
				};
			}

			if (char === "{") {
				return {
					state,
					token: parser.finishToken("OpenCurlyBracket"),
				};
			}

			if (char === "}") {
				return {
					state,
					token: parser.finishToken("CloseCurlyBracket"),
				};
			}

			if (char === "") {
				return {
					state,
					token: parser.finishToken("Equals"),
				};
			}

			if (char === "\n") {
				return {
					state,
					token: parser.finishToken("NewLine"),
				};
			}
		}

		if (!escaped && state.inValue) {
			if (char === '"') {
				return {
					state: {
						...state,
						inValue: false,
					},
					token: parser.finishToken("Quote"),
				};
			}
		}

		if (state.inValue) {
			const [value, endIndex] = parser.readInputFrom(
				index,
				(char) => {
					return char !== '"';
				},
			);

			return {
				state,
				token: parser.finishValueToken("Text", value, endIndex),
			};
		}

		const [value, endIndex] = parser.readInputFrom(
			index,
			(char) => {
				return isAlpha(char) || isDigit(char) || char === " ";
			},
		);

		return {
			state,
			token: parser.finishValueToken("Text", value, endIndex),
		};
	},
});

function parseChild(parser: TomlParser) {
	const token = parser.getToken();

	switch (token.type) {
		case "Text": {
			return parseKeyValue(parser);
		}
		default: {
			parser.unexpectedDiagnostic();
			parser.nextToken();
			return undefined;
		}
	}
}

export function parseToml(opts: ParserOptionsWithRequiredPath): TomlRoot {
	const parser = createTomlParser(opts);
	const start = parser.getPosition();
	const body: Array<AnyTomlNode> = [];

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
			type: "TomlRoot",
			body,
		}),
	);
}

export function tokenizeToml(opts: ParserOptionsWithRequiredPath) {
	return createTomlParser(opts).tokenizeAll();
}
