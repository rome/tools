import {
	ParserOptionsWithRequiredPath,
	createParser,
} from "@internal/parser-core";
import {TomlRoot} from "@internal/ast";
import {isEscaped} from "@internal/string-utils";
import {TomlParser, TomlParserTypes} from "./types";
import {AnyTomlNode} from "@internal/ast/toml/unions";
import {parseKeyValue} from "@internal/toml-parser/parser/keyValue";
import {allowedCharacterForKey} from "@internal/toml-parser/utils";

const createTomlParser = createParser<TomlParserTypes>({
	diagnosticCategory: "parse/toml",
	ignoreWhitespaceTokens: false,
	getInitialState: () => ({
		inValue: undefined,
	}),
	tokenizeWithState(parser, index, state) {
		const char = parser.getInputCharOnly(index);
		const escaped = isEscaped(index, parser.input);

		if (!escaped && !state.inValue) {
			if (char === '"') {
				return {
					state: {
						...state,
						inValue: '"',
					},
					token: parser.finishToken("DoubleQuote"),
				};
			}

			if (char === "'") {
				return {
					state: {
						...state,
						inValue: "'",
					},
					token: parser.finishToken("SingleQuote"),
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

			if (char === "\n") {
				return {
					state,
					token: parser.finishToken("NewLine"),
				};
			}
			if (char === " ") {
				return {
					state,
					token: parser.finishToken("Space"),
				};
			}
		}

		if (!escaped && state.inValue) {
			if (char === '"') {
				return {
					state: {
						...state,
						inValue: undefined,
					},
					token: parser.finishToken("DoubleQuote"),
				};
			}
			if (char === "'") {
				return {
					state: {
						...state,
						inValue: undefined,
					},
					token: parser.finishToken("SingleQuote"),
				};
			}
		}

		if (state.inValue) {
			const [value, endIndex] = parser.readInputFrom(
				index,
				(char) => {
					return char !== state.inValue;
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
				return allowedCharacterForKey(char);
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
		case "DoubleQuote":
		case "SingleQuote":
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
