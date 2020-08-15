/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// Matches a whole line break (where CRLF is considered a single line break). Used to count lines.
export const lineBreak = /\r\n?|\n|u2028|u2029/;
export const lineBreakG = new RegExp(lineBreak.source, "g");

export function isNewLine(code: number): boolean {
	return code === 10 || code === 13 || code === 8_232 || code === 8_233;
}

export const nonASCIIwhitespace = /[\u1680\u180e\u2000-\u200a\u202f\u205f\u3000\ufeff]/;

// rome-ignore lint/regex/noEmptyMatches
export const skipWhiteSpace = /(?:\s|\/\/.*|\/\*[^]*?\*\/)*/g;
