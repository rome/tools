import {
	TOMLArray,
	TOMLKey,
	TOMLKeys,
	TOMLObject,
	TOMLParser,
	TOMLValue,
	Tokens,
} from "./types";
import {Comments, PathComments} from "../types";
import {descriptions} from "@internal/diagnostics";
import {isPlainObject} from "@internal/typescript-helpers";
import {isValidWordKey} from "./tokenizer";
import {serializeConsumePath} from "@internal/consume";
import {ZeroIndexed} from "@internal/numbers";

function parseComments(parser: TOMLParser): Comments {
	const comments: Comments = [];

	while (parser.matchToken("Comment")) {
		comments.push({
			type: "LineComment",
			value: parser.expectToken("Comment").value,
		});
	}

	return comments;
}

function serializeKeys(keys: TOMLKeys): string {
	return serializeConsumePath(keys.map(({key}) => key));
}

function setComments(
	parser: TOMLParser,
	table: Table,
	inlineKeys: TOMLKeys,
	comments: PathComments,
): void {
	parser.state.pathComments.set(
		serializeKeys([...table.keys, ...inlineKeys]),
		comments,
	);
}

type Table = {
	object: TOMLObject;
	keys: TOMLKeys;
};

function parseTableHeader(parser: TOMLParser, root: Table): Table {
	parser.expectToken("OpenSquareBracket");

	let isArrayElement = false;

	if (parser.matchToken("OpenSquareBracket")) {
		isArrayElement = true;
		parser.assertNoSpace();
		parser.expectToken("OpenSquareBracket");
	}

	const start = parser.getPosition();
	const keys = parseKeys(parser);
	const end = parser.getPosition();

	const path = getPath(parser, root.object, keys, isArrayElement);

	let serialKey;
	if (!isArrayElement) {
		serialKey = serializeKeys(path.keys);
		if (parser.state.explicitDefinedPaths.has(serialKey)) {
			throw parser.unexpected({
				description: descriptions.TOML.DUPLICATE_DECLARATION,
				start,
				end,
			});
		}
	}

	parser.expectToken("CloseSquareBracket");

	if (isArrayElement) {
		parser.assertNoSpace();
		parser.expectToken("CloseSquareBracket");
	}

	if (serialKey !== undefined) {
		parser.state.explicitDefinedPaths.add(serialKey);
	}

	return path;
}

export function parseRoot(parser: TOMLParser): TOMLObject {
	const root: Table = {
		object: {},
		keys: [],
	};
	let table: Table = root;

	while (true) {
		let propComments = parseComments(parser);

		while (parser.matchToken("OpenSquareBracket")) {
			table = parseTableHeader(parser, root);

			if (!(parser.matchToken("Comment") || parser.matchToken("EOF"))) {
				parser.assertNewline();
			}

			if (propComments.length > 0) {
				setComments(
					parser,
					table,
					[],
					{
						inner: [],
						outer: propComments,
					},
				);
			}

			propComments = parseComments(parser);
		}

		if (parser.matchToken("EOF")) {
			break;
		}

		const startKeyToken = parser.getToken();
		const keys = parseKeys(parser);

		if (propComments.length > 0) {
			setComments(
				parser,
				table,
				keys,
				{
					inner: [],
					outer: propComments,
				},
			);
		}

		parser.assertNoNewline(startKeyToken);

		const equalsToken = parser.getToken();
		if (!parser.eatToken("Equals")) {
			throw parser.unexpected({
				description: descriptions.TOML.NO_VALUE_FOR_KEY(keys.join(".")),
			});
		}

		parser.assertNoNewline(equalsToken);
		const value = parseValue(parser, table, keys);
		setObjectValue(parser, table, table.object, keys, value);

		if (!(parser.matchToken("Comment") || parser.matchToken("EOF"))) {
			parser.assertNewline();
		}
	}

	return root.object;
}

function getPath(
	parser: TOMLParser,
	obj: TOMLObject,
	keys: TOMLKeys,
	isArrayElement: boolean,
): Table {
	if (keys.length === 0) {
		return {
			object: obj,
			keys,
		};
	}

	const finalKeys: TOMLKeys = [];
	let target: TOMLObject = obj;

	for (let i = 0; i < keys.length; i++) {
		const isLast = i === keys.length - 1;
		const elem = keys[i];

		const {key, start, end} = elem;
		const value = target[key];

		finalKeys.push(elem);

		if (value === undefined) {
			const obj: TOMLObject = {};
			if (isArrayElement && isLast) {
				target[key] = [obj];
			} else {
				target[key] = obj;
			}
			target = obj;
		} else if (isPlainObject(value)) {
			if (isArrayElement && isLast) {
				throw parser.unexpected({
					description: descriptions.TOML.BAD_ARRAY_TYPE,
					startIndex: start,
					endIndex: end,
				});
			} else {
				target = value;
			}
		} else if (Array.isArray(value)) {
			if (isArrayElement && isLast) {
				finalKeys.push({
					key: String(value.length),
					start,
					end,
				});

				const obj: TOMLObject = {};
				value.push(obj);
				target = obj;
			} else {
				const index = value.length - 1;
				const last = value[index];

				finalKeys.push({
					key: String(index),
					start,
					end,
				});

				if (isPlainObject(last)) {
					target = last;
				} else {
					throw parser.unexpected({
						description: descriptions.TOML.BAD_ARRAY_TYPE,
						startIndex: start,
						endIndex: end,
					});
				}
			}
		} else {
			throw parser.unexpected({
				description: descriptions.TOML.BAD_TABLE_TYPE,
				startIndex: start,
				endIndex: end,
			});
		}
	}

	return {
		keys: finalKeys,
		object: target,
	};
}

function setObjectValue(
	parser: TOMLParser,
	table: Table,
	obj: TOMLObject,
	keys: TOMLKeys,
	value: TOMLValue,
) {
	const prop = keys[keys.length - 1];
	const {key, start, end} = prop;

	const path = getPath(parser, obj, keys.slice(0, -1), false);

	const serialKey = serializeKeys([...table.keys, ...path.keys, prop]);

	if (parser.state.explicitDefinedPaths.has(serialKey)) {
		throw parser.unexpected({
			description: descriptions.TOML.DUPLICATE_DECLARATION,
			startIndex: start,
			endIndex: end,
		});
	} else {
		parser.state.explicitDefinedPaths.add(serialKey);
	}

	path.object[key] = value;
}

function parseInlineTable(
	parser: TOMLParser,
	table: Table,
	inlineKeys: TOMLKeys,
): TOMLObject {
	const openToken = parser.expectToken("OpenCurlyBrace");
	const obj: TOMLObject = {};

	let trailingComma: undefined | Tokens["Comma"];

	const subtable: Table = {
		object: obj,
		keys: [...table.keys, ...inlineKeys],
	};

	while (true) {
		const propComments = parseComments(parser);

		if (parser.matchToken("EOF") || parser.matchToken("CloseCurlyBrace")) {
			if (propComments.length > 0) {
				setComments(
					parser,
					subtable,
					inlineKeys,
					{
						inner: propComments,
						outer: [],
					},
				);
			}
			break;
		}

		const keys = parseKeys(parser);

		if (propComments.length > 0) {
			setComments(
				parser,
				subtable,
				keys,
				{
					inner: [],
					outer: propComments,
				},
			);
		}

		parser.expectToken("Equals");

		const value = parseValue(parser, subtable, keys);

		setObjectValue(parser, subtable, obj, keys, value);

		trailingComma = parser.eatToken("Comma");
	}

	if (trailingComma !== undefined) {
		throw parser.unexpected({
			description: descriptions.TOML.TRAILING_INLINE_TABLE_COMMA,
			token: trailingComma,
		});
	}

	parser.assertNoNewline(openToken);
	parser.expectToken("CloseCurlyBrace");

	return obj;
}

function isArrayElementSeparator(parser: TOMLParser): boolean {
	if (parser.eatToken("Comma")) {
		return true;
	}

	if (parser.matchToken("CloseSquareBracket")) {
		return true;
	}

	return false;
}

function parseArray(
	parser: TOMLParser,
	table: Table,
	inlineKeys: TOMLKeys,
): TOMLArray {
	const arr: TOMLArray = [];

	while (true) {
		const propKeys: TOMLKeys = [...inlineKeys, {key: String(arr.length)}];
		const propComments = parseComments(parser);

		if (parser.matchToken("EOF") || parser.matchToken("CloseSquareBracket")) {
			if (propComments.length > 0) {
				setComments(
					parser,
					table,
					inlineKeys,
					{
						inner: propComments,
						outer: [],
					},
				);
			}
			break;
		}

		if (propComments.length > 0) {
			setComments(
				parser,
				table,
				propKeys,
				{
					inner: [],
					outer: propComments,
				},
			);
		}

		const value = parseValue(parser, table, propKeys);
		arr.push(value);

		if (!isArrayElementSeparator(parser)) {
			throw parser.unexpected({
				description: descriptions.TOML.UNKNOWN_ARRAY_SEPARATOR,
			});
		}
	}

	parser.expectToken("CloseSquareBracket");

	return arr;
}

function parseNumberWord(parser: TOMLParser, token: Tokens["Word"]): number {
	switch (token.value) {
		case "nan": {
			parser.nextToken();
			return NaN;
		}

		case "inf": {
			parser.nextToken();
			return Infinity;
		}

		default:
			throw parser.unexpected({
				description: descriptions.TOML.UNKNOWN_WORD,
			});
	}
}

function parseWord(parser: TOMLParser, token: Tokens["Word"]): TOMLValue {
	switch (token.value) {
		case "true": {
			parser.nextToken();
			return true;
		}

		case "false": {
			parser.nextToken();
			return false;
		}

		default:
			return parseNumberWord(parser, token);
	}
}

function parseNumber(parser: TOMLParser): number {
	const token = parser.getToken();

	switch (token.type) {
		case "Float":
		case "Int": {
			parser.nextToken();
			return Number(token.value);
		}

		case "Plus":
			throw parser.unexpected({
				description: descriptions.TOML.EXCESSIVE_PLUS,
			});

		case "Minus":
			throw parser.unexpected({
				description: descriptions.TOML.EXCESSIVE_MINUS,
			});

		default:
			throw parser.unexpected();
	}
}

function parseDate(parser: TOMLParser, token: Tokens["Date"]): Date {
	const date = new Date();
	date.setFullYear(token.year, token.month - 1, token.day);
	date.setHours(0, 0, 0, 0);
	return date;
}

// TODO: Temporal?
function parseTime(parser: TOMLParser, token: Tokens["Time"]): TOMLObject {
	return {
		hours: token.hours,
		minutes: token.minutes,
		seconds: token.seconds,
	};
}

function padTime(num: number): string {
	if (num < 10) {
		return `0${String(num)}`;
	} else {
		return String(num);
	}
}

function parseDateTime(parser: TOMLParser, token: Tokens["DateTime"]): Date {
	let iso = [
		token.year,
		"-",
		padTime(token.month),
		"-",
		padTime(token.day),
		"T",
		padTime(token.hours),
		":",
		padTime(token.minutes),
		":",
		padTime(token.seconds),
	].join("");

	if (token.utc) {
		iso += "Z";
	} else if (token.offset !== undefined) {
		const {negative, hours, minutes} = token.offset;
		if (negative) {
			iso += "-";
		} else {
			iso += "+";
		}
		iso += `${padTime(hours)}:${padTime(minutes)}`;
	}

	return new Date(iso);
}

function parseValue(
	parser: TOMLParser,
	table: Table,
	inlineKeys: TOMLKeys,
): TOMLValue {
	const token = parser.getToken();

	switch (token.type) {
		case "String": {
			parser.nextToken();
			return token.value;
		}

		case "OpenSquareBracket": {
			parser.nextToken();
			return parseArray(parser, table, inlineKeys);
		}

		case "OpenCurlyBrace": {
			return parseInlineTable(parser, table, inlineKeys);
		}

		case "Word":
			return parseWord(parser, token);

		case "Minus": {
			parser.nextToken();
			return -parseNumber(parser);
		}

		case "Plus": {
			parser.nextToken();
			return +parseNumber(parser);
		}

		case "Int":
		case "Float":
			return parseNumber(parser);

		case "Date": {
			parser.nextToken();
			return parseDate(parser, token);
		}

		case "Time": {
			parser.nextToken();
			return parseTime(parser, token);
		}

		case "DateTime": {
			parser.nextToken();
			return parseDateTime(parser, token);
		}

		default: {
			throw parser.unexpected();
		}
	}
}

function parseKey(parser: TOMLParser, start: ZeroIndexed): TOMLKey {
	if (parser.matchToken("String")) {
		const token = parser.expectToken("String");
		return {
			key: token.value,
			start,
			end: token.end,
		};
	}

	const parts: string[] = [];

	while (true) {
		const token = parser.getToken();

		if (parts.length > 0) {
			parser.assertNoSpace();
		}

		switch (token.type) {
			case "String":
				throw parser.unexpected();

			case "Int": {
				parts.push(token.value);
				parser.nextToken();
				break;
			}

			case "Word": {
				const key = token.value;

				if (isValidWordKey(key)) {
					parser.nextToken();
					parts.push(key);
				} else {
					throw parser.unexpected({
						token,
						description: descriptions.TOML.INVALID_KEY_CHAR(key),
					});
				}
				break;
			}

			default:
				throw parser.unexpected();
		}

		if (!isAtValidKeyPart(parser)) {
			break;
		}
	}

	const end = parser.getIndex();
	return {
		key: parts.join(""),
		start,
		end,
	};
}

function isAtValidKeyPart(parser: TOMLParser): boolean {
	return (
		parser.matchToken("Word") ||
		parser.matchToken("String") ||
		parser.matchToken("Int")
	);
}

function parseKeys(parser: TOMLParser): TOMLKeys {
	let keys: TOMLKeys = [];
	let start = parser.getIndex();

	while (true) {
		keys.push(parseKey(parser, start));

		const trailingDot = parser.eatToken("Dot");

		if (!isAtValidKeyPart(parser)) {
			if (trailingDot !== undefined) {
				throw parser.unexpected({
					description: descriptions.TOML.TRAILING_KEY_DOT,
					token: trailingDot,
				});
			}
			break;
		}
	}

	return keys;
}
