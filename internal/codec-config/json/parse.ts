/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	DIAGNOSTIC_CATEGORIES,
	DiagnosticLocation,
	descriptions,
} from "@internal/diagnostics";
import {
	ConfigCommentMap,
	ConfigParserOptions,
	ConfigParserResult,
	ConfigType,
} from "../types";
import {JSONObject, JSONValue, Tokens} from "./types";
import {
	ConsumeContext,
	ConsumePath,
	ConsumeSourceLocationRequestTarget,
	serializeConsumePath,
} from "@internal/consume";
import {unescapeString} from "@internal/string-escape";
import {
	ParserCore,
	ParserOptions,
	Position,
	SourceLocation,
	createParser,
	isDigit,
} from "@internal/parser-core";
import {ZeroIndexed} from "@internal/numbers";
import {isEscaped} from "@internal/string-utils";
import {isWordChar, isWordStartChar} from "@internal/codec-config/util";

// Check if a character is a part of a string, returning false for a newline or unescaped quote char
function isJSONStringValueChar(
	char: string,
	index: ZeroIndexed,
	input: string,
): boolean {
	if (char === "\n") {
		return false;
	}

	return !(char === '"' && !isEscaped(index, input));
}

type PathInfo = {
	originalValue: unknown;
	keyStart: Position;
	keyEnd: Position;
	valueStart: Position;
	valueEnd: Position;
};

type State = {
	pathToComments: ConfigCommentMap;
	pathKeys: ConsumePath;
	paths: Map<string, PathInfo>;
};

type JSONParserTypes = {
	tokens: Tokens;
	state: State;
	options: ConfigParserOptions;
	meta: {
		type: ConfigType;
	};
};

type JSONParser = ParserCore<JSONParserTypes>;

export const jsonParser = createParser<JSONParserTypes>({
	diagnosticLanguage: "json",
	ignoreWhitespaceTokens: true,
	retainCarriageReturn: true,
	getInitialState() {
		return {
			pathKeys: [],
			paths: new Map(),
			pathToComments: new Map(),
		};
	},
	tokenize(parser, tokenizer) {
		if (tokenizer.consume('"')) {
			const valueStart = tokenizer.index;
			const value = tokenizer.read(isJSONStringValueChar);

			if (tokenizer.isEOF()) {
				throw parser.unexpected({
					description: descriptions.JSON.UNCLOSED_STRING,
					start: tokenizer.getPosition(),
				});
			}

			// Don't allow newlines in strings
			for (let strIndex = 0; strIndex < value.length; strIndex++) {
				const char = value[strIndex];

				if (char === "\n") {
					throw parser.unexpected({
						description: descriptions.JSON.STRING_NEWLINES_IN_JSON,
						start: parser.getPositionFromIndex(valueStart.add(strIndex)),
					});
				}
			}

			// Unescape the string
			const unescaped = unescapeString(
				value,
				{
					mode: "json",
					unexpected(metadata, strIndex) {
						throw parser.unexpected({
							description: metadata,
							start: parser.getPositionFromIndex(valueStart.add(strIndex)),
						});
					},
				},
			);

			tokenizer.assert('"');

			// increment to take the trailing quote
			return tokenizer.finishValueToken("String", unescaped);
		}

		if (tokenizer.startsWith("'")) {
			throw parser.unexpected({
				description: descriptions.JSON.SINGLE_QUOTE_USAGE,
				start: tokenizer.getPosition(),
			});
		}

		if (tokenizer.startsWith("//") || tokenizer.startsWith("/*")) {
			throw parser.unexpected({
				description: descriptions.JSON.COMMENTS_IN_JSON,
				start: tokenizer.getPosition(),
			});
		}

		if (tokenizer.startsWith("/")) {
			throw parser.unexpected({
				description: descriptions.JSON.REGEX_IN_JSON,
				start: tokenizer.getPosition(),
			});
		}

		if (tokenizer.consume(",")) {
			return tokenizer.finishToken("Comma");
		}

		if (tokenizer.consume(".")) {
			return tokenizer.finishToken("Dot");
		}

		if (tokenizer.consume("-")) {
			return tokenizer.finishToken("Minus");
		}

		if (tokenizer.consume("+")) {
			return tokenizer.finishToken("Plus");
		}

		if (tokenizer.consume(":")) {
			return tokenizer.finishToken("Colon");
		}

		if (tokenizer.consume("{")) {
			return tokenizer.finishToken("BraceOpen");
		}

		if (tokenizer.consume("}")) {
			return tokenizer.finishToken("BraceClose");
		}

		if (tokenizer.consume("[")) {
			return tokenizer.finishToken("BracketOpen");
		}

		if (tokenizer.consume("]")) {
			return tokenizer.finishToken("BracketClose");
		}

		// Numbers
		if (isDigit(tokenizer.get())) {
			const raw = tokenizer.read(isDigit);
			const num = Number(raw);
			return tokenizer.finishValueToken("Number", num);
		}

		// Word - boolean, undefined etc
		if (isWordStartChar(tokenizer.get())) {
			const value = tokenizer.read(isWordChar);
			return tokenizer.finishValueToken("Word", value);
		}

		// Unknown character
		return undefined;
	},
});

function getPathInfo(
	parser: JSONParser,
	path: ConsumePath,
): undefined | PathInfo {
	return parser.state.paths.get(serializeConsumePath(path));
}

function setPath({state}: JSONParser, info: PathInfo) {
	state.paths.set(serializeConsumePath(state.pathKeys), info);
	state.pathKeys.pop();
}

function parseObject(parser: JSONParser): JSONObject {
	const obj: JSONObject = {};

	do {
		if (parser.matchToken("BraceClose")) {
			break;
		}

		// Throw a meaningful error for redundant commas
		if (parser.matchToken("Comma")) {
			throw parser.unexpected({
				description: descriptions.JSON.REDUNDANT_COMMA,
			});
		}

		const keyStart = parser.getPosition();

		// Parse the property key
		const key = parsePropertyKey(parser);

		const keyEnd = parser.getPosition();
		parser.expectToken("Colon");

		parser.state.pathKeys.push(key);

		// Parse the value.
		const valueStart = parser.getPosition();
		const value = parseExpression(parser);
		const valueEnd = parser.getLastEndPosition();

		setPath(
			parser,
			{
				keyStart,
				keyEnd,
				valueStart,
				valueEnd,
				originalValue: value,
			},
		);

		// Set the object correctly, accounting for JS weirdness
		if (key === "__proto__") {
			// Need to use defineProperty to avoid triggering the Object.prototype.__proto__ setter
			Object.defineProperty(
				obj,
				"__proto__",
				{
					value,
					configurable: true,
					writable: true,
					enumerable: true,
				},
			);
		} else {
			obj[key] = value;
		}
	} while (eatPropertySeparator(parser));

	parser.expectToken("BraceClose");

	return obj;
}

function parseArray(parser: JSONParser): JSONValue[] {
	parser.expectToken("BracketOpen");

	const arr = [];
	let i = 0;

	do {
		if (parser.matchToken("BracketClose")) {
			break;
		}

		if (parser.matchToken("Comma")) {
			throw parser.unexpected({
				description: descriptions.JSON.REDUNDANT_COMMA,
			});
		}

		const start = parser.getPosition();
		parser.state.pathKeys.push(i);
		i++;

		// Parse the value
		const item = parseExpression(parser);
		arr.push(item);
		const end = parser.getLastEndPosition();

		setPath(
			parser,
			{
				originalValue: item,
				keyStart: start,
				keyEnd: end,
				valueStart: start,
				valueEnd: end,
			},
		);

		// Have a meaningful error message when an object is incorrectly using brackets: ["foo": "bar"]
		if (parser.matchToken("Colon")) {
			throw parser.unexpected({
				description: descriptions.JSON.MISTAKEN_ARRAY_IDENTITY,
			});
		}
	} while (eatPropertySeparator(parser));

	parser.expectToken("BracketClose");

	return arr;
}

// Check if the current token is a property separator and eat it if necessary
function eatPropertySeparator(parser: JSONParser): boolean {
	const token = parser.getToken();

	if (token.type !== "Comma") {
		return false;
	}

	// Make sure this isn't a trailing comma
	const lookahead = parser.lookaheadToken();
	if (lookahead.type === "BraceClose" || lookahead.type === "BracketClose") {
		throw parser.unexpected({
			description: descriptions.JSON.TRAILING_COMMA_IN_JSON,
		});
	}

	parser.nextToken();
	return true;
}

function parseWord(parser: JSONParser, isStart: boolean): JSONValue {
	const token = parser.expectToken("Word");

	switch (token.value) {
		case "true":
			return true;

		case "false":
			return false;

		case "null":
			return null;

		case "undefined":
			throw parser.unexpected({
				description: descriptions.JSON.UNDEFINED_IN_JSON,
			});
	}

	if (isStart && parser.matchToken("Colon")) {
		throw parser.unexpected({
			description: descriptions.JSON.IMPLICIT_OBJECT_IN_JSON,
		});
	}

	throw parser.unexpected({
		description: descriptions.JSON.UNKNOWN_WORD_IN_JSON(token.value),
	});
}

function parseNumber(parser: JSONParser): number {
	const isNegative = parser.eatToken("Minus") !== undefined;

	// Get a string of the current number that we'll parse later
	const token = parser.expectToken("Number");
	let value: string = String(token.value);

	// Decimals
	if (parser.eatToken("Dot")) {
		value += ".";

		const decimal = parser.expectToken("Number");
		value += String(decimal.value);
	}

	// Scientific notation
	const nextToken = parser.getToken();
	if (
		nextToken.type === "Word" &&
		(nextToken.value === "e" || nextToken.value === "E")
	) {
		value += "e";

		// Operator
		const operator = parser.nextToken();
		if (operator.type === "Minus") {
			value += "-";
		} else if (operator.type === "Plus") {
			value += "+";
		} else {
			throw parser.unexpected();
		}

		// Factor
		parser.nextToken();
		const factor = parser.expectToken("Number");
		value += String(factor.value);
	}

	// BigInt
	const nextToken2 = parser.getToken();
	if (nextToken2.type === "Word" && nextToken2.value === "n") {
		throw parser.unexpected({
			description: descriptions.JSON.BIGINT_IN_JSON,
		});
	}

	// Turn the string into an actual number
	let num = Number(value);
	if (isNegative) {
		num = -num;
	}
	return num;
}

function parsePropertyKey(parser: JSONParser) {
	const token = parser.getToken();

	switch (token.type) {
		case "String": {
			parser.nextToken();
			return token.value;
		}

		case "Word":
			throw parser.unexpected({
				description: descriptions.JSON.PROPERTY_KEY_UNQUOTED_IN_JSON,
			});

		default:
			throw parser.unexpected();
	}
}

function parseString(parser: JSONParser, isStart: boolean): string | JSONObject {
	const token = parser.expectToken("String");

	if (isStart && parser.nextToken().type === "Colon") {
		throw parser.unexpected({
			description: descriptions.JSON.IMPLICIT_OBJECT_IN_JSON,
		});
	} else {
		return token.value;
	}
}

function parseExpression(
	parser: JSONParser,
	isStart: boolean = false,
): JSONValue {
	const token = parser.getToken();

	switch (token.type) {
		case "String":
			return parseString(parser, isStart);

		case "Minus":
		case "Number":
			return parseNumber(parser);

		case "Word":
			return parseWord(parser, isStart);

		case "BracketOpen":
			return parseArray(parser);

		case "BraceOpen": {
			parser.nextToken();
			return parseObject(parser);
		}

		default:
			throw parser.unexpected();
	}
}

function parseEntry(parser: JSONParser): JSONValue {
	if (parser.matchToken("EOF")) {
		throw parser.unexpected({
			description: descriptions.JSON.EMPTY_INPUT_IN_JSON,
		});
	} else {
		return parseExpression(parser, true);
	}
}

export function parseJSONExtra(
	opts: ParserOptions,
	type: ConfigType,
): ConfigParserResult {
	const parser = jsonParser.create(
		opts,
		{
			type,
		},
		{diagnosticLanguage: type},
	);

	const categoryValue = parser.options.consumeDiagnosticCategoryValue ?? "json";

	let expectSyntaxError = false;

	if (parser.meta.type === "json") {
		// If we're in regular JSON, try the native JSON.parse
		try {
			const value = JSON.parse(parser.input);

			// Lazy parse when we need location information
			let context: undefined | Required<ConsumeContext>;
			function getContext(): Required<ConsumeContext> {
				if (context === undefined) {
					const res = _parse(parser, categoryValue);
					context = res.context;
					return res.context;
				} else {
					return context;
				}
			}

			return {
				type: "json",
				comments: new Map(),
				context: {
					category: DIAGNOSTIC_CATEGORIES.parse,
					categoryValue,
					normalizeKey(path) {
						return getContext().normalizeKey(path);
					},
					getOriginalValue(path) {
						return getContext().getOriginalValue(path);
					},
					getDiagnosticLocation(keys, target) {
						return getContext().getDiagnosticLocation(keys, target);
					},
				},
				value,
			};
		} catch (err) {
			// On syntax errors we'll fall back to our parser which is slower, but produces more meaningful errors
			if (err instanceof SyntaxError) {
				expectSyntaxError = true;
			} else {
				throw err;
			}
		}
	}

	const res: ConfigParserResult = _parse(parser, categoryValue);

	if (expectSyntaxError) {
		throw parser.unexpected({
			description: descriptions.JSON.UNEXPECTED_PARSING,
		});
	}

	return res;
}

function _parse(parser: JSONParser, categoryValue: string): ConfigParserResult {
	const expr = parseEntry(parser);

	parser.finalize();

	const context: Required<ConsumeContext> = {
		category: DIAGNOSTIC_CATEGORIES.parse,
		categoryValue,
		normalizeKey: (key) => key,
		getDiagnosticLocation: (
			keys: ConsumePath,
			target: ConsumeSourceLocationRequestTarget,
		): DiagnosticLocation => {
			const info = getPathInfo(parser, keys);
			if (info === undefined) {
				return {
					language: parser.language,
					path: parser.path,
				};
			}

			let start = info.keyStart;
			let end = info.valueEnd;

			if (target === "key") {
				end = info.keyEnd;
			}

			if (target === "value" || target === "inner-value") {
				start = info.valueStart;
			}

			let loc: SourceLocation = {
				path: parser.path,
				start,
				end,
			};

			if (target === "inner-value") {
				const originalValue = context.getOriginalValue(keys);

				// Remove quote marks for strings
				if (typeof originalValue === "string") {
					loc = {
						...loc,
						start: {
							...loc.start,
							column: loc.start.column.add(1),
						},
						end: {
							...loc.end,
							column: loc.end.column.subtract(1),
						},
					};
				}
			}

			return {
				language: parser.language,
				...loc,
				integrity: parser.integrity,
				sourceText: undefined,
			};
		},
		getOriginalValue: (keys: ConsumePath) => {
			const info = getPathInfo(parser, keys);
			if (info !== undefined) {
				return info.originalValue;
			}
		},
	};

	return {
		type: parser.meta.type,
		comments: parser.state.pathToComments,
		value: expr,
		context,
	};
}
