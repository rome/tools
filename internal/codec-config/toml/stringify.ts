import {Consumer} from "@internal/consume";
import {ConfigCommentMap} from "@internal/codec-config";
import {EscapeStringQuoteChar, escapeJSString} from "@internal/string-escape";
import {isValidWordKey} from "./tokenizer";
import {Comments, PathComments} from "../types";
import StringifyHelper, {createStringifyHelper} from "../StringifyHelper";
import {isObject} from "@internal/typescript-helpers";

function stringifyArray(consumer: Consumer, helper: StringifyHelper): string {
	const buff: string[] = [];
	stringifyComments(helper.getComments(consumer).inner, buff);

	const innerHelper = helper.fork();

	for (const elem of consumer.asIterable()) {
		stringifyPropComments(helper.getComments(elem), buff, elem.asUnknown());
		buff.push(`${stringifyValue(elem, innerHelper, true)},`);
	}

	return helper.wrap("[", buff, "]");
}

function stringifyKey(key: string): string {
	if (isValidWordKey(key)) {
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

function stringifyPrimitives(consumer: Consumer): undefined | string {
	const value = consumer.asUnknown();

	// Basic primitive types
	switch (typeof value) {
		case "symbol":
		case "function":
		case "undefined":
			return "null";

		case "boolean":
			return String(value);

		// TODO
		case "number":
			return String(value);

		case "string": {
			let quote: EscapeStringQuoteChar = value.includes('"') ? "'" : '"';
			if (value.includes("\n") || value.includes(quote)) {
				quote = quote === '"' ? '"""' : "'''";
			}
			return escapeJSString(
				value,
				{
					quote,
					json: true,
					ignoreWhitespaceEscapes: true,
				},
			);
		}

		case "bigint":
			// This is the actual V8 message lol
			throw new Error("Do not know how to serialize a BigInt");
	}

	return undefined;
}

function stringifyValue(
	consumer: Consumer,
	helper: StringifyHelper,
	inline: boolean,
): string {
	const asPrim = stringifyPrimitives(consumer);
	if (asPrim !== undefined) {
		return asPrim;
	}

	return stringifyObject(consumer, helper, inline);
}

function stringifyComments(comments: Comments, buff: string[]): void {
	for (const comment of comments) {
		for (const line of comment.value.split("\n")) {
			buff.push(`#${line}`);
		}
	}
}

function stringifyPropComments(
	comments: PathComments,
	buff: string[],
	value: unknown,
): void {
	stringifyComments(comments.outer, buff);

	if (!(isObject(value) || Array.isArray(value))) {
		stringifyComments(comments.inner, buff);
	}
}

function isArrayOfObjects(arr: unknown): arr is unknown[] {
	if (!Array.isArray(arr)) {
		return false;
	}

	if (arr.length === 0) {
		return false;
	}

	for (const elem of arr) {
		if (!isObject(elem)) {
			return false;
		}
	}

	return true;
}

function printSingleObjects(val: unknown): undefined | [string, unknown] {
	if (isObject(val)) {
		const keys = Object.keys(val);
		if (keys.length === 1) {
			const key = keys[0];
			const prop = val[key];
			const converted = printSingleObjects(prop);
			if (converted === undefined) {
				return [key, prop];
			} else {
				return [`${key}.${converted[0]}`, converted[1]];
			}
		}
	}

	return undefined;
}

function stringifyPlainObject(
	consumer: Consumer,
	helper: StringifyHelper,
	inline: boolean,
): string {
	const map = consumer.asMap();
	let buff: string[] = [];
	stringifyComments(helper.getComments(consumer).inner, buff);

	let buffTables: string[] = [];

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

	const innerHelper = inline ? helper.fork() : helper;

	for (const [key, consumer] of map) {
		let propKey = stringifyKey(key);
		const possibleValue = consumer.asUnknown();
		const comments = helper.getComments(consumer);

		const single = printSingleObjects(possibleValue);

		if (single === undefined && !inline) {
			if (isObject(possibleValue)) {
				stringifyPropComments(comments, buffTables, possibleValue);

				buffTables.push(`[${propKey}]`);
				buffTables.push(stringifyPlainObject(consumer, helper, false));
				continue;
			} else if (isArrayOfObjects(possibleValue)) {
				stringifyPropComments(comments, buffTables, possibleValue);
				for (const elem of consumer.asIterable()) {
					buffTables.push(`[[${propKey}]]`);
					buffTables.push(stringifyPlainObject(elem, helper, false));
				}
				continue;
			}
		}

		let propValue;
		if (single === undefined) {
			propValue = stringifyValue(consumer, innerHelper, true);
		} else {
			propKey = `${propKey}.${single[0]}`;
			propValue = single[1];
		}

		let item = `${propKey} = ${propValue}`;
		if (inline) {
			item += ",";
		}
		buff.push(item);
	}

	if (inline) {
		return helper.wrap("{", buff, "}");
	} else {
		return [...buff, ...buffTables].join("\n");
	}
}

function stringifyObject(
	consumer: Consumer,
	helper: StringifyHelper,
	inline: boolean,
): string {
	const value = consumer.asUnknown();

	if (Array.isArray(value)) {
		return stringifyArray(consumer, helper);
	}

	return stringifyPlainObject(consumer, helper, inline);
}

export function stringifyTOMLFromConsumer(
	consumer: Consumer,
	pathToComments: ConfigCommentMap,
): string {
	const helper = createStringifyHelper(pathToComments);
	return stringifyPlainObject(consumer, helper, false);
}
