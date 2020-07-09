/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {markupToPlainTextString} from "@romefrontend/string-markup";
import highlightCode, {AnsiHighlightOptions} from "./highlightCode";
import {NEWLINE} from "@romefrontend/js-parser-utils";
import {removeCarriageReturn} from "@romefrontend/string-utils";

export function normalizeTabs(str: string): string {
	return str.replace(/\t/g, "  ");
}

function isWhitespace(char: undefined | string): boolean {
	return char === " " || char === "\t" || char === "\r" || char === "\n";
}

export function showInvisibles(str: string, isLineBeginning: boolean): string {
	let ret = "";
	let foundVisible = false;

	for (let i = 0; i < str.length; i++) {
		const char = str[i];
		let showInvisible = false;

		// Show if whitespace on either side
		if (isWhitespace(str[i - 1]) || isWhitespace(str[i + 1])) {
			showInvisible = true;
		}

		// Always show trailing and leading
		if (i === 0 || i === str.length - 1) {
			showInvisible = true;
		}

		// Don't show leading tabs
		if (isLineBeginning && char === "\t" && !foundVisible) {
			showInvisible = false;
		}

		if (!showInvisible) {
			if (!isWhitespace(char)) {
				foundVisible = true;
			}
			ret += char;
			continue;
		}

		switch (char) {
			case " ": {
				ret += "\xb7"; // Middle Dot, \u00B7
				break;
			}
			case "\r": {
				ret += "\u240d";
				break;
			}
			case "\n": {
				ret += "\u23ce"; // Return Symbol, \u23ce
				break;
			}
			case "\t": {
				ret += "\u21b9"; // Left Arrow To Bar Over Right Arrow To Bar, \u21b9
				break;
			}
			default: {
				ret += char;
				break;
			}
		}
	}

	return ret;
}

export function cleanEquivalentString(str: string): string {
	str = markupToPlainTextString(str);

	// Replace all whitespace with spaces
	str = str.replace(/[\s\n]+/g, " ");

	// Remove trailing dot
	str = str.replace(/\.+$/, "");

	// Remove surrounding quotes
	str = str.replace(/^"(.*?)"$/, "$1");

	return str;
}

export function joinNoBreak(lines: Array<string>): string {
	return `<nobr>${lines.join("\n")}</nobr>`;
}

export function splitLines(src: string): Array<string> {
	return src.replace(/\t/g, " ").split(NEWLINE);
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
