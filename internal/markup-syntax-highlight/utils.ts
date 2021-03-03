import {
	HighlightCodeResult,
	ReduceCallback,
	ReduceCallbackResult,
	TokenShape,
} from "./types";
import {
	MarkupTokenType,
	StaticMarkup,
	convertToMarkupFromRandomString,
	joinMarkup,
	markup,
	markupTag,
	readMarkup,
} from "@internal/markup";
import {splitLines} from "@internal/string-utils";
import {Markup} from "@internal/markup/escape";

export function reduce<Token extends TokenShape>(
	input: string,
	tokens: Token[],
	callback: ReduceCallback<Token>,
): HighlightCodeResult {
	let prevEnd = 0;
	let parts: Markup[] = [];

	for (let i = 0; i < tokens.length; i++) {
		const token = tokens[i];
		const start = token.start.valueOf();
		const end = token.end.valueOf();
		let value = input.slice(start, end);

		// Add on text between tokens
		parts.push(markup`${input.slice(prevEnd, start)}`);
		prevEnd = end;

		// Print this token
		// We need to break up the token text into lines, so that we can easily split the highlighted newlines and have the ansi codes be unbroken
		const lines = splitLines(value);
		for (let j = 0; j < lines.length; j++) {
			const line = lines[j];

			if (line !== "") {
				const prev = tokens[i - 1];
				const next = tokens[i + 1];
				const escapedLine = markup`${line}`;
				const res = callback(token, escapedLine, prev, next);
				if (res === undefined) {
					parts.push(escapedLine);
				} else {
					const {value = escapedLine, type} = res;
					if (type === undefined) {
						parts.push(value);
					} else {
						parts.push(markupToken(type, value));
					}
				}
			}

			// Last element isn't a line break
			const isLast = j === lines.length - 1;
			if (!isLast) {
				parts.push(markup`\n`);
			}
		}
	}

	return concatSplitLinesMarkup(parts);
}

export function markupToken(
	type: MarkupTokenType,
	value: StaticMarkup | string,
): StaticMarkup {
	return markupTag("token", markup`${value}`, {type});
}

export function concatSplitLinesMarkup(parts: Markup[]): Markup[] {
	return splitLines(readMarkup(joinMarkup(parts))).map((line) =>
		convertToMarkupFromRandomString(line)
	);
}

export function invalidHighlight(line: StaticMarkup): ReduceCallbackResult {
	return {
		value: markupTag("emphasis", markupTag("color", line, {bg: "red"})),
	};
}

export function reduceParserCore<Token extends TokenShape & {
	type: string;
}>(
	input: string,
	tokens: Token[],
	callback: ReduceCallback<Token>,
): HighlightCodeResult {
	return reduce(
		input,
		tokens,
		(token, value, prev, next) => {
			switch (token.type) {
				case "Invalid":
					return invalidHighlight(value);

				// Will never be hit
				case "EOF":
				case "SOF":
					return {value: markup``};

				default:
					// We should have refined `token` to not include any of the base tokens
					return callback(token, value, prev, next);
			}
		},
	);
}
