/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	UnknownObject,
	isIterable,
	mergeObjects,
} from "@internal/typescript-helpers";
import {escapeJSString} from "@internal/string-escape";
import {naturalCompare} from "@internal/string-utils";
import {
	AnyMarkup,
	LazyMarkupFactory,
	StaticMarkup,
	concatMarkup,
	markup,
	markupTag,
	readMarkup,
	serializeLazyMarkup,
} from "@internal/markup";
import {markupToJoinedPlainText} from "@internal/cli-layout";

type RecursiveStack = Array<unknown>;

type FormatOptions = {
	allowCustom: boolean;
	stack: RecursiveStack;
	depth: number;
	maxDepth: number;
	compact: boolean;
};

type FormatPartialOptions = {
	allowCustom?: boolean;
	maxDepth?: number;
	stack?: RecursiveStack;
	compact?: boolean;
};

const DEFAULT_OPTIONS: FormatOptions = {
	allowCustom: true,
	maxDepth: Infinity,
	depth: 0,
	stack: [],
	compact: false,
};

export const CUSTOM_PRETTY_FORMAT = Symbol.for("custom-pretty-format");

export function prettyFormatToString(value: unknown): string {
	return markupToJoinedPlainText(markup`${prettyFormat(value)}`);
}

export function pretty(
	strs: TemplateStringsArray,
	...values: Array<unknown>
): string {
	let out = "";

	for (let i = 0; i < strs.length; i++) {
		const str = strs[i];
		out += str;
		if (i === strs.length - 1) {
			continue;
		}

		const value = values[i];
		out += prettyFormatToString(value);
	}

	return out;
}

export function prettyFormatEager(
	obj: unknown,
	opts?: FormatPartialOptions,
): StaticMarkup {
	return serializeLazyMarkup(prettyFormat(obj, opts));
}

export default function prettyFormat(
	obj: unknown,
	rawOpts: FormatPartialOptions = {},
): LazyMarkupFactory {
	return () => {
		const opts: FormatOptions = mergeObjects(DEFAULT_OPTIONS, rawOpts);

		if (opts.maxDepth === opts.depth) {
			return markup`[depth exceeded]`;
		}

		switch (typeof obj) {
			case "symbol": {
				return markupTag("token", formatSymbol(obj), {type: "string"});
			}

			case "string": {
				return markupTag("token", formatString(obj), {type: "string"});
			}

			case "bigint":
			case "number": {
				return markupTag("token", formatNumber(obj), {type: "number"});
			}

			case "boolean": {
				return markupTag("token", formatBoolean(obj), {type: "boolean"});
			}

			case "undefined": {
				return markupTag("color", formatUndefined(), {fg: "brightBlack"});
			}

			case "function":
				return formatFunction(obj, opts);

			case "object":
				return formatObjectish((obj as Objectish), opts);

			default:
				throw new Error("Unknown type");
		}
	};
}

function isNativeFunction(val: Function): boolean {
	return val.toString().endsWith("{ [native code] }");
}

function formatSymbol(val: Symbol): StaticMarkup {
	return markup`${String(val)}`;
}

function formatString(val: string): StaticMarkup {
	return markup`${escapeJSString(
		val,
		{
			quote: '"',
		},
	)}`;
}

// This function is used by rome-json so make sure it can parse whatever you return here
export function formatNumber(val: bigint | number): StaticMarkup {
	if (typeof val === "bigint") {
		return markup`<number>${String(val)}</number>n`;
	} else if (isNaN(val)) {
		return markup`NaN`;
	} else if (Object.is(val, -0)) {
		return markup`-0`;
	} else if (isFinite(val)) {
		return markup`${val}`;
	} else if (Object.is(val, -Infinity)) {
		return markup`-Infinity`;
	} else if (Object.is(val, +Infinity)) {
		return markup`Infinity`;
	} else {
		throw new Error("Don't know how to format this number");
	}
}

function formatUndefined(): StaticMarkup {
	return markup`undefined`;
}

function formatNull(): StaticMarkup {
	return markup`null`;
}

function formatBoolean(val: boolean): StaticMarkup {
	return val ? markup`true` : markup`false`;
}

function formatFunction(val: Function, opts: FormatOptions): AnyMarkup {
	const name = val.name === "" ? "anonymous" : val.name;
	let label = markup`Function ${name}`;

	if (isNativeFunction(val)) {
		label = markup`Native${label}`;
	}

	if (Object.keys(val).length === 0) {
		return label;
	}

	// rome-ignore lint/ts/noExplicitAny
	return formatObject(label, (val as any), opts, []);
}

function getExtraObjectProps(
	obj: Objectish,
	opts: FormatOptions,
): {
	props: Array<AnyMarkup>;
	ignoreKeys: UnknownObject;
} {
	const props: Array<AnyMarkup> = [];
	const ignoreKeys: UnknownObject = {};

	if (isIterable(obj)) {
		// Duck typing Map check
		if (
			typeof obj.keys === "function" &&
			typeof obj.values === "function" &&
			typeof obj.size === "number"
		) {
			for (const item of obj) {
				if (Array.isArray(item) && item.length === 2) {
					const [key, val] = item;
					const formattedKey =
						typeof key === "string" ? formatKey(key) : prettyFormat(key, opts);
					props.push(markup`${formattedKey} => ${prettyFormat(val, opts)}`);
				} else {
					props.push(markup`${prettyFormat(item, opts)}`);
				}
			}
		} else {
			let i = 0;
			for (const val of obj) {
				ignoreKeys[String(i++)] = val;
				props.push(markup`${prettyFormat(val, opts)}`);
			}
		}
	}

	return {ignoreKeys, props};
}

function formatKey(rawKey: string): StaticMarkup {
	// Format as a string if it contains any special characters
	if (/[^A-Za-z0-9_$]/g.test(rawKey)) {
		return formatString(rawKey);
	} else {
		return markup`${rawKey}`;
	}
}

// These are object keys that should always go at the top and ignore any alphabetization
// This is fairly arbitrary but should include generic identifier keys
export const PRIORITIZE_KEYS = ["id", "type", "kind", "key", "name", "value"];

type KeyInfo = {
	key: string;
	object: boolean;
};

function sortKeys(obj: Objectish): Array<KeyInfo> {
	const sortedKeys: Set<string> = new Set(Object.keys(obj).sort(naturalCompare));

	const priorityKeys: Array<KeyInfo> = [];
	const otherKeys: Array<KeyInfo> = [];
	const objectKeys: Array<KeyInfo> = [];

	for (const key of PRIORITIZE_KEYS) {
		if (sortedKeys.has(key)) {
			priorityKeys.push({key, object: false});
			sortedKeys.delete(key);
		}
	}

	for (const key of sortedKeys) {
		const val = obj[key];

		// Objects with properties should be at the bottom
		let isObject = false;
		if (typeof val === "object" && val != null && Object.keys(val).length > 0) {
			isObject = true;
		}
		if (Array.isArray(val) && val.length > 0) {
			isObject = true;
		}
		if (isObject) {
			objectKeys.push({key, object: true});
		} else {
			otherKeys.push({key, object: false});
		}
	}

	return [...priorityKeys, ...otherKeys, ...objectKeys];
}

function lineCount(str: string): number {
	return str.split("\n").length;
}

function lineCountCompare(a: string, b: string): number {
	return lineCount(a) - lineCount(b);
}

function formatObjectLabel(label: StaticMarkup): StaticMarkup {
	return markupTag("color", label, {fg: "cyan"});
}

function formatObject(
	label: StaticMarkup,
	obj: Objectish,
	opts: FormatOptions,
	labelKeys: Array<string>,
): AnyMarkup {
	// Detect circular references, and create a pointer to the specific value
	const {stack} = opts;
	if (stack.length > 0 && stack.includes(obj)) {
		label = markup`Circular ${label} ${String(stack.indexOf(obj))}`;
		return formatObjectLabel(label);
	}

	const customFormat = obj[CUSTOM_PRETTY_FORMAT];
	if (opts.allowCustom && typeof customFormat === "function") {
		return markupTag("dim", markup`${String(customFormat.call(obj))}`);
	}

	//
	const nextOpts: FormatOptions = {
		...opts,
		stack: [...stack, obj],
		depth: opts.depth + 1,
	};
	const {ignoreKeys, props} = getExtraObjectProps(obj, nextOpts);

	// For props that have object values, we always put them at the end, sorted by line count
	const objProps: Array<AnyMarkup> = [];

	// Get string props
	for (const {key, object} of sortKeys(obj)) {
		const val = obj[key];
		if (key in ignoreKeys && ignoreKeys[key] === val) {
			continue;
		}

		if (opts.compact && val === undefined) {
			continue;
		}

		// Ignore any properties already displayed in the label
		if (labelKeys.includes(key)) {
			continue;
		}

		const prop = markup`${formatKey(key)}: ${prettyFormat(val, nextOpts)}`;
		if (object) {
			objProps.push(prop);
		} else {
			props.push(prop);
		}
	}

	// Sort object props by line count and push them on
	for (const prop of objProps.sort((a, b) =>
		lineCountCompare(readMarkup(a), readMarkup(b))
	)) {
		props.push(prop);
	}

	// Get symbol props
	for (const sym of Object.getOwnPropertySymbols(obj)) {
		const val: unknown = Reflect.get(obj, sym);
		props.push(
			markup`${prettyFormat(sym, opts)}: ${prettyFormat(val, nextOpts)}`,
		);
	}

	//
	let open = "{";
	let close = "}";
	if (isIterable(obj)) {
		open = "[";
		close = "]";
	}

	//
	let inner = concatMarkup(props, markup`\n`);
	if (props.length > 1 || readMarkup(inner).includes("\n")) {
		inner = markup`\n<indent>${inner}</indent>\n`;
	}

	return markup`${formatObjectLabel(label)} ${open}${inner}${close}`;
}

function formatRegExp(val: RegExp): StaticMarkup {
	return markup`${String(val)}`;
}

function formatDate(val: Date): StaticMarkup {
	return markup`${val.toISOString()}`;
}

type Objectish = {
	type?: unknown;
	[CUSTOM_PRETTY_FORMAT]?: () => string;
	[key: string]: unknown;
	[Symbol.iterator]?: unknown;
};

function formatObjectish(val: null | Objectish, opts: FormatOptions): AnyMarkup {
	if (val === null) {
		return markupTag("emphasis", formatNull());
	}

	if (val instanceof RegExp) {
		return markupTag("color", formatRegExp(val), {fg: "red"});
	}

	if (val instanceof Date) {
		const str = formatDate(val);
		return markupTag("color", str, {fg: "magenta"});
	}

	let labelKeys: Array<string> = [];

	let label = markup`null`;

	if (val.constructor !== undefined) {
		label = markup`${val.constructor.name}`;

		if (val.constructor.name === "Object") {
			if (typeof val.type === "string") {
				label = markup`${val.type}`;
				labelKeys.push("type");
			} else if (typeof val.kind === "string") {
				label = markup`${val.kind}`;
				labelKeys.push("kind");
			}
		}
	}

	return formatObject(label, val, opts, labelKeys);
}
