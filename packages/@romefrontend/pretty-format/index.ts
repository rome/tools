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
} from "@romefrontend/typescript-helpers";
import {escapeJSString} from "@romefrontend/string-escape";
import {naturalCompare} from "@romefrontend/string-utils";
import {
	Markup,
	concatMarkup,
	markup,
	markupTag,
	markupToPlainText,
} from "@romefrontend/cli-layout";
import {joinMarkupLines} from "@romefrontend/cli-layout/format";

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
	return joinMarkupLines(markupToPlainText(prettyFormat(value)));
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

export default function prettyFormat(
	obj: unknown,
	rawOpts: FormatPartialOptions = {},
): Markup {
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
}

function isNativeFunction(val: Function): boolean {
	return val.toString().endsWith("{ [native code] }");
}

function formatSymbol(val: Symbol): Markup {
	return markup`${String(val)}`;
}

function formatString(val: string): Markup {
	return markup`${escapeJSString(
		val,
		{
			quote: '"',
		},
	)}`;
}

// This function is used by rome-json so make sure it can parse whatever you return here
export function formatNumber(val: bigint | number): Markup {
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

function formatUndefined(): Markup {
	return markup`undefined`;
}

function formatNull(): Markup {
	return markup`null`;
}

function formatBoolean(val: boolean): Markup {
	return val === true ? markup`true` : markup`false`;
}

function formatFunction(val: Function, opts: FormatOptions): Markup {
	const name = val.name === "" ? "anonymous" : val.name;
	let label = markup`Function ${name}`;

	if (isNativeFunction(val)) {
		label = markup`Native${label}`;
	}

	if (Object.keys(val).length === 0) {
		return label;
	}

	// rome-ignore lint/js/noExplicitAny
	return formatObject(label, (val as any), opts, []);
}

function getExtraObjectProps(
	obj: Objectish,
	opts: FormatOptions,
): {
	props: Array<Markup>;
	ignoreKeys: UnknownObject;
} {
	const props: Array<Markup> = [];
	const ignoreKeys: UnknownObject = {};

	if (obj instanceof Map) {
		for (const [key, val] of obj) {
			const formattedKey =
				typeof key === "string" ? formatKey(key) : prettyFormat(key, opts);
			props.push(markup`${formattedKey} => ${prettyFormat(val, opts)}`);
		}
	} else if (isIterable(obj)) {
		let i = 0;
		for (const val of obj) {
			ignoreKeys[String(i++)] = val;
			props.push(markup`${prettyFormat(val, opts)}`);
		}
	}

	return {ignoreKeys, props};
}

function formatKey(rawKey: string): Markup {
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

function formatObjectLabel(label: Markup): Markup {
	return markupTag("color", label, {fg: "cyan"});
}

function formatObject(
	label: Markup,
	obj: Objectish,
	opts: FormatOptions,
	labelKeys: Array<string>,
): Markup {
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
	const objProps: Array<Markup> = [];

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
	for (const prop of objProps.sort((a, b) => lineCountCompare(a.value, b.value))) {
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
	if (props.length > 1 || inner.value.includes("\n")) {
		inner = markup`\n<indent>${inner}</indent>\n`;
	}

	return markup`${formatObjectLabel(label)} ${open}${inner}${close}`;
}

function formatRegExp(val: RegExp): Markup {
	return markup`${String(val)}`;
}

function formatDate(val: Date): Markup {
	return markup`${val.toISOString()}`;
}

type Objectish = {
	type?: unknown;
	[CUSTOM_PRETTY_FORMAT]?: () => string;
	[key: string]: unknown;
};

function formatObjectish(val: null | Objectish, opts: FormatOptions): Markup {
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

	let label = markup`null`;

	if (val.constructor !== undefined) {
		label = markup`${val.constructor.name}`;
	}

	let labelKeys: Array<string> = [];

	// If there's a string type or kind property then use it as the label
	if (typeof val.type === "string") {
		label = markup`${val.type}`;
		labelKeys.push("type");
	} else if (typeof val.kind === "string") {
		label = markup`${val.kind}`;
		labelKeys.push("kind");
	}

	return formatObject(label, val, opts, labelKeys);
}
