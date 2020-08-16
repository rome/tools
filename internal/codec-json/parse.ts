/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	DiagnosticCategory,
	DiagnosticLocation,
	descriptions,
} from "@internal/diagnostics";
import {
	Comments,
	JSONObject,
	JSONParserOptions,
	JSONParserResult,
	JSONValue,
	PathComments,
	RJSONCommentMap,
	Tokens,
} from "./types";
import {
	ConsumeContext,
	ConsumePath,
	ConsumeSourceLocationRequestTarget,
} from "@internal/consume";
import {unescapeJSONString} from "@internal/string-escape";
import {
	ParserCore,
	ParserOptions,
	Position,
	SourceLocation,
	createParser,
	isAlpha,
	isDigit,
} from "@internal/parser-core";
import {Number0, ob1Add, ob1Get0, ob1Inc, ob1Sub} from "@internal/ob1";
import {isEscaped} from "@internal/string-utils";

// Words can't start with a digit
function isWordStartChar(char: string): boolean {
	return isAlpha(char) || char === "_" || char === "$";
}

// But a digit can appear inside of a word
function isWordChar(char: string): boolean {
	return isWordStartChar(char) || isDigit(char);
}

// Check if an input string is a valid word, this is used by the stringifier to
// determine if a property key should be quoted
export function isValidWord(word: string): boolean {
	if (word.length === 0 || !isWordStartChar(word[0])) {
		return false;
	}

	for (const char of word) {
		if (!isWordChar(char)) {
			return false;
		}
	}

	return true;
}

// Check if a character is a part of a string, returning false for a newline or unescaped quote char
function isJSONStringValueChar(
	char: string,
	index: Number0,
	input: string,
): boolean {
	if (char === "\n") {
		return false;
	}

	return isRJSONStringValueChar(char, index, input);
}

// NOTE: Different methods as we allow newlines in RJSON strings
function isRJSONStringValueChar(
	char: string,
	index: Number0,
	input: string,
): boolean {
	return !(char === '"' && !isEscaped(index, input));
}

// Turn a path into a string key we can use
export function toPathKey(parts: Array<string>) {
	// Right now this could conflict weirdly with properties with dots in them if they cause collisions
	// We have this method abstracted so we can make changes later if it's necessary (probably not worth it)
	return parts.join(".");
}

function isntNewline(char: string): boolean {
	return char !== "\n";
}

function isntBlockCommentEnd(
	char: string,
	index: Number0,
	input: string,
): boolean {
	const nextChar = input[ob1Get0(index) + 1];
	return char !== "*" && nextChar !== "/";
}

// Used for Number token validation, allow underscore as a separatore
function isNumberChar(char: string): boolean {
	return isDigit(char) || char === "_";
}

type PathInfo = {
	originalValue: unknown;
	keyStart: Position;
	keyEnd: Position;
	valueStart: Position;
	valueEnd: Position;
};

type State = {
	pathToComments: RJSONCommentMap;
	hasExtensions: boolean;
	pathKeys: ConsumePath;
	paths: Map<string, PathInfo>;
};

type JSONParserTypes = {
	tokens: Tokens;
	state: State;
	options: JSONParserOptions;
	meta: void;
};
type JSONParser = ParserCore<JSONParserTypes>;

export const createJSONParser = createParser<JSONParserTypes>({
	diagnosticCategory: "parse/json",
	ignoreWhitespaceTokens: true,
	retainCarriageReturn: true,
	getInitialState(parser) {
		return {
			hasExtensions: parser.path !== undefined &&
			parser.path.getBasename().endsWith(".rjson"),
			pathKeys: [],
			paths: new Map(),
			pathToComments: new Map(),
		};
	},
	tokenize(parser: JSONParser, index: Number0) {
		const char = parser.getInputCharOnly(index);
		const nextChar = parser.getInputCharOnly(index, 1);

		// Line comment
		if (char === "/" && nextChar === "/") {
			const commentValueIndex = ob1Add(index, 2);
			const [value] = parser.readInputFrom(commentValueIndex, isntNewline);
			// (comment content start + comment content length)
			return parser.finishValueToken(
				"LineComment",
				value,
				ob1Add(commentValueIndex, value.length),
			);
		}

		// BlockComment
		if (char === "/" && nextChar === "*") {
			const commentValueIndex = ob1Add(index, 2);
			const [value] = parser.readInputFrom(
				commentValueIndex,
				isntBlockCommentEnd,
			);

			// (comment content start + comment content length + 2 characters for comment end)
			const endIndex = ob1Add(ob1Add(commentValueIndex, value.length), 2);

			// Ensure the comment is closed
			if (
				parser.input[ob1Get0(endIndex) - 2] !== "*" ||
				parser.input[ob1Get0(endIndex) - 1] !== "/"
			) {
				throw parser.unexpected({
					description: descriptions.JSON.UNCLOSED_BLOCK_COMMENT,
					start: parser.getPositionFromIndex(endIndex),
				});
			}

			return parser.finishValueToken("BlockComment", value, endIndex);
		}

		// Single character token starters
		switch (char) {
			case '"': {
				const [value, end, overflow] = parser.readInputFrom(
					ob1Inc(index),
					parser.state.hasExtensions
						? isRJSONStringValueChar
						: isJSONStringValueChar,
				);

				if (overflow) {
					throw parser.unexpected({
						description: descriptions.JSON.UNCLOSED_STRING,
						start: parser.getPositionFromIndex(end),
					});
				}

				// Don't allow newlines in JSON
				if (!parser.state.hasExtensions) {
					for (let strIndex = 0; strIndex < value.length; strIndex++) {
						const char = value[strIndex];

						if (char === "\n") {
							throw parser.unexpected({
								description: descriptions.JSON.STRING_NEWLINES_IN_JSON,
								start: parser.getPositionFromIndex(ob1Add(index, strIndex)),
							});
						}
					}
				}

				// Unescape the string
				const unescaped = unescapeJSONString(
					value,
					(metadata, strIndex) => {
						throw parser.unexpected({
							description: metadata,
							start: parser.getPositionFromIndex(ob1Add(index, strIndex)),
						});
					},
					parser.state.hasExtensions,
				);

				// increment to take the trailing quote
				return parser.finishValueToken("String", unescaped, ob1Inc(end));
			}

			case "'":
				throw parser.unexpected({
					description: descriptions.JSON.SINGLE_QUOTE_USAGE,
					start: parser.getPositionFromIndex(index),
				});

			case "/":
				throw parser.unexpected({
					description: descriptions.JSON.REGEX_IN_JSON,
					start: parser.getPositionFromIndex(index),
				});

			case ",":
				return parser.finishToken("Comma");

			case ".":
				return parser.finishToken("Dot");

			case "-":
				return parser.finishToken("Minus");

			case "+":
				return parser.finishToken("Plus");

			case ":":
				return parser.finishToken("Colon");

			case "{":
				return parser.finishToken("BraceOpen");

			case "}":
				return parser.finishToken("BraceClose");

			case "[":
				return parser.finishToken("BracketOpen");

			case "]":
				return parser.finishToken("BracketClose");
		}

		// Numbers
		if (isDigit(char)) {
			const value = removeUnderscores(
				parser,
				index,
				parser.readInputFrom(index, isNumberChar)[0],
			);
			const num = Number(value);
			return parser.finishValueToken("Number", num, ob1Add(index, value.length));
		}

		// Word - boolean, undefined etc
		if (isWordStartChar(char)) {
			const [value] = parser.readInputFrom(index, isWordChar);
			return parser.finishValueToken("Word", value, ob1Add(index, value.length));
		}

		// Unknown character
		return undefined;
	},
});

function getPathInfo(
	parser: JSONParser,
	path: ConsumePath,
): undefined | PathInfo {
	return parser.state.paths.get(path.join("."));
}

function setComments(parser: JSONParser, pathComments: PathComments) {
	const key = parser.state.pathKeys.join(".");

	const existing = parser.state.pathToComments.get(key);
	if (existing === undefined) {
		parser.state.pathToComments.set(key, pathComments);
	} else {
		parser.state.pathToComments.set(
			key,
			{
				inner: [...existing.inner, ...pathComments.inner],
				outer: [...existing.outer, ...pathComments.outer],
			},
		);
	}
}

function setPath({state}: JSONParser, info: PathInfo) {
	state.paths.set(state.pathKeys.join("."), info);
	state.pathKeys.pop();
}

function parseObject(
	parser: JSONParser,
	firstKeyStart?: Position,
	firstKey?: string,
): JSONObject {
	const obj: JSONObject = {};

	let innerComments: Comments = [];
	let isFirstProp = true;

	// These are comments that the next property should take in case the previous accidently took them
	let nextLeadingComments;

	do {
		if (parser.matchToken("BraceClose")) {
			break;
		}

		// Eat all the comments that appeared before this property, it's the most common and natural place to put them,
		// and is where we'll print all comments for a property.
		let leadingComments = eatComments(parser);

		// Take any leading comments that were left by the previous property
		if (nextLeadingComments !== undefined) {
			leadingComments = [...nextLeadingComments, ...leadingComments];
			nextLeadingComments = undefined;
		}

		// Throw a meaningful error for redundant commas
		if (parser.matchToken("Comma")) {
			throw parser.unexpected({
				description: descriptions.JSON.REDUNDANT_COMMA,
			});
		}

		// If there's no property key indicator then delegate any comments we have to object
		const hasKey = isFirstProp && firstKey !== undefined;
		if (!hasKey && !parser.matchToken("String") && !parser.matchToken("Word")) {
			innerComments = [...innerComments, ...leadingComments];
			break;
		}

		const keyStart =
			isFirstProp && firstKeyStart !== undefined
				? firstKeyStart
				: parser.getPosition();

		// Parse the property key
		let key;
		if (isFirstProp && firstKey !== undefined) {
			// If this is the first property and we've been given a property key then use it instead
			key = firstKey;
		} else {
			key = parsePropertyKey(parser);
		}
		isFirstProp = false;

		const keyEnd = parser.getPosition();
		parser.expectToken("Colon");

		// Having comments before the value is a really weird place to put them, but we'll handle it
		// anyway to avoid throwing a parser error. When stringified, the comments will all be before
		// the property.
		const leadingValueComments = eatComments(parser);

		parser.state.pathKeys.push(key);

		// Parse the value.
		const valueStart = parser.getPosition();
		const value = parseExpression(parser);
		const valueEnd = parser.getLastEndPosition();

		// Eat the comments after the expression and associate the comments with them
		let trailingValueComments = eatComments(parser);

		// If the next token isn't a comma or closing brace then we've just stolen
		// the leading comments of the next property
		if (!parser.matchToken("Comma") && !parser.matchToken("BraceClose")) {
			nextLeadingComments = trailingValueComments;
			trailingValueComments = [];
		}

		setComments(
			parser,
			{
				inner: [],
				outer: [
					...leadingComments,
					...leadingValueComments,
					...trailingValueComments,
				],
			},
		);

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

	// Take any loose leading comments
	if (nextLeadingComments !== undefined) {
		innerComments = [...innerComments, ...nextLeadingComments];
	}

	// If we were passed a first key then this was an implicit object so there's no end token
	if (firstKey === undefined) {
		parser.expectToken("BraceClose");
	}

	setComments(
		parser,
		{
			inner: innerComments,
			outer: [],
		},
	);

	return obj;
}

// Remove underscores from 'a string, this is used for numeric separators eg. 100_000
function removeUnderscores(
	parser: JSONParser,
	index: Number0,
	raw: string,
): string {
	let str = "";

	for (let i = 0; i < raw.length; i++) {
		const char = raw[i];

		if (char === "_") {
			// Don't allow separators in JSON
			if (!parser.state.hasExtensions) {
				throw parser.unexpected({
					description: descriptions.JSON.NUMERIC_SEPARATORS_IN_JSON,
					start: parser.getPositionFromIndex(ob1Inc(index)),
				});
			}
		} else {
			str += char;
		}
	}

	return str;
}

function eatComments(parser: JSONParser): Comments {
	const comments: Comments = [];

	while (true) {
		const token = parser.getToken();

		if (token.type === "LineComment") {
			comments.push({
				type: "LineComment",
				value: token.value,
			});
		} else if (token.type === "BlockComment") {
			comments.push({
				type: "BlockComment",
				value: token.value,
			});
		} else {
			break;
		}

		// Comments aren't allowed in regular JSON
		if (!parser.state.hasExtensions) {
			throw parser.unexpected({
				description: descriptions.JSON.COMMENTS_IN_JSON,
			});
		}

		parser.nextToken();
	}

	return comments;
}

function parseArray(parser: JSONParser): Array<JSONValue> {
	parser.expectToken("BracketOpen");

	const arr = [];
	let innerComments: Comments = [];
	let i = 0;

	do {
		if (parser.matchToken("BracketClose")) {
			break;
		}

		// Eat all the comments before an element
		const leadingComments = eatComments(parser);

		if (parser.matchToken("Comma")) {
			throw parser.unexpected({
				description: descriptions.JSON.REDUNDANT_COMMA,
			});
		}

		// If we're at the end of the array then associate these comments with the array
		if (parser.matchToken("BracketClose")) {
			innerComments = [...innerComments, ...leadingComments];
			break;
		}

		const start = parser.getPosition();
		parser.state.pathKeys.push(i);
		i++;

		// Parse the value
		const item = parseExpression(parser);
		arr.push(item);
		const end = parser.getLastEndPosition();

		// Trailing comments are really weird, but let's handle them just like object properties
		const trailingComments = eatComments(parser);

		setComments(
			parser,
			{
				outer: [...leadingComments, ...trailingComments],
				inner: [],
			},
		);

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

	setComments(
		parser,
		{
			inner: innerComments,
			outer: [],
		},
	);

	return arr;
}

// Check if the current token is a property separator and eat it if necessary
function eatPropertySeparator(parser: JSONParser): boolean {
	const token = parser.getToken();

	// Implicit commas are only allowed in rjson
	if (parser.state.hasExtensions) {
		// Eat the token, don't care if we're in RJSON
		if (token.type === "Comma") {
			parser.nextToken();
		}

		// An object or array close is an instant failure
		// Doesn't matter what we're parsing since the subsequent tokens will be validated
		if (token.type === "BraceClose" || token.type === "BracketClose") {
			return false;
		}

		return true;
	} else {
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
}

function parseWord(parser: JSONParser, isStart: boolean): JSONValue {
	const start = parser.getPosition();
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
		if (parser.state.hasExtensions) {
			return parseObject(parser, start, token.value);
		} else {
			throw parser.unexpected({
				description: descriptions.JSON.IMPLICIT_OBJECT_IN_JSON,
			});
		}
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
			if (parser.state.hasExtensions) {
				parser.nextToken();
				return token.value;
			} else {
				throw parser.unexpected({
					description: descriptions.JSON.PROPERTY_KEY_UNQUOTED_IN_JSON,
				});
			}

		default:
			throw parser.unexpected();
	}
}

function parseString(parser: JSONParser, isStart: boolean): string | JSONObject {
	const start = parser.getPosition();
	const token = parser.expectToken("String");

	if (isStart && parser.nextToken().type === "Colon") {
		if (parser.state.hasExtensions) {
			return parseObject(parser, start, token.value);
		} else {
			throw parser.unexpected({
				description: descriptions.JSON.IMPLICIT_OBJECT_IN_JSON,
			});
		}
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
		if (parser.state.hasExtensions) {
			// If we're in RJSON mode then an empty input is an implicit object
			return {};
		} else {
			throw parser.unexpected({
				description: descriptions.JSON.EMPTY_INPUT_IN_JSON,
			});
		}
	} else {
		return parseExpression(parser, true);
	}
}

export function parseJSONExtra(opts: ParserOptions): JSONParserResult {
	const parser = createJSONParser(opts);
	const consumeDiagnosticCategory: DiagnosticCategory =
		parser.options.consumeDiagnosticCategory ?? "parse/json";

	let expectSyntaxError = false;

	if (!parser.state.hasExtensions) {
		// If we're in regular JSON, try the native JSON.parse
		try {
			const value = JSON.parse(parser.input);

			// Lazy parse when we need location information
			let context: undefined | Required<ConsumeContext>;
			function getContext(): Required<ConsumeContext> {
				if (context === undefined) {
					const res = _parse(parser, consumeDiagnosticCategory);
					context = res.context;
					return res.context;
				} else {
					return context;
				}
			}

			return {
				hasExtensions: false,
				path: parser.path,
				comments: new Map(),
				context: {
					category: consumeDiagnosticCategory,
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

	const res: JSONParserResult = _parse(parser, consumeDiagnosticCategory);

	if (expectSyntaxError) {
		throw new Error(
			"JSON.parse failed but our custom JSON parser was successful... That doesn't smell right",
		);
	}

	return res;
}

function _parse(
	parser: JSONParser,
	category: DiagnosticCategory,
): JSONParserResult {
	const leadingComments = eatComments(parser);

	const expr = parseEntry(parser);

	const trailingComments = eatComments(parser);
	setComments(
		parser,
		{
			inner: [],
			outer: [...leadingComments, ...trailingComments],
		},
	);

	parser.finalize();

	const context: Required<ConsumeContext> = {
		category,
		normalizeKey: (key) => key,
		getDiagnosticLocation: (
			keys: ConsumePath,
			target: ConsumeSourceLocationRequestTarget,
		): DiagnosticLocation => {
			const info = getPathInfo(parser, keys);
			if (info === undefined) {
				return {
					language: "json",
					filename: parser.filename,
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
				filename: parser.filename,
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
							column: ob1Add(loc.start.column, 1),
						},
						end: {
							...loc.end,
							column: ob1Sub(loc.end.column, 1),
						},
					};
				}
			}

			return {
				language: "json",
				...loc,
				mtime: parser.mtime,
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
		path: parser.path,
		hasExtensions: parser.state.hasExtensions,
		comments: parser.state.pathToComments,
		value: expr,
		context,
	};
}
