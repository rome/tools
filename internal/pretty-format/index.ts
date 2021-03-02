/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	UnknownObject,
	isIterable,
	isPlainObject,
} from "@internal/typescript-helpers";
import {escapeJSString} from "@internal/string-escape";
import {naturalCompare} from "@internal/string-utils";
import {
	AnyMarkup,
	LazyMarkupFactory,
	StaticMarkup,
	joinMarkup,
	markup,
	markupTag,
} from "@internal/markup";
import {markupToJoinedPlainText} from "@internal/cli-layout";
import {Position, isPositionish, isSourceLocation} from "@internal/parser-core";
import util = require("util");

type RecursiveStack = unknown[];

type FormatOptions = {
	stack: RecursiveStack;
	depth: number;
	maxDepth: number;
	path: (number | string)[];
	insertLocator: undefined | (number | string)[];
	accurate: boolean;
	referencedStack: Set<unknown>;
};

type FormatPartialOptions = {
	maxDepth?: number;
	path?: FormatOptions["path"];
	stack?: RecursiveStack;
	referencedStack?: Set<unknown>;
	insertLocator?: FormatOptions["insertLocator"];
	accurate?: boolean;
};

const NODE_UTIL_INSPECT_CUSTOM = Symbol.for("nodejs.util.inspect.custom");

export function prettyFormatToString(
	value: unknown,
	opts?: FormatPartialOptions,
): string {
	return markupToJoinedPlainText(prettyFormat(value, opts));
}

export function pretty(strs: TemplateStringsArray, ...values: unknown[]): string {
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
	{
		maxDepth = Infinity,
		referencedStack = new Set(),
		stack = [],
		path = [],
		insertLocator = undefined,
		accurate = false,
	}: FormatPartialOptions = {},
): StaticMarkup {
	const opts: FormatOptions = {
		maxDepth,
		depth: 0,
		referencedStack,
		stack,
		path,
		insertLocator,
		accurate,
	};
	const value = formatValue(obj, opts);

	if (needsLocator(opts)) {
		return markup`<locator>${value}</locator>`;
	} else {
		return value;
	}
}

// By default we return lazy markup to avoid prettifying expensive values that never end up being printed
export default function prettyFormat(
	obj: unknown,
	opts: FormatPartialOptions = {},
): LazyMarkupFactory {
	return () => prettyFormatEager(obj, opts);
}

function formatValue(obj: unknown, opts: FormatOptions): StaticMarkup {
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
			return formatObjectish(obj as Objectish, opts);

		default:
			throw new Error("Unknown type");
	}
}

function needsLocator(opts: FormatOptions): boolean {
	const {path, insertLocator} = opts;
	if (insertLocator === undefined) {
		return false;
	}

	// Cannot possibly be the same if there's a different amount of parts
	if (path.length !== insertLocator.length) {
		return false;
	}

	// Check first and last entry as they are the most likely to trigger a negative
	if (path[0] !== insertLocator[0]) {
		return false;
	}
	if (path[path.length - 1] !== insertLocator[insertLocator.length - 1]) {
		return false;
	}

	// Verify parts
	for (let i = 0; i < path.length; i++) {
		if (path[i] !== insertLocator[i]) {
			return false;
		}
	}

	return true;
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

function formatFunction(val: Function, opts: FormatOptions): StaticMarkup {
	const name = val.name === "" ? "anonymous" : val.name;
	let type = "Function";

	if (isNativeFunction(val)) {
		type = "NativeFunction";
	}

	let label = markup`${formatLabel(type)} ${name}`;

	if (Object.keys(val).length === 0) {
		return label;
	}

	// rome-ignore lint/ts/noExplicitAny: future cleanup
	return markup`${label} ${formatObject(undefined, val as any, opts, [])}`;
}

function getExtraObjectProps(
	obj: Objectish,
	opts: FormatOptions,
): {
	props: AnyMarkup[];
	ignoreKeys: UnknownObject;
} {
	const props: AnyMarkup[] = [];
	const ignoreKeys: UnknownObject = {};

	if (isIterable(obj)) {
		// Duck typing Map check
		if (
			typeof obj.keys === "function" &&
			typeof obj.values === "function" &&
			typeof obj.size === "number"
		) {
			let i = 0;
			for (const item of obj) {
				const elemOpts: FormatOptions = {
					...opts,
					path: [...opts.path, i],
				};
				if (Array.isArray(item) && item.length === 2) {
					const [key, val] = item;
					const formattedKey =
						typeof key === "string"
							? formatKey(key, true)
							: prettyFormat(key, opts);
					props.push(markup`${formattedKey} => ${prettyFormat(val, elemOpts)}`);
				} else {
					props.push(prettyFormat(item, elemOpts));
				}
				i++;
			}
		} else {
			let i = 0;
			for (const val of obj) {
				props.push(
					prettyFormat(
						val,
						{
							...opts,
							path: [...opts.path, i],
						},
					),
				);
				ignoreKeys[String(i++)] = val;
			}
		}
	}

	return {ignoreKeys, props};
}

function formatKey(rawKey: string, forceString: boolean = false): StaticMarkup {
	// Format as a string if it contains any special characters
	if (forceString || /[^A-Za-z0-9_$]/g.test(rawKey)) {
		return markupTag("token", formatString(rawKey), {type: "string"});
	} else {
		return markup`${rawKey}`;
	}
}

// These are object keys that should always go at the top and ignore any alphabetization
// This is fairly arbitrary but should include generic identifier keys
const PRIORITIZE_KEYS = new Set(["id", "type", "kind", "key", "name", "value"]);

// Same, but we put them at the bottom
const DEPRIORITIZE_KEYS = new Set(["loc", "pos", "start", "end"]);

export function sortKeys(keys: string[]): string[] {
	const sortedKeys: Set<string> = new Set(keys.sort(naturalCompare));

	const topKeys: string[] = [];
	const middleKeys: string[] = [];
	const bottomKeys: string[] = [];

	for (const key of sortedKeys) {
		if (PRIORITIZE_KEYS.has(key)) {
			topKeys.push(key);
		} else if (DEPRIORITIZE_KEYS.has(key)) {
			bottomKeys.push(key);
		} else {
			middleKeys.push(key);
		}
	}

	return [...topKeys, ...middleKeys, ...bottomKeys];
}

function formatLabel(label: StaticMarkup): StaticMarkup {
	return markupTag("color", label, {fg: "cyan"});
}

function formatPositionValue(val: Position): StaticMarkup {
	return markup`<token type="number">${String(val.line.valueOf())}:${String(
		val.column.valueOf(),
	)}</token>`;
}

function formatObject(
	label: undefined | string,
	obj: Objectish,
	opts: FormatOptions,
	labelKeys: string[],
): StaticMarkup {
	// Detect circular references, and create a pointer to the specific value
	const {stack, referencedStack} = opts;

	const existingIndex = stack.indexOf(obj);
	if (existingIndex >= 0) {
		referencedStack.add(obj);
		return formatLabel(`Circular ${String(existingIndex)}`);
	}

	if (!opts.accurate) {
		const customFormat = obj[NODE_UTIL_INSPECT_CUSTOM];
		if (typeof customFormat === "function") {
			const customValue = customFormat.call(obj, opts.depth, {});
			let inner;
			if (typeof customValue === "string") {
				inner = markup`${customValue}`;
			} else {
				inner = prettyFormatEager(
					customValue,
					{
						stack: opts.stack,
					},
				);
			}
			return markupTag("italic", inner);
		}

		if (isPositionish(obj)) {
			const label = formatLabel(markup`Position`);
			return markup`${label} ${formatPositionValue(obj)}`;
		}

		if (isSourceLocation(obj)) {
			let inner = markup`<token type="string">${obj.path.format()}</token> ${formatPositionValue(
				obj.start,
			)}<dim>-</dim>${formatPositionValue(obj.end)}`;
			if (obj.identifierName !== undefined) {
				inner = markup`${inner} (${escapeJSString(obj.identifierName)})`;
			}

			const label = formatLabel(markup`SourceLocation`);
			return markup`${label} ${inner}`;
		}
	}

	//
	const nextOpts: FormatOptions = {
		...opts,
		stack: [...stack, obj],
		depth: opts.depth + 1,
	};
	const {ignoreKeys, props} = getExtraObjectProps(obj, nextOpts);

	// Get string props
	for (const key of sortKeys(Object.keys(obj))) {
		const val = obj[key];
		if (key in ignoreKeys && ignoreKeys[key] === val) {
			continue;
		}

		if (!opts.accurate && val === undefined) {
			continue;
		}

		// Ignore any properties already displayed in the label
		if (labelKeys.includes(key)) {
			continue;
		}

		const propOpts: FormatOptions = {
			...nextOpts,
			path: [...nextOpts.path, key],
		};

		const prop = markup`${formatKey(key)}: ${prettyFormat(val, propOpts)}`;
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

	const parts: AnyMarkup = [];

	// Hide labels for arrays and objects in compact mode
	let includeLabel = true;
	if (!opts.accurate) {
		if (Array.isArray(obj) && label === "Array") {
			includeLabel = false;
		} else if (isPlainObject(obj) && label === "Object") {
			includeLabel = false;
		}
	}
	if (includeLabel && label !== undefined) {
		parts.push(markup`${formatLabel(label)} `);
	}

	// Check if we were referenced circularly
	if (referencedStack.has(obj)) {
		parts.push(`<Ref ${String(stack.length)}>`);
	}

	parts.push(open);

	if (props.length > 0) {
		if (opts.accurate) {
			const inner = joinMarkup(props, "\n");
			parts.push(markup`\n${inner}\n`);
		} else {
			const inner = joinMarkup(props.map((prop) => markup`<li>${prop}</li>`));
			parts.push(markup`<ul joinSameLine=", ">${inner}</ul>`);
		}
	}

	parts.push(close);

	return joinMarkup(parts);
}

function formatRegExp(val: RegExp): StaticMarkup {
	return markup`${String(val)}`;
}

function formatDate(val: Date): StaticMarkup {
	return markup`${val.toISOString()}`;
}

type Objectish = {
	type?: unknown;
	[NODE_UTIL_INSPECT_CUSTOM]?: (
		depth: number,
		opts: NodeJS.InspectOptions,
	) => unknown;
	[key: string]: unknown;
	[Symbol.iterator]?: unknown;
	[Symbol.toStringTag]?: unknown;
};

function formatObjectish(
	val: null | Objectish,
	opts: FormatOptions,
): StaticMarkup {
	if (val === null) {
		return markupTag("emphasis", formatNull());
	}

	if (util.types.isRegExp(val)) {
		return markupTag("color", formatRegExp(val), {fg: "red"});
	}

	if (util.types.isDate(val)) {
		const str = formatDate(val);
		return markupTag("color", str, {fg: "magenta"});
	}

	// TODO boxed primitives
	// TODO TypedArray/ArrayBuffer/DataView
	// TODO WeakSet/WeakMap
	// TODO prototypes that differ from constructor
	// TODO promise
	// TODO proxies

	let labelKeys: string[] = [];

	let label = "null";

	if (typeof val[Symbol.toStringTag] === "string") {
		label = String(val[Symbol.toStringTag]);
	} else if (val.constructor !== undefined) {
		label = String(val.constructor.name);

		if (val.constructor.name === "Object" && !opts.accurate) {
			if (typeof val.type === "string") {
				label = val.type;
				labelKeys.push("type");
			} else if (typeof val.kind === "string") {
				label = val.kind;
				labelKeys.push("kind");
			}
		}
	}

	return formatObject(label, val, opts, labelKeys);
}
