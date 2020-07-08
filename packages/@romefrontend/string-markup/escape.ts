/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Dict} from "@romefrontend/typescript-helpers";
import {MarkupTagName} from "./types";

// A tagged template literal helper that will escape all interpolated strings, ensuring only markup works
export function markup(
	strs: TemplateStringsArray,
	...values: Array<unknown>
): string {
	let out = "";

	for (let i = 0; i < strs.length; i++) {
		const str = strs[i];
		out += str;

		const interpolated = values[i];

		if (interpolated instanceof SafeMarkup) {
			out += interpolated.value;
			continue;
		}

		if (interpolated !== undefined) {
			out += escapeMarkup(String(interpolated));
		}
	}

	return out;
}

class SafeMarkup {
	constructor(value: string) {
		this.value = value;
	}

	value: string;
}

export function safeMarkup(input: string): SafeMarkup {
	return new SafeMarkup(input);
}

// Escape all \ and >
export function escapeMarkup(input: string): string {
	let escaped = "";
	for (let i = 0; i < input.length; i++) {
		const char = input[i];

		if (char === "<") {
			escaped += "\\<";
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
	text: string,
	attrs?: Dict<undefined | string | number | boolean>,
): string {
	let ret = `<${tagName}`;

	if (attrs !== undefined) {
		for (const key in attrs) {
			const value = attrs[key];
			if (value !== undefined) {
				ret += markup` ${key}="${String(value)}"`;
			}
		}
	}

	ret += `>${text}</${tagName}>`;

	return ret;
}

export function unescapeTextValue(str: string): string {
	let unescaped = "";

	for (let i = 0; i < str.length; i++) {
		const char = str[i];

		// Unescape \\< to just <
		// Unescape \\\\ to just \\
		if (char === "\\") {
			const nextChar = str[i + 1];
			if (nextChar === "<" || nextChar === "\\") {
				i++;
				unescaped += nextChar;
				continue;
			}
		}

		unescaped += char;
	}

	return unescaped;
}
