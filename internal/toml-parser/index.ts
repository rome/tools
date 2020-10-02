import {
	ParserOptionsWithRequiredPath,
	createParser,
	readUntilLineBreak,
} from "@internal/parser-core";
import {TomlRoot} from "@internal/ast";
import {isEscaped} from "@internal/string-utils";
import {TomlParser, TomlParserTypes} from "./types";
import {parseText} from "@internal/toml-parser/parser/text";
import {AnyTomlNode} from "@internal/ast/toml/unions";

const createTomlParser = createParser<TomlParserTypes>({
	diagnosticCategory: "parse/toml",
	ignoreWhitespaceTokens: false,
	getInitialState: () => ({}),
	tokenizeWithState(parser, index, state) {
		const char = parser.getInputCharOnly(index);
		const escaped = isEscaped(index, parser.input);

		if (!escaped) {
			if (char === "[") {
				return {
					state,
					token: parser.finishToken("OpenSquareBracket", index),
				};
			}
			if (char === "]") {
				return {
					state,
					token: parser.finishToken("CloseSquareBracket", index),
				};
			}

			if (char === "=") {
				return {
					state,
					token: parser.finishToken("Equals", index),
				};
			}
		}

		const [value, endIndex] = parser.readInputFrom(index, readUntilLineBreak);

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
			return parseText(parser);
		}
		default: {
			throw parser.unexpected();
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
