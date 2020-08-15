/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Dict} from "@internal/typescript-helpers";
import {MarkupTagName} from "./types";
import {
	AbsoluteFilePath,
	AnyFilePath,
	RelativeFilePath,
	URLPath,
	UnknownPath,
} from "@internal/path";

type MarkupPart = StaticMarkup | RawMarkup | string;
type LazyMarkupPart = MarkupPart | LazyMarkupFactory | LazyMarkup;

export type LazyMarkupFactory = () => AnyMarkup;

export type StaticMarkup = {
	type: "MARKUP";
	parts: Array<MarkupPart>;
};

export type StaticMarkups = Array<StaticMarkup>;

export type AnyMarkup = StaticMarkup | LazyMarkup | LazyMarkupFactory;

export type AnyMarkups = Array<AnyMarkup>;

type LazyMarkup = {
	type: "LAZY_MARKUP";
	parts: Array<LazyMarkupPart>;
};

type RawMarkup = {
	type: "RAW_MARKUP";
	value: string;
};

function isMarkup(part: LazyMarkupPart): part is StaticMarkup {
	return typeof part === "object" && part != null && part.type === "MARKUP";
}

function isRawMarkup(part: LazyMarkupPart): part is RawMarkup {
	return typeof part === "object" && part != null && part.type === "RAW_MARKUP";
}

function isLazyMarkup(part: LazyMarkupPart): part is LazyMarkup {
	return typeof part === "object" && part != null && part.type === "LAZY_MARKUP";
}

// Awkward name since we should only be doing this very very sparingly
export function convertToMarkupFromRandomString(unsafe: string): StaticMarkup {
	return {
		type: "MARKUP",
		parts: [toRawMarkup(unsafe)],
	};
}

export function filePathToMarkup(
	path: AnyFilePath,
	explicit: boolean = false,
): StaticMarkup {
	let tagName: MarkupTagName = "filelink";
	if (path instanceof URLPath) {
		tagName = "hyperlink";
	}

	const target = path.join();
	const text = explicit ? markup`${target}` : markup``;

	return markupTag(
		tagName,
		text,
		{
			target,
		},
	);
}

type InterpolatedValue = undefined | number | AnyFilePath;

const markupTemplateCache: WeakMap<TemplateStringsArray, AnyMarkup> = new WeakMap();

// A tagged template literal helper that will escape all interpolated strings, ensuring only markup works
export function markup(
	strs: TemplateStringsArray,
	...values: Array<MarkupPart | InterpolatedValue>
): StaticMarkup;
export function markup(
	strs: TemplateStringsArray,
	...values: Array<LazyMarkupPart | InterpolatedValue>
): AnyMarkup;
export function markup(
	strs: TemplateStringsArray,
	...values: Array<LazyMarkupPart | InterpolatedValue>
): AnyMarkup {
	if (values.length === 0) {
		const cached = markupTemplateCache.get(strs);
		if (cached !== undefined) {
			return cached;
		}
	}

	const parts: Array<LazyMarkupPart> = [];
	let hasLazy = false;

	for (let i = 0; i < strs.length; i++) {
		const str = strs[i];
		if (str !== "") {
			parts.push({
				type: "RAW_MARKUP",
				value: str,
			});
		}

		// Last string is not followed by an interpolated value
		if (i === strs.length - 1) {
			continue;
		}

		const value = values[i];
		if (typeof value === "undefined") {
			parts.push(toRawMarkup("<dim>undefined</dim>"));
		} else if (typeof value === "number") {
			parts.push(toRawMarkup(`<number>${String(value)}</number>`));
		} else if (
			value instanceof RelativeFilePath ||
			value instanceof AbsoluteFilePath ||
			value instanceof URLPath ||
			value instanceof UnknownPath
		) {
			parts.push(filePathToMarkup(value));
		} else {
			if (typeof value === "function" || isLazyMarkup(value)) {
				hasLazy = true;
			}
			parts.push(value);
		}
	}

	const obj: AnyMarkup = {
		// @ts-ignore
		type: hasLazy ? "LAZY_MARKUP" : "MARKUP",
		parts,
	};

	// No interpolated values so result is static
	if (values.length === 0) {
		markupTemplateCache.set(strs, obj);
	}

	return obj;
}

// Here we have a cache making serializing markup so we can call it performantly with only the object
// We can also benefit from a small speed up by common interpolated markup
const readCache: WeakMap<AnyMarkup, string> = new WeakMap();

export function readMarkup(item: AnyMarkup): string {
	if (typeof item === "function") {
		return readMarkup(serializeLazyMarkup(item));
	}

	const cached = readCache.get(item);
	if (cached !== undefined) {
		return cached;
	}

	let out = "";
	for (const part of item.parts) {
		if (isRawMarkup(part)) {
			out += part.value;
		} else if (
			isMarkup(part) ||
			isLazyMarkup(part) ||
			typeof part === "function"
		) {
			out += readMarkup(part);
		} else {
			out += escapeMarkup(String(part));
		}
	}
	readCache.set(item, out);
	return out;
}

const factoryCache: WeakMap<LazyMarkupFactory, AnyMarkup> = new WeakMap();

export function serializeLazyMarkup(markup: AnyMarkup): StaticMarkup {
	if (isLazyMarkup(markup)) {
		return {
			type: "MARKUP",
			parts: [toRawMarkup(readMarkup(markup))],
		};
	} else if (typeof markup === "function") {
		let res = factoryCache.get(markup);
		if (res === undefined) {
			res = markup();
			factoryCache.set(markup, res);
		}
		return serializeLazyMarkup(res);
	} else {
		return markup;
	}
}

export function isEmptyMarkup(safe: AnyMarkup): boolean {
	if (typeof safe === "function") {
		return isEmptyMarkup(serializeLazyMarkup(safe));
	}

	for (const part of safe.parts) {
		if (typeof part === "string") {
			if (part !== "") {
				return false;
			}
		} else if (isMarkup(part)) {
			if (!isEmptyMarkup(part)) {
				return false;
			}
		} else if (isRawMarkup(part)) {
			if (part.value !== "") {
				return false;
			}
		} else if (isLazyMarkup(part)) {
			if (!isEmptyMarkup(part)) {
				return false;
			}
		} else {
			return false;
		}
	}

	return true;
}

export function concatMarkup(
	items: AnyMarkups,
	separator?: StaticMarkup,
): StaticMarkup;
export function concatMarkup(
	items: Array<AnyMarkup>,
	separator?: AnyMarkup,
): AnyMarkup;
export function concatMarkup(
	items: Array<AnyMarkup>,
	separator: AnyMarkup = markup``,
): AnyMarkup {
	const parts: Array<LazyMarkupPart> = [];
	let hasLazy = isLazyMarkup(separator);

	for (let i = 0; i < items.length; i++) {
		const item = items[i];
		parts.push(item);

		if (isLazyMarkup(item)) {
			hasLazy = true;
		}

		if (i !== items.length - 1) {
			parts.push(separator);
		}
	}

	return {
		// @ts-ignore
		type: hasLazy ? "LAZY_MARKUP" : "MARKUP",
		parts,
	};
}

function toRawMarkup(value: string): RawMarkup {
	return {
		type: "RAW_MARKUP",
		value,
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

type MarkupAttributes = Dict<undefined | string | number | boolean>;

export function markupTag(
	tagName: MarkupTagName,
	text: StaticMarkup,
	attrs?: MarkupAttributes,
): StaticMarkup;
export function markupTag(
	tagName: MarkupTagName,
	text: AnyMarkup,
	attrs?: MarkupAttributes,
): AnyMarkup;
export function markupTag(
	tagName: MarkupTagName,
	text: AnyMarkup,
	attrs?: MarkupAttributes,
): AnyMarkup {
	let ret = `<${tagName}`;

	if (attrs !== undefined) {
		for (const key in attrs) {
			const value = attrs[key];
			if (value !== undefined) {
				ret += ` ${escapeMarkup(key)}="${escapeMarkup(String(value))}"`;
			}
		}
	}

	const open = toRawMarkup(`${ret}>`);
	const close = toRawMarkup(`</${tagName}>`);

	if (typeof text === "function" || text.type === "LAZY_MARKUP") {
		return {
			type: "LAZY_MARKUP",
			parts: [open, text, close],
		};
	} else {
		return {
			type: "MARKUP",
			parts: [open, text, close],
		};
	}
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
