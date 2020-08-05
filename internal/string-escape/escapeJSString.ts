/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/**
 * This file contains code licensed under MIT from the jsesc project:
 *
 *   https://github.com/mathiasbynens/jsesc
 *
 * And is covered by the following license:
 *
 *   Copyright Mathias Bynens <https://mathiasbynens.be/>
 *
 *   Permission is hereby granted, free of charge, to any person obtaining
 *   a copy of this software and associated documentation files (the
 *   "Software"), to deal in the Software without restriction, including
 *   without limitation the rights to use, copy, modify, merge, publish,
 *   distribute, sublicense, and/or sell copies of the Software, and to
 *   permit persons to whom the Software is furnished to do so, subject to
 *   the following conditions:
 *
 *   The above copyright notice and this permission notice shall be
 *   included in all copies or substantial portions of the Software.
 *
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 *   EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 *   MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 *   NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
 *   LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 *   OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
 *   WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

import {DOUBLE_QUOTE, SINGLE_QUOTE, TICK_QUOTE} from "./constants";
import {isDigit} from "@internal/parser-core";

// This regex represents printable ASCII characters, except the characters: '"\`
const PRINTABLE_ASCII = /[ !#-&\(-\[\]-_a-~]/;

function escapeChar(
	char: string,
	ignoreWhitespaceEscapes: boolean,
): undefined | string {
	switch (char) {
		case '"':
			return '\\"';

		case "'":
			return "\\'";

		case "\b":
			return "\\b";

		case "\f":
			return "\\f";

		case "\\":
			return "\\\\";
	}

	switch (char) {
		case "\n":
			return ignoreWhitespaceEscapes ? char : "\\n";

		case "\r":
			return ignoreWhitespaceEscapes ? char : "\\r";

		case "\t":
			return ignoreWhitespaceEscapes ? char : "\\t";
	}

	return undefined;
}

type QuoteChar = "" | '"' | "'" | "`";

type EscapeStringOptions = {
	quote?: QuoteChar;
	json?: boolean;
	ignoreWhitespaceEscapes?: boolean;
	unicodeOnly?: boolean;
};

export default function escapeJSString(
	str: string,
	opts: EscapeStringOptions = {},
): string {
	let index = -1;
	let result = "";

	const {
		ignoreWhitespaceEscapes = false,
		quote = "",
		json = false,
		unicodeOnly = false,
	} = opts;

	// Loop over each code unit in the string and escape it
	while (++index < str.length) {
		const char = str[index];

		// Handle surrogate pairs in non-JSON mode
		if (!json) {
			const charCode = str.charCodeAt(index);
			const isHighSurrogate = charCode >= 55_296 && charCode <= 56_319;
			const hasNextCodePoint = str.length > index + 1;
			const isSurrogatePairStart = isHighSurrogate && hasNextCodePoint;

			if (isSurrogatePairStart) {
				const nextCharCode = str.charCodeAt(index + 1);
				const isLowSurrogate = nextCharCode >= 56_320 && nextCharCode <= 57_343;
				if (isLowSurrogate) {
					// https://mathiasbynens.be/notes/javascript-encoding#surrogate-formulae
					const codePoint =
						(charCode - 55_296) * 1_024 + nextCharCode - 56_320 + 65_536;
					const hex = codePoint.toString(16);
					result += `\\u{${hex}}`;
					index++;
					continue;
				}
			}
		}

		//
		if (PRINTABLE_ASCII.test(char)) {
			// It’s a printable ASCII character that is not `"`, `'` or `\`,
			// so don’t escape it.
			result += char;
			continue;
		}

		// Escape double quotes
		if (char === DOUBLE_QUOTE) {
			result += quote === char ? '\\"' : char;
			continue;
		}

		// Escape single quotes
		if (char === SINGLE_QUOTE) {
			result += quote === char ? "\\'" : char;
			continue;
		}

		// Escape back tick
		if (char === TICK_QUOTE) {
			result += quote === char ? "\\`" : char;
			continue;
		}

		// Null escape
		if (char === "\0" && !json && !isDigit(str[index + 1])) {
			result += "\\0";
			continue;
		}

		// Simple escapes
		if (!unicodeOnly) {
			const replacement = escapeChar(char, ignoreWhitespaceEscapes);
			if (replacement !== undefined) {
				result += replacement;
				continue;
			}
		}

		// Unicode escape
		const hex = char.charCodeAt(0).toString(16);
		const isLonghand = json || hex.length > 2;
		const modifier = isLonghand ? "u" : "x";
		const code = `0000${hex}`.slice(isLonghand ? -4 : -2);
		const escaped = `\\${modifier}${code}`;
		result += escaped;
	}

	return `${quote}${result}${quote}`;
}
