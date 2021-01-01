import {ParserOptions, TokenBase, createParser} from "@internal/parser-core";
import {isEscaped} from "@internal/string-utils";
import {TOMLParser, TOMLParserTypes, TOMLValue} from "./types";
import {JSONObject} from "@internal/codec-config";
import {
	ConfigParserOptions,
	ConfigParserResult,
	PartialConfigHandler,
	PartialConsumeConfigResult,
} from "@internal/codec-config/types";

import {descriptions} from "@internal/diagnostics";
import {Number0, ob1Inc} from "@internal/ob1";

function isSingleStringValueChar(
	char: string,
	index: Number0,
	input: string,
): boolean {
	return !(char === "'" && !isEscaped(index, input));
}
function isDoubleStringValueChar(
	char: string,
	index: Number0,
	input: string,
): boolean {
	return !(char === "'" && !isEscaped(index, input));
}

function allowedCharacterForKey(char: string) {
	return char !== undefined && /^[A-Za-z0-9_\-]+$/.test(char);
}

const createTomlParser = createParser<TOMLParserTypes>({
	diagnosticLanguage: "toml",
	ignoreWhitespaceTokens: true,
	getInitialState: () => ({
		inValue: undefined,
	}),
	tokenize(parser, index) {
		const char = parser.getInputCharOnly(index);

		switch (char) {
			case "'":
			case '"': {
				const [value, end] = parser.readInputFrom(
					ob1Inc(index),
					char === '"' ? isDoubleStringValueChar : isSingleStringValueChar,
				);

				// TODO check overflow

				// TODO string unescaping
				return parser.finishValueToken("String", value, ob1Inc(end));
			}

			case "[":
				return parser.finishToken("OpenSquareBracket");

			case "]":
				return parser.finishToken("CloseSquareBracket");

			case "=":
				return parser.finishToken("Equals");

			case "{":
				return parser.finishToken("OpenCurlyBracket");

			case "}":
				return parser.finishToken("CloseCurlyBracket");
		}

		if (allowedCharacterForKey(char)) {
			const [value, endIndex] = parser.readInputFrom(
				index,
				allowedCharacterForKey,
			);

			return parser.finishValueToken("Text", value, endIndex);
		} else {
			// Invalid but we'll reverify it wqith allowedCharacterForKey later
			return parser.finishValueToken("Text", char);
		}
	},
});

function parseObject(parser: TOMLParser): JSONObject {
	const obj: JSONObject = {};

	while (!parser.matchToken("EOF")) {
		const prop = parseKeyValue(parser);
		if (prop !== undefined) {
			const [key, value] = prop;
			obj[key] = value;
		}
	}

	return obj;
}

function parseValue(parser: TOMLParser): undefined | TOMLValue {
	const valueToken = parser.getToken();

	switch (valueToken.type) {
		case "String": {
			parser.nextToken();
			return valueToken.value;
		}

		default: {
			parser.unexpectedDiagnostic();
			parser.nextToken();
			return undefined;
		}
	}
}

function parseKeyValue(parser: TOMLParser): [string, TOMLValue] | undefined {
	const key = parseKeyAndEquals(parser);
	if (key === undefined) {
		return undefined;
	}

	const value = parseValue(parser);
	if (value === undefined) {
		parser.unexpectedDiagnostic({
			description: descriptions.TOML_PARSER.VALUE_NOT_RECOGNISED(key),
		});
		parser.nextToken();
		return undefined;
	}

	return [key, value];
}

function parseKeyAndEquals(parser: TOMLParser): string | undefined {
	const token = parser.getToken();

	if (token.type === "Text") {
		const key = token.value;

		if (!allowedCharacterForKey(key)) {
			parser.unexpectedDiagnostic({
				token,
				description: descriptions.TOML_PARSER.INVALID_KEY_CHAR(key),
			});
			parser.nextToken();
			return undefined;
		}

		if (!parser.eatToken("Equals")) {
			parser.unexpectedDiagnostic({
				description: descriptions.TOML_PARSER.NO_VALUE_FOR_KEY(key),
			});
			parser.nextToken();
			return undefined;
		}

		parser.nextToken();
		return key;
	}

	// TODO
	parser.unexpected();
	parser.nextToken();
	return undefined;
}

export const toml: PartialConfigHandler = {
	type: "toml",
	language: "toml",
	extensions: ["toml", "ini"],
	jsonSuperset: false,

	parseExtra(opts: ParserOptions): ConfigParserResult {
		const parser = createTomlParser(opts);

		const root = parseObject(parser);

		parser.finalize();

		return {
			type: "toml",
			value: root,
			// TODO position tracking
			context: {
				category: "parse",
				categoryValue: "toml",
				normalizeKey: (key) => key,
				getDiagnosticLocation: () => ({}),
				getOriginalValue: () => undefined,
			},
			// TODO comments
			comments: new Map(),
		};
	},

	tokenize(opts: ConfigParserOptions): TokenBase[] {
		return createTomlParser(opts).getAllTokens();
	},

	stringifyFromConsumer(opts: PartialConsumeConfigResult): string {
		throw new Error("todo");
	},
};
