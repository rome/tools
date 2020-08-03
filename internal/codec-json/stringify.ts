/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Comments, PathComments, RJSONCommentMap} from "./types";
import {naturalCompare} from "@internal/string-utils";
import {isValidWord} from "./parse";
import {Consumer} from "@internal/consume";
import {PRIORITIZE_KEYS, formatNumber} from "@internal/pretty-format";
import {escapeJSString} from "@internal/string-escape";
import {markupToJoinedPlainText} from "@internal/cli-layout";

function joinList(
	open: string,
	close: string,
	indent: string,
	items: Array<string>,
) {
	if (items.length === 0) {
		return open + close;
	} else {
		return [open, ...items, indent + close].join("\n");
	}
}

function stringifyKey(key: string): string {
	if (isValidWord(key)) {
		// A property key doesn't need quotes if it's a valid word
		return key;
	} else {
		return escapeJSString(
			key,
			{
				quote: '"',
				ignoreWhitespaceEscapes: true,
				json: true,
			},
		);
	}
}

export function stringifyComments(
	indent: string,
	comments: Comments,
): Array<string> {
	return comments.map((node) => {
		if (node.type === "BlockComment") {
			return `${indent}/*${node.value}*/`;
		} else {
			// node.type === 'LineComment'
			return `${indent}//${node.value}`;
		}
	});
}

function stringifyPrimitives(value: unknown): undefined | string {
	if (value === null) {
		return "null";
	}

	// Coerce primitive objects to their primitive form, as specified in ECMA262 24.5.2.1
	if (
		value instanceof Number ||
		value instanceof String ||
		value instanceof Boolean
	) {
		value = value.valueOf();
	}

	// Basic primitive types
	switch (typeof value) {
		case "symbol":
		case "function":
		case "undefined":
			return "null";

		case "boolean":
			return value ? "true" : "false";

		case "string":
			return escapeJSString(
				value,
				{
					quote: '"',
					json: true,
					ignoreWhitespaceEscapes: true,
				},
			);

		case "bigint":
			// This is the actual V8 message lol
			throw new Error("Do not know how to serialize a BigInt");

		case "number":
			return markupToJoinedPlainText(formatNumber(value));
	}

	return undefined;
}

function sortMapKeys(map: Map<string, unknown>): Set<string> {
	return new Set(Array.from(map.keys()).sort(naturalCompare));
}

function sortMap(map: Map<string, Consumer>): Map<string, Consumer> {
	const sortedMap: Map<string, Consumer> = new Map();
	const sortedKeys: Set<string> = sortMapKeys(map);

	// Add any prioritized keys so they're before anything alphabetized
	for (const key of PRIORITIZE_KEYS) {
		if (sortedKeys.has(key)) {
			sortedKeys.delete(key);

			const val = map.get(key);
			if (val === undefined) {
				throw new Error("Expected value");
			}

			sortedMap.set(key, val);
		}
	}

	// Now add the rest
	for (const key of sortedKeys) {
		const val = map.get(key);
		if (val === undefined) {
			throw new Error("Expected value");
		}

		sortedMap.set(key, val);
	}

	return sortedMap;
}

type StringifyOptions = {
	comments: RJSONCommentMap;
	isTopLevel: boolean;
	level: number;
	stack: Set<unknown>;
};

type StringifyObjectOptions = StringifyOptions & {
	prevIndent: string;
	nextIndent: string;
};

function getComments(consumer: Consumer, opts: StringifyOptions): PathComments {
	const comments = opts.comments.get(consumer.keyPath.join("."));
	if (comments === undefined) {
		return {
			inner: [],
			outer: [],
		};
	} else {
		return comments;
	}
}

function stringifyArray(consumer: Consumer, info: StringifyObjectOptions) {
	const {level, prevIndent, nextIndent, stack} = info;

	let buff: Array<string> = [];

	const arr = consumer.asIterable();
	for (const consumer of arr) {
		// Add element comments
		const comments = getComments(consumer, info).outer;
		buff = buff.concat(stringifyComments(nextIndent, comments));

		// Add the actual element line
		const element = stringifyConsumer(
			consumer,
			{
				comments: info.comments,
				isTopLevel: false,
				level: level + 1,
				stack,
			},
		);
		buff.push(`${nextIndent}${element}`);
	}

	// Add inner comments
	const innerComments = getComments(consumer, info).inner;
	buff = buff.concat(stringifyComments(nextIndent, innerComments));

	return joinList("[", "]", prevIndent, buff);
}

function stringifyPlainObject(
	consumer: Consumer,
	info: StringifyObjectOptions,
): string {
	const {level, prevIndent, stack, isTopLevel} = info;
	let {nextIndent} = info;

	// Must be an object if we failed all the other conditions
	let buff: Array<string> = [];
	const map = consumer.asMap();

	// Remove function, symbol, and undefined properties
	for (const [key, consumer] of map) {
		const value = consumer.asUnknown();

		if (
			typeof value === "function" ||
			typeof value === "undefined" ||
			typeof value === "symbol"
		) {
			map.delete(key);
		}
	}

	let propLevel = level + 1;

	// We only want to increase the level for properties when we aren't at the top
	if (isTopLevel && level === 0) {
		propLevel = 0;
		nextIndent = "";
	}

	// Build properties
	for (const [key, consumer] of sortMap(map)) {
		// Add property comments
		const comments = getComments(consumer, info).outer;
		buff = buff.concat(stringifyComments(nextIndent, comments));

		// Add the actual property line
		const propKey = stringifyKey(key);
		const propValue = stringifyConsumer(
			consumer,
			{
				comments: info.comments,
				isTopLevel: false,
				level: propLevel,
				stack,
			},
		);
		buff.push(`${nextIndent}${propKey}: ${propValue}`);
	}

	// We track this so we know whether we can safely put everything at the top level

	// If we only have comments then there's no way the parser could infer it was originally an object
	const hasProps = buff.length > 0;

	// Add inner comments
	const innerComments = getComments(consumer, info).inner;
	buff = buff.concat(stringifyComments(nextIndent, innerComments));

	if (level === 0 && isTopLevel) {
		if (hasProps) {
			return buff.join("\n");
		} else if (buff.length > 0) {
			// Otherwise we just have a bunch of comments
			// Indent them correctly and just output it as a normal object
			buff = buff.map((str) => {
				return `  ${str}`;
			});
		}
	}

	return joinList("{", "}", prevIndent, buff);
}

function stringifyObject(
	consumer: Consumer,
	value: unknown,
	opts: StringifyOptions,
) {
	const {isTopLevel, level, stack} = opts;

	const info: StringifyObjectOptions = {
		comments: opts.comments,
		isTopLevel,
		nextIndent: "\t".repeat(level + 1),
		prevIndent: level === 0 ? "" : "\t".repeat(level - 1),
		level,
		stack,
	};

	try {
		stack.add(value);

		if (Array.isArray(value) || value instanceof Set) {
			return stringifyArray(consumer, info);
		}

		return stringifyPlainObject(consumer, info);
	} finally {
		stack.delete(value);
	}
}

export function stringifyRootConsumer(
	consumer: Consumer,
	pathToComments: RJSONCommentMap,
): string {
	const opts: StringifyOptions = {
		comments: pathToComments,
		isTopLevel: true,
		level: 0,
		stack: new Set(),
	};

	// Nothing else handles comments at the top level
	const inner = stringifyConsumer(consumer, opts);
	const comments = getComments(consumer, opts);
	const outer = stringifyComments("", comments.outer);

	return [...outer, inner].join("\n");
}

function stringifyConsumer(consumer: Consumer, opts: StringifyOptions): string {
	const value = consumer.asUnknown();

	// Stringify primitives
	const asPrim = stringifyPrimitives(value);
	if (asPrim !== undefined) {
		return asPrim;
	}

	// Check if we're already stringfying this value to prevent recursion
	if (opts.stack.has(value)) {
		throw new TypeError("Recursive");
	}

	return stringifyObject(consumer, value, opts);
}
