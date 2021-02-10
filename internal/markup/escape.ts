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
	AnyPath,
	RelativeFilePath,
	UIDPath,
	URLPath,
	UnknownPath,
} from "@internal/path";
import {UnknownNumber, isNumber, ob1Get} from "@internal/ob1";

type LazyMarkupPart = StaticMarkup | LazyMarkupFactory | LazyMarkup;

export type LazyMarkupFactory = () => AnyMarkup;

export type StaticMarkups = StaticMarkup[];

export type StaticMarkup = string | RawMarkup | StaticMarkup[];

export type AnyMarkup = StaticMarkup | LazyMarkup | LazyMarkupFactory;

export type AnyMarkups = AnyMarkup[];

type LazyMarkup = {
	type: "LAZY_MARKUP";
	parts: LazyMarkupPart[];
};

type RawMarkup = {
	type: "RAW_MARKUP";
	value: string;
};

function isRawMarkup(part: LazyMarkupPart): part is RawMarkup {
	return (
		typeof part === "object" &&
		part != null &&
		!Array.isArray(part) &&
		part.type === "RAW_MARKUP"
	);
}

function isLazyMarkup(
	part: LazyMarkupPart,
): part is LazyMarkup | LazyMarkupFactory {
	return isLazyMarkupParts(part) || isLazyMarkupFactory(part);
}

function isLazyMarkupParts(part: LazyMarkupPart): part is LazyMarkup {
	return (
		typeof part === "object" &&
		part != null &&
		!Array.isArray(part) &&
		part.type === "LAZY_MARKUP"
	);
}

function isLazyMarkupFactory(part: LazyMarkupPart): part is LazyMarkupFactory {
	return typeof part === "function";
}

// Awkward name since we should only be doing this very very sparingly
export function convertToMarkupFromRandomString(unsafe: string): StaticMarkup {
	return toRawMarkup(unsafe);
}

export function filePathToMarkup(
	path: AnyPath,
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

type InterpolatedValue = undefined | number | bigint | AnyPath | UnknownNumber;

const markupTemplateCache: WeakMap<TemplateStringsArray, AnyMarkup> = new WeakMap();

// A tagged template literal helper that will escape all interpolated strings, ensuring only markup works
export function markup(
	strs: TemplateStringsArray,
	...values: Array<StaticMarkup | InterpolatedValue>
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

	const parts: LazyMarkupPart[] = [];

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
		} else if (isNumber(value)) {
			parts.push(toRawMarkup(`<number>${String(ob1Get(value))}</number>`));
		} else if (
			value instanceof RelativeFilePath ||
			value instanceof AbsoluteFilePath ||
			value instanceof URLPath ||
			value instanceof UIDPath ||
			value instanceof UnknownPath
		) {
			parts.push(filePathToMarkup(value));
		} else {
			parts.push(value);
		}
	}

	const obj = concatMarkup(parts);

	// No interpolated values so result is static
	if (values.length === 0) {
		markupTemplateCache.set(strs, obj);
	}

	return obj;
}

// Here we have a cache making serializing markup so we can call it performantly with only the object
// We can also benefit from a small speed up by common interpolated markup
const readCache: WeakMap<Exclude<AnyMarkup, string>, string> = new WeakMap();

export function readMarkup(item: AnyMarkup): string {
	if (isLazyMarkupFactory(item)) {
		return readMarkup(serializeLazyMarkup(item));
	}

	if (typeof item === "string") {
		return escapeMarkup(item);
	}

	const cached = readCache.get(item);
	if (cached !== undefined) {
		return cached;
	}

	if (isRawMarkup(item)) {
		return item.value;
	}

	let out = "";

	if (isLazyMarkupParts(item)) {
		for (const part of item.parts) {
			out += readMarkup(part);
		}
	} else {
		for (const part of item) {
			out += readMarkup(part);
		}
	}

	readCache.set(item, out);
	return out;
}

const factoryCache: WeakMap<LazyMarkupFactory, AnyMarkup> = new WeakMap();

export function serializeLazyMarkup(markup: AnyMarkup): StaticMarkup {
	if (isLazyMarkupParts(markup)) {
		return [toRawMarkup(readMarkup(markup))];
	} else if (isLazyMarkupFactory(markup)) {
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

export function isEmptyMarkup(
	safe: AnyMarkup | StaticMarkup | LazyMarkupPart[],
	allowLazy: boolean = true,
): boolean {
	if (Array.isArray(safe)) {
		for (const part of safe) {
			if (!isEmptyMarkup(part, allowLazy)) {
				return false;
			}
		}

		return true;
	}

	if (!allowLazy && (isLazyMarkupParts(safe) || isLazyMarkupFactory(safe))) {
		return false;
	}

	if (isLazyMarkupFactory(safe)) {
		return isEmptyMarkup(serializeLazyMarkup(safe));
	}

	if (typeof safe === "string") {
		return safe === "";
	}

	if (isRawMarkup(safe)) {
		return isEmptyMarkup(safe.value, allowLazy);
	}

	if (isLazyMarkup(safe)) {
		return isEmptyMarkup(safe.parts, allowLazy);
	}

	throw new Error("safe should be never");
}

export function concatMarkup(
	items: AnyMarkups,
	separator?: StaticMarkup,
): StaticMarkup;
export function concatMarkup(
	items: AnyMarkup[],
	separator?: AnyMarkup,
): AnyMarkup;
export function concatMarkup(
	items: AnyMarkup[],
	separator?: AnyMarkup,
): AnyMarkup {
	let hasLazy = separator !== undefined && isLazyMarkup(separator);
	let hasSeparator = separator !== undefined && !isEmptyMarkup(separator, false);
	let canSimplify =
		separator !== undefined &&
		(typeof separator === "string" || isRawMarkup(separator));

	// Construct
	const fatParts: LazyMarkupPart[] = [];
	for (let i = 0; i < items.length; i++) {
		const item = items[i];

		if (!isEmptyMarkup(item, false)) {
			fatParts.push(item);

			if (typeof item === "string" || isRawMarkup(item)) {
				canSimplify = true;
			}

			if (isLazyMarkup(item)) {
				hasLazy = true;
			}
		}

		if (hasSeparator && separator !== undefined && i !== items.length - 1) {
			fatParts.push(separator);
		}
	}

	let parts: LazyMarkupPart[] = fatParts;

	// Simplify parts
	if (canSimplify) {
		parts = [];

		while (fatParts.length > 0) {
			let part = fatParts.shift()!;

			if (fatParts.length > 0) {
				if (typeof part === "string") {
					while (typeof fatParts[0] === "string") {
						part += fatParts.shift();
					}
				}

				if (isRawMarkup(part)) {
					while (isRawMarkup(fatParts[0])) {
						part = {
							type: "RAW_MARKUP",
							value: part.value + (fatParts.shift() as RawMarkup).value,
						};
					}
				}
			}

			parts.push(part);
		}
	}

	if (parts.length === 0) {
		return "";
	}

	if (parts.length === 1) {
		return parts[0];
	}

	if (hasLazy) {
		return {
			type: "LAZY_MARKUP",
			parts,
		};
	} else {
		return parts as StaticMarkups;
	}
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

	if (typeof text !== "string" && isLazyMarkup(text)) {
		return {
			type: "LAZY_MARKUP",
			parts: [open, text, close],
		};
	} else {
		return [open, text, close];
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
