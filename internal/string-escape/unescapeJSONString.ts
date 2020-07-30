/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {isHexDigit} from "@internal/parser-core";
import {DiagnosticDescription, descriptions} from "@internal/diagnostics";
import {isEscaped} from "@internal/string-utils";
import {ob1Coerce0} from "@internal/ob1";
import {readMarkup} from "@internal/markup";

function unescapeChar(modifier: string): string {
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

const UNEXPECTED_DEFAULT_THROWER: UnescapeStringUnexpected = (
	metadata: Omit<DiagnosticDescription, "category">,
	index: number,
) => {
	throw new TypeError(`${readMarkup(metadata.message)} (${String(index)})`);
};

export default function unescapeJSONString(
	input: string,
	unexpected: UnescapeStringUnexpected = UNEXPECTED_DEFAULT_THROWER,
	allowWhitespace: boolean = false,
): string {
	let buffer = "";

	let index = 0;

	while (index < input.length) {
		const char = input[index];

		if (allowWhitespace) {
			if (char === "\r") {
				// Ignore it
				index++;
				continue;
			}

			if (char === "\n" || char === "\t") {
				// Add it verbatim
				buffer += char;
				index++;
				continue;
			}
		}

		// It's verbatim if it's an escaped backslash or not a backslash
		if ((isEscaped(ob1Coerce0(index), input) && char === "\\") || char !== "\\") {
			// Validate that this is a valid character
			const codePoint = char.codePointAt(0);
			if (codePoint === undefined) {
				throw new Error("Already validated that this index exists");
			}
			if (codePoint >= 0 && codePoint <= 31) {
				throw unexpected(
					descriptions.STRING_ESCAPE.INVALID_STRING_CHARACTER,
					index,
				);
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
				throw unexpected(
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
					throw unexpected(
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
			buffer += unescapeChar(modifier);

			// Skip ahead two indexes to also take along the modifier
			index += 2;
		}
	}

	return buffer;
}
