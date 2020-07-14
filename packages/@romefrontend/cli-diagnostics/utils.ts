/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import highlightCode, {AnsiHighlightOptions} from "./highlightCode";
import {NEWLINE, nonASCIIwhitespace} from "@romefrontend/js-parser-utils";
import {removeCarriageReturn} from "@romefrontend/string-utils";
import {
	joinMarkupLines,
	markupToPlainText,
} from "@romefrontend/cli-layout/format";

const unicodeControls = /[\u0000-\u001f\u007f-\u00a0]/u;

function isWhitespace(char: undefined | string): boolean {
	return char === " " || char === "\t" || char === "\r" || char === "\n";
}

export function showInvisibles(
	str: string,
	{atLineStart, atLineEnd}: {
		atLineStart: boolean;
		atLineEnd: boolean;
	},
): {
	value: string;
	hadNonWhitespace: boolean;
} {
	let hadNonWhitespace = false;
	let ret = "";

	// Get the first trailing whitespace character in the string
	let trailingWhitespaceIndex = str.length;
	while (isWhitespace(str[trailingWhitespaceIndex - 1])) {
		trailingWhitespaceIndex--;
	}

	for (let i = 0; i < str.length; i++) {
		const char = str[i];
		let showInvisible = true;

		// Only highlight spaces when surrounded by other spaces
		if (char === " ") {
			showInvisible = false;

			if (isWhitespace(str[i - 1]) || isWhitespace(str[i + 1])) {
				showInvisible = false;
			}
		}

		// Don't show leading tabs
		if (atLineStart && !hadNonWhitespace && char === "\t") {
			showInvisible = false;
		}

		// Always show if at the end of line
		if (atLineEnd && i >= trailingWhitespaceIndex) {
			showInvisible = true;
		}

		if (!showInvisible) {
			if (!isWhitespace(char)) {
				hadNonWhitespace = true;
			}
			ret += char;
			continue;
		}

		const visible = showInvisibleChar(char);
		if (visible !== undefined) {
			ret += visible;
			continue;
		}

		if (nonASCIIwhitespace.test(char) || unicodeControls.test(char)) {
			ret += showUnicodeChar(char);
			continue;
		}

		ret += char;
	}

	return {
		hadNonWhitespace,
		value: ret,
	};
}

function showUnicodeChar(char: string): string {
	// We use inverse to make it clear that it's not in the source
	return `<inverse>U+${char.codePointAt(0)!.toString(16)}</inverse>`;
}

function showInvisibleChar(char: string): undefined | string {
	switch (char) {
		case " ":
			return "\xb7"; // Middle Dot

		case "\r":
			return "\u240d"; // Carriage Return Symbol

		case "\n":
			return "\u23ce"; // Return Symbol

		case "\t":
			return "\u21b9"; // Left Arrow To Bar Over Right Arrow To Bar

		case "\0":
			return "\u2400"; // Null Symbol

		case "\x0b":
			return "\u240b"; // Vertical Tabulation Symbol

		case "\b":
			return "\u232b"; // Backspace Symbol

		case "\f":
			return "\u21a1"; // Downards Two Headed Arrow

		// These are display characters we use above. Remove the ambiguity by escaping them
		case "\u240d":
		case "\u23ce":
		case "\u21b9":
		case "\u2400":
		case "\u240b":
		case "\u232b":
		case "\u21a1":
			return showUnicodeChar(char);

		default:
			return undefined;
	}
}

export function cleanEquivalentString(str: string): string {
	str = joinMarkupLines(markupToPlainText(str));

	// Replace all whitespace with spaces
	str = str.replace(/[\s\n]+/g, " ");

	// Remove trailing dot
	str = str.replace(/\.+$/, "");

	// Remove surrounding quotes
	str = str.replace(/^"(.*?)"$/, "$1");

	return str;
}

export function splitLines(src: string): Array<string> {
	return src.split(NEWLINE);
}

export type ToLines = {
	length: number;
	raw: Array<string>;
	highlighted: Array<string>;
};

export function toLines(opts: AnsiHighlightOptions): ToLines {
	const input = removeCarriageReturn(opts.input);
	const raw = splitLines(input);
	const highlighted = splitLines(highlightCode({...opts, input}));

	if (raw.length !== highlighted.length) {
		throw new Error(
			`${opts.path.join()}: raw and highlighted line count mismatch ${raw.length} !== ${highlighted.length}`,
		);
	}

	return {
		length: raw.length,
		raw,
		highlighted,
	};
}
