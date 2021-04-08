/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Comments,
	ConfigCommentMap,
} from "../types";
import {Consumer} from "@internal/consume";
import {sortKeys} from "@internal/pretty-format";
import {escapeJSString} from "@internal/string-escape";
import {isValidWord} from "@internal/codec-config/util";
import StringifyHelper, {createStringifyHelper} from "../StringifyHelper";

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

export function stringifyComments(comments: Comments): string[] {
	return comments.map((node) => {
		if (node.type === "BlockComment") {
			return `/*${node.value}*/`;
		} else {
			// node.type === 'LineComment'
			return `//${node.value}`;
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
			return String(value);
	}

	return undefined;
}

function sortMap(map: Map<string, Consumer>): Map<string, Consumer> {
	const sortedMap: Map<string, Consumer> = new Map();
	const sortedKeys = sortKeys(Array.from(map.keys()));

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

function stringifyArray(consumer: Consumer, helper: StringifyHelper) {
	let inner: string[] = [];

	const arr = consumer.asIterable();
	const elemHelper = helper.fork();
	for (const consumer of arr) {
		// Add element comments
		const comments = helper.getComments(consumer).outer;
		inner = inner.concat(stringifyComments(comments));

		// Add the actual element line
		inner.push(stringifyValue(consumer, elemHelper));
	}

	// Add inner comments
	const innerComments = helper.getComments(consumer).inner;
	inner = inner.concat(stringifyComments(innerComments));

	return elemHelper.wrap("[", inner, "]");
}

function stringifyPlainObject(
	consumer: Consumer,
	helper: StringifyHelper,
): string {
	// Must be an object if we failed all the other conditions
	let buff: string[] = [];
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

	// Build properties
	const propHelper = helper.fork();
	for (const [key, consumer] of sortMap(map)) {
		// Add property comments
		const comments = helper.getComments(consumer).outer;
		buff = buff.concat(stringifyComments(comments));

		// Add the actual property line
		const propKey = stringifyKey(key);
		const propValue = stringifyValue(
			consumer,
			propHelper,
		);
		buff.push(`${propKey}: ${propValue},`);
	}

	// Add inner comments
	const innerComments = helper.getComments(consumer).inner;
	buff = buff.concat(stringifyComments(innerComments));

	return propHelper.wrap("{", buff, "}");
}

function stringifyObject(
	consumer: Consumer,
	value: unknown,
	helper: StringifyHelper,
) {
	const {stack} = helper.options;

	try {
		stack.add(value);

		if (Array.isArray(value)) {
			return stringifyArray(consumer, helper);
		} else {
      return stringifyPlainObject(consumer, helper);
    }
	} finally {
		stack.delete(value);
	}
}

export function stringifyJSON5RootConsumer(
	consumer: Consumer,
	pathToComments: ConfigCommentMap,
): string {
	const helper = createStringifyHelper(pathToComments);

	// Nothing else handles comments at the top level
	const inner = stringifyValue(consumer, helper);
	const comments = helper.getComments(consumer);
	const outer = stringifyComments(comments.outer);

	return [...outer, inner].join("\n");
}

function stringifyValue(consumer: Consumer, helper: StringifyHelper): string {
	const value = consumer.asUnknown();

	// Stringify primitives
	const asPrim = stringifyPrimitives(value);
	if (asPrim !== undefined) {
		return asPrim;
	}

	// Check if we're already stringfying this value to prevent recursion
	if (helper.options.stack.has(value)) {
		throw new TypeError("Recursive");
	}

	return stringifyObject(consumer, value, helper);
}
