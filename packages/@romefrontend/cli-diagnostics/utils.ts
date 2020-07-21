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
import {AnyRoot} from "@romefrontend/ast";
import {DiagnosticLanguage} from "@romefrontend/diagnostics";
import {
	Markup,
	convertToMarkupFromRandomString,
	markup,
} from "@romefrontend/cli-layout";

const unicodeControls = /[\u0000-\u001f\u007f-\u00a0]/u;

function isWhitespace(char: undefined | string): boolean {
	return char === " " || char === "\t" || char === "\r" || char === "\n";
}

export function showInvisibles(
	str: string,
	{atLineStart, atLineEnd, ignoreLoneSpaces, ignoreLeadingTabs}: {
		ignoreLeadingTabs: boolean;
		ignoreLoneSpaces: boolean;
		atLineStart: boolean;
		atLineEnd: boolean;
	},
): {
	value: Markup;
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
		if (char === " " && ignoreLoneSpaces) {
			showInvisible = false;

			if (isWhitespace(str[i - 1]) || isWhitespace(str[i + 1])) {
				showInvisible = false;
			}
		}

		// Don't show leading tabs
		if (atLineStart && !hadNonWhitespace && char === "\t" && ignoreLeadingTabs) {
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
			ret += markup`<dim>${visible}</dim>`.value;
			continue;
		}

		if (nonASCIIwhitespace.test(char) || unicodeControls.test(char)) {
			ret += showUnicodeChar(char).value;
			continue;
		}

		ret += char;
	}

	return {
		hadNonWhitespace,
		value: convertToMarkupFromRandomString(ret),
	};
}

function showUnicodeChar(char: string): Markup {
	// We use inverse to make it clear that it's not in the source
	return markup`<inverse>U+${char.codePointAt(0)!.toString(16)}</inverse>`;
}

function showInvisibleChar(char: string): undefined | string | Markup {
	switch (char) {
		case " ":
			return "\xb7"; // Middle Dot

		case "\r":
			return "\u240d"; // Carriage Return Symbol

		case "\n":
			return "\u23ce"; // Return Symbol

		case "\t":
			// TODO this should be repeated for tabWidth
			return "→ "; // Rightwards Arrow

		case "\0":
			return "\u2400"; // Null Symbol

		case "\x0b":
			return "\u240b"; // Vertical Tabulation Symbol

		case "\b":
			return "\u232b"; // Backspace Symbol

		case "\f":
			return "\u21a1"; // Downards Two Headed Arrow

		default:
			return undefined;
	}
}

export function cleanEquivalentString(safe: string | Markup): string {
	let str =
		typeof safe === "string" ? safe : joinMarkupLines(markupToPlainText(safe));

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

export type ToLines = Array<[string, Markup]>;

export function toLines(opts: AnsiHighlightOptions): ToLines {
	const input = removeCarriageReturn(opts.input);
	const raw = splitLines(input);
	const highlighted = highlightCode({...opts, input});

	if (raw.length !== highlighted.length) {
		throw new Error(
			`${opts.path.join()}: raw and highlighted line count mismatch ${raw.length} !== ${highlighted.length}`,
		);
	}

	return raw.map((line, i) => [line, highlighted[i]]);
}

export function inferDiagnosticLanguageFromRootAST(
	ast: AnyRoot,
): DiagnosticLanguage {
	switch (ast.type) {
		case "JSRoot":
			return "js";

		case "HTMLRoot":
			return "html";

		case "CSSRoot":
			return "css";

		case "MarkdownRoot":
			return "md";
	}
}
