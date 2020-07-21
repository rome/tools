/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Dict} from "@romefrontend/typescript-helpers";
import {MarkupTagName} from "./types";
import { RelativeFilePath, AbsoluteFilePath, URLFilePath } from "@romefrontend/path";

// Awkward name since we should only be doing this very very sparingly
export function convertToMarkupFromRandomString(unsafe: string): Markup {
	return safeMarkup(unsafe);
}

// A tagged template literal helper that will escape all interpolated strings, ensuring only markup works
export function markup(
	strs: TemplateStringsArray,
	...values: Array<Markup | string | number | RelativeFilePath | AbsoluteFilePath | URLFilePath>
): Markup {
	let out = "";

	for (let i = 0; i < strs.length; i++) {
		const str = strs[i];
		out += str;
		if (i === strs.length - 1) {
			continue;
		}

		const value = values[i];

		if (typeof value === "number") {
			out += `<number>${String(value)}</number>`;
		} else if (value instanceof URLFilePath) {
			out += markup`<hyperlink target="${value.join()}" />`.value;
		} else if (value instanceof RelativeFilePath || value instanceof AbsoluteFilePath) {
			out += markup`<filelink target="${value.join()}" />`.value;
		} else if (typeof value === "object" && value != null && value.type === "SAFE_MARKUP") {
			out += value.value;
		} else {
			out += escapeMarkup(String(value));
			continue;
		}
	}

	return safeMarkup(out);
}

export function isEmptyMarkup(safe: Markup): boolean {
	return safe.value === "";
}

export type Markup = {
	type: "SAFE_MARKUP";
	value: string;
};

export function concatMarkup(
	items: Array<Markup>,
	separator: Markup = markup``,
): Markup {
	return safeMarkup(items.map((item) => item.value).join(separator.value));
}

function safeMarkup(input: string): Markup {
	return {
		type: "SAFE_MARKUP",
		value: input,
		toString() {
			throw new Error(`Wtf??? ${input}`);
		},
	};
}

// Escape all \ and >
function escapeMarkup(input: string): string {
	let escaped = "";
	for (let i = 0; i < input.length; i++) {
		const char = input[i];

		if (char === "<") {
			escaped += "\\<";
		} else if (char === '"') {
			escaped += '\\"';
		} else if (char === "\\") {
			escaped += "\\\\";
		} else {
			escaped += char;
		}
	}
	return escaped;
}

export function markupTag(
	tagName: MarkupTagName,
	text: Markup,
	attrs?: Dict<undefined | string | number | boolean>,
): Markup {
	let ret = `<${tagName}`;

	if (attrs !== undefined) {
		for (const key in attrs) {
			const value = attrs[key];
			if (value !== undefined) {
				ret += markup` ${key}="${String(value)}"`.value;
			}
		}
	}

	ret += `>${text.value}</${tagName}>`;

	return safeMarkup(ret);
}

export function unescapeTextValue(str: string): string {
	let unescaped = "";

	for (let i = 0; i < str.length; i++) {
		const char = str[i];

		// Unescape \\< to just <
		// Unescape \\\\ to just \\
		if (char === "\\") {
			const nextChar = str[i + 1];
			if (nextChar === "<" || nextChar === '"' || nextChar === "\\") {
				i++;
				unescaped += nextChar;
				continue;
			}
		}

		unescaped += char;
	}

	return unescaped;
}
