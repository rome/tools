/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	TokenizerCore,
	TokenizerUnexpected,
	isHexDigit,
	isWhitespace,
} from "@internal/parser-core";
import {descriptions} from "@internal/diagnostics";
import {hasEscapes, isEscaped} from "@internal/string-utils";
import {ZeroIndexed} from "@internal/numbers";

function unescapeChar(
	tokenizer: TokenizerCore,
	opts: UnescapeStringOptions,
	modifier: string,
	index: ZeroIndexed,
): string {
	// Allowed in JSON and TOML
	switch (modifier) {
		case "b":
			return "\b";

		case "f":
			return "\f";

		case "n":
			return "\n";

		case "r":
			return "\r";

		case "t":
			return "\t";
	}

	if (opts.mode === "toml-singleline" || opts.mode === "toml-multiline") {
		// Only additional escape allowed is a double quote or backslash
		if (modifier === '"' || modifier === "\\") {
			return modifier;
		}

		tokenizer.unexpected(descriptions.STRING_ESCAPE.TOML_INVALID_ESCAPE, index);
	}

	// Allowed in JSON
	switch (modifier) {
		case "v":
			return "\x0b";

		default:
			return modifier;
	}
}

type UnescapeStringOptions = {
	mode: "json" | "toml-singleline" | "toml-multiline";
	unexpected?: TokenizerUnexpected;
};

function isValidTOMLUnicode(code: number): boolean {
	return (code >= 0 && code <= 0xd7ff) || (code >= 0xe000 && code <= 0x10ffff);
}

function unescapeUnicode(
	tokenizer: TokenizerCore,
	length: number,
	isTOML: boolean,
): string {
	const start = tokenizer.index;
	const rawCode = tokenizer.readAssertCount("hex digit", length, isHexDigit);

	// Validate the code point
	const code = parseInt(rawCode, 16);

	if (isTOML && !isValidTOMLUnicode(code)) {
		tokenizer.unexpected(
			descriptions.STRING_ESCAPE.TOML_INVALID_UNICODE_POINT,
			start,
		);
	}

	// Get the character for this code point
	return String.fromCodePoint(code);
}

export default function unescapeString(
	input: string,
	opts: UnescapeStringOptions,
): string {
	if (!hasEscapes(input)) {
		return input;
	}

	const {mode} = opts;
	const isTOML = mode === "toml-singleline" || mode === "toml-multiline";

	let buffer = "";

	const tokenizer = new TokenizerCore({input, unexpected: opts.unexpected});

	while (!tokenizer.isEOF()) {
		const {index} = tokenizer;
		const char = tokenizer.take(1);

		if (isTOML) {
			if (char === "\n" && opts.mode === "toml-singleline") {
				tokenizer.unexpected(
					descriptions.STRING_ESCAPE.TOML_NEWLINE_IN_SINGLE_QUOTE_STRING,
					index,
				);
			}

			if (char === "\n" || char === "\t") {
				// Add it verbatim
				buffer += char;
				continue;
			}
		}

		// It's verbatim if it's an escaped backslash or not a backslash
		if ((isEscaped(index, input) && char === "\\") || char !== "\\") {
			// Validate that this is a valid character
			const codePoint = char.codePointAt(0)!;
			if (codePoint >= 0 && codePoint <= 31) {
				tokenizer.unexpected(
					descriptions.STRING_ESCAPE.INVALID_STRING_CHARACTER,
					index,
				);
				continue;
			}

			// Add it verbatim
			buffer += char;
			continue;
		}

		// Anything after here is escaped
		const modifier = tokenizer.take(1);

		if (modifier === "U" && isTOML) {
			buffer += unescapeUnicode(tokenizer, 8, true);
			continue;
		}

		if (modifier === "u") {
			buffer += unescapeUnicode(tokenizer, 4, isTOML);
			continue;
		}

		// Multiline TOML strings with a trailing newline will trim all subsequent whitespace
		if (opts.mode === "toml-multiline" && modifier === "\n") {
			// Trim leading whitespace
			tokenizer.read(isWhitespace);
			continue;
		}

		// Unescape a basic modifier like \t
		buffer += unescapeChar(tokenizer, opts, modifier, index);
	}

	return buffer;
}
