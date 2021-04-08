/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {isHexDigit} from "@internal/parser-core";
import {DiagnosticDescription, descriptions} from "@internal/diagnostics";
import {isEscaped} from "@internal/string-utils";
import {ZeroIndexed} from "@internal/numbers";
import {readMarkup} from "@internal/markup";

function unescapeChar(modifier: string, index: number, opts: UnescapeStringOptions): string {
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

		opts.unexpected(descriptions.STRING_ESCAPE.TOML_INVALID_ESCAPE, index);
	}

	// Allowed in JSON
	switch (modifier) {
		case "v":
			return "\x0b";

		default:
			return modifier;
	}
}

type UnescapeStringUnexpected = (
	metadata: Omit<DiagnosticDescription, "category">,
	index: number,
) => void;

type UnescapeStringOptions = {
	mode: "json" | "toml-singleline" | "toml-multiline";
	unexpected: UnescapeStringUnexpected;
};

const UNEXPECTED_DEFAULT_THROWER: UnescapeStringUnexpected = (
	metadata: Omit<DiagnosticDescription, "category">,
	index: number,
) => {
	throw new TypeError(`${readMarkup(metadata.message)} at index ${String(index)}`);
};

export default function unescapeString(
	input: string,
	rawOpts: UnescapeStringOptions | Omit<UnescapeStringOptions, "unexpected">,
): string {
	const opts: UnescapeStringOptions = "unexpected" in rawOpts ? rawOpts : {unexpected: UNEXPECTED_DEFAULT_THROWER, mode: rawOpts.mode};
	const {unexpected, mode} = opts;

	let buffer = "";
	let index = 0;

	while (index < input.length) {
		const char = input[index];

		if (mode === "toml-singleline" || mode === "toml-multiline") {
			if (char === "\r") {
				// Ignore it
				index++;
				continue;
			}

			if (char === "\n" && opts.mode === "toml-singleline") {
				unexpected(descriptions.STRING_ESCAPE.TOML_NEWLINE_IN_SINGLE_QUOTE_STRING, index);
			}

			if (char === "\n" || char === "\t") {
				// Add it verbatim
				buffer += char;
				index++;
				continue;
			}
		}

		// It's verbatim if it's an escaped backslash or not a backslash
		if (
			(isEscaped(new ZeroIndexed(index), input) && char === "\\") ||
			char !== "\\"
		) {
			// Validate that this is a valid character
			const codePoint = char.codePointAt(0)!;
			if (codePoint >= 0 && codePoint <= 31) {
				unexpected(
					descriptions.STRING_ESCAPE.INVALID_STRING_CHARACTER,
					index,
				);
				index++;
				continue;
			}

			// Add it verbatim
			buffer += char;
			index++;
			continue;
		}

		// Anything after here is escaped
		const modifierIndex = index + 1;
		const modifier = input[modifierIndex];

		if (modifier === "u") {
			// Get the next 4 characters as the code point
			const codeStartIndex = modifierIndex + 1;
			const rawCode = input.slice(codeStartIndex, codeStartIndex + 4);

			// Validate that we have at least 4 digits
			if (rawCode.length < 4) {
				// (index of the point start + total point digits)
				const lastDigitIndex = codeStartIndex + rawCode.length - 1;
				unexpected(
					descriptions.STRING_ESCAPE.NOT_ENOUGH_CODE_POINTS,
					lastDigitIndex,
				);
			}

			// Validate that each character is a valid hex digit
			for (let i = 0; i < rawCode.length; i++) {
				const char = rawCode[i];
				if (!isHexDigit(char)) {
					// Get the current source index for this character
					// (code start index + digit index)
					const pos = codeStartIndex + i;
					unexpected(
						descriptions.STRING_ESCAPE.INVALID_HEX_DIGIT_FOR_ESCAPE,
						pos,
					);
				}
			}

			// Validate the code point
			const code = parseInt(rawCode, 16);

			// Get the character for this code point
			buffer += String.fromCodePoint(code);

			// Skip ahead six indexes (1 escape char +  1 modifier + 4 hex digits)
			index += 6;
		} else {
			// Unescape a basic modifier like \t
			buffer += unescapeChar(modifier, index, opts);

			// Skip ahead two indexes to also take along the modifier
			index += 2;
		}
	}

	return buffer;
}
