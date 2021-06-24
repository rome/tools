import {
	createParser,
	createReadCallback,
	isDigit,
	isntLineBreak,
} from "@internal/parser-core";
import {TOMLParser, TOMLParserTypes, Tokens} from "./types";
import {descriptions} from "@internal/diagnostics";
import {ZeroIndexed} from "@internal/numbers";
import {unescapeString} from "@internal/string-escape";

// Remove underscores from 'a string, this is used for numeric separators eg. 100_000
function removeUnderscores(
	parser: TOMLParser,
	index: ZeroIndexed,
	raw: string,
): string {
	let str = "";

	for (let i = 0; i < raw.length; i++) {
		const char = raw[i];

		if (char === "_") {
			if (i === 0) {
				throw parser.unexpected({
					description: descriptions.TOML.LEADING_NUMBER_UNDERSCORE,
					startIndex: index.add(i),
				});
			} else if (i === raw.length - 1) {
				throw parser.unexpected({
					description: descriptions.TOML.TRAILING_NUMBER_UNDERSCORE,
					startIndex: index.add(i),
				});
			} else if (raw[i + 1] === "_") {
				throw parser.unexpected({
					description: descriptions.TOML.DOUBLE_NUMBER_UNDERSCORE,
					startIndex: index.add(i),
				});
			}
		} else {
			str += char;
		}
	}

	return str;
}

// Used for Number token validation, allow underscore as a separator
function isNumberChar(char: string): boolean {
	return isDigit(char) || char === "_";
}

const isSingleStringValueChar = createReadCallback("'");
const isDoubleStringValueChar = createReadCallback('"');
const isMultilineSingleStringValueChar = createReadCallback("'''");
const isMultilineDoubleStringValueChar = createReadCallback('"""');

export function isValidWordKey(char: string) {
	return char !== undefined && /^[A-Za-z0-9_\-]+$/.test(char);
}

export const tomlParser = createParser<TOMLParserTypes>({
	diagnosticLanguage: "toml",
	ignoreWhitespaceTokens: true,
	getInitialState: () => ({
		path: [],
		explicitDefinedPaths: new Set(),
		pathComments: new Map(),
	}),
	tokenize(parser, tokenizer) {
		const char = tokenizer.get();

		switch (char) {
			case "#": {
				tokenizer.assert("#");
				const value = tokenizer.read(isntLineBreak);
				return tokenizer.finishValueToken("Comment", value);
			}

			case "'":
			case '"':
				return tokenizeString(parser, char, tokenizer);
		}

		if (tokenizer.consume("[")) {
			return tokenizer.finishToken("OpenSquareBracket");
		}

		if (tokenizer.consume("]")) {
			return tokenizer.finishToken("CloseSquareBracket");
		}

		if (tokenizer.consume(":")) {
			return tokenizer.finishToken("Colon");
		}

		if (tokenizer.consume("=")) {
			return tokenizer.finishToken("Equals");
		}

		if (tokenizer.consume(".")) {
			return tokenizer.finishToken("Dot");
		}

		if (tokenizer.consume("+")) {
			return tokenizer.finishToken("Plus");
		}

		if (tokenizer.consume("-")) {
			return tokenizer.finishToken("Minus");
		}

		if (tokenizer.consume("{")) {
			return tokenizer.finishToken("OpenCurlyBrace");
		}

		if (tokenizer.consume("}")) {
			return tokenizer.finishToken("CloseCurlyBrace");
		}

		if (tokenizer.consume(",")) {
			return tokenizer.finishToken("Comma");
		}

		if (isDigit(char)) {
			return tokenizeDigit(parser, tokenizer);
		}

		if (isValidWordKey(char)) {
			const value = tokenizer.read(isValidWordKey);
			return tokenizer.finishValueToken("Word", value);
		} else {
			tokenizer.take(1);

			// Invalid but we'll reverify it with allowedCharacterForKey later
			return tokenizer.finishValueToken("Word", char);
		}
	},
});

function tokenizeString(
	parser: TOMLParser,
	char: string,
	tokenizer: TOMLParser["tokenizer"],
) {
	tokenizer.consume(char);

	let isMultiline = tokenizer.consume(char.repeat(2));

	let valueCallback =
		char === '"' ? isDoubleStringValueChar : isSingleStringValueChar;
	if (isMultiline) {
		valueCallback =
			char === '"'
				? isMultilineDoubleStringValueChar
				: isMultilineSingleStringValueChar;

		// Trailing newline for opening delimeter is ignored
		tokenizer.eat("\n");
	}

	const valueStart = tokenizer.index;
	let value = tokenizer.read(valueCallback);

	if (tokenizer.isEOF()) {
		throw parser.unexpected({
			description: descriptions.TOML.UNCLOSED_STRING,
			start: tokenizer.getPosition(),
		});
	}

	if (char === '"') {
		value = unescapeString(
			value,
			{
				mode: isMultiline ? "toml-multiline" : "toml-singleline",
				unexpected(metadata, strIndex) {
					throw parser.unexpected({
						description: metadata,
						start: parser.getPositionFromIndex(valueStart.add(strIndex)),
					});
				},
			},
		);
	} else {
		// Single quotes are literal strings
	}

	if (isMultiline) {
		tokenizer.assert(char.repeat(3));
	} else {
		tokenizer.assert(char);
	}

	return tokenizer.finishValueToken("String", value);
}

function tokenizeDate(
	parser: TOMLParser,
	tokenizer: TOMLParser["tokenizer"],
	year: number,
): Tokens["Date"] | Tokens["DateTime"] {
	tokenizer.assert("-");

	// Get month
	const month = tokenizer.readAssertCount("month", 2, isDigit);
	tokenizer.assert("-");

	// Get day
	const day = tokenizer.readAssertCount("day", 2, isDigit);

	const date = tokenizer.finishComplexToken(
		"Date",
		{
			year,
			month: Number(month),
			day: Number(day),
		},
	);

	if (tokenizer.consume(" ") || tokenizer.consume("T")) {
		return tokenizeDateTime(parser, tokenizer, date);
	}

	return date;
}

function tokenizeDateTime(
	parser: TOMLParser,
	tokenizer: TOMLParser["tokenizer"],
	date: Tokens["Date"],
): Tokens["DateTime"] {
	const hoursStr = tokenizer.readAssertCount("hour", 2, isDigit);

	const {hours, minutes, seconds} = tokenizeTime(
		parser,
		tokenizer,
		Number(hoursStr),
	);
	let utc = false;
	let offset: Tokens["DateTime"]["offset"];

	if (tokenizer.consume("Z")) {
		utc = true;
	} else if (tokenizer.startsWith("-") || tokenizer.startsWith("+")) {
		const negativeOffset = tokenizer.take(1) === "-";

		const hoursOffset = tokenizer.readAssertCount("hour", 2, isDigit);
		tokenizer.assert(":");
		const minutesOffset = tokenizer.readAssertCount("minute", 2, isDigit);

		offset = {
			negative: negativeOffset,
			hours: Number(hoursOffset),
			minutes: Number(minutesOffset),
		};
	}

	return tokenizer.finishComplexToken(
		"DateTime",
		{
			year: date.year,
			month: date.month,
			day: date.day,
			hours,
			minutes,
			seconds,
			utc,
			offset,
		},
	);
}

function tokenizeTime(
	parser: TOMLParser,
	tokenizer: TOMLParser["tokenizer"],
	hours: number,
): Tokens["Time"] {
	tokenizer.assert(":");

	const minutes = tokenizer.readAssertCount("minute", 2, isDigit);
	tokenizer.assert(":");

	let seconds = tokenizer.readAssertCount("second", 2, isDigit);

	// Get fractional seconds
	if (tokenizer.consume(".")) {
		seconds += ".";

		let fractionalSeconds = tokenizer.read(isDigit);
		if (fractionalSeconds.length === 0) {
			// TODO custom error
			throw parser.unexpected({
				index: tokenizer.index,
			});
		} else {
			seconds += fractionalSeconds;
		}
	}

	return tokenizer.finishComplexToken(
		"Time",
		{
			hours,
			minutes: Number(minutes),
			seconds: Number(seconds),
		},
	);
}

function tokenizeDigit(parser: TOMLParser, tokenizer: TOMLParser["tokenizer"]) {
	const start = tokenizer.index;
	let raw = tokenizer.read(isNumberChar);
	let num = removeUnderscores(parser, start, raw);
	let isFloat = false;

	if (raw.length === 4 && num.length === 4 && tokenizer.startsWith("-")) {
		return tokenizeDate(parser, tokenizer, Number(num));
	}

	if (raw.length === 2 && num.length === 2 && tokenizer.startsWith(":")) {
		return tokenizeTime(parser, tokenizer, Number(num));
	}

	if (num === "0") {
		// TODO octal
		// TODO hex
		// TODO binary
	}

	if (num.length > 1 && num[0] === "0") {
		// TODO custom error: leading zero
		throw parser.unexpected({
			index: start,
		});
	}

	if (tokenizer.consume(".")) {
		isFloat = true;
		num += ".";

		const start = tokenizer.index;
		const raw = tokenizer.read(isNumberChar);
		if (raw.length === 0) {
			// TODO custom error
			throw parser.unexpected({
				index: tokenizer.index,
			});
		} else {
			num += removeUnderscores(parser, start, raw);
		}
	}

	if (tokenizer.consume("e") || tokenizer.consume("E")) {
		isFloat = true;
		num += "e";

		if (tokenizer.startsWith("+") || tokenizer.startsWith("-")) {
			num += tokenizer.take(1);
		}

		const start = tokenizer.index;
		const raw = tokenizer.read(isNumberChar);
		if (raw.length === 0) {
			// TODO specific error
			throw parser.unexpected({
				index: tokenizer.index,
			});
		} else {
			num += removeUnderscores(parser, start, raw);
		}
	}

	if (isFloat) {
		return tokenizer.finishValueToken("Float", num);
	} else {
		return tokenizer.finishValueToken("Int", num);
	}
}
