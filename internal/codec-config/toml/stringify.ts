import {ConsumePath, Consumer} from "@internal/consume";
import {ConfigCommentMap} from "@internal/codec-config";
import {EscapeStringQuoteChar, escapeJSString} from "@internal/string-escape";
import {isValidWordKey} from "./tokenizer";
import {Comments, PathComments} from "../types";
import StringifyHelper, {createStringifyHelper} from "../StringifyHelper";
import {isObject} from "@internal/typescript-helpers";
import {humanizeNumber} from "@internal/numbers";

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

function stringifyKeys(keys: ConsumePath): string {
	return keys.map((key) => stringifyKey(String(key))).join(".");
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

		case "number":
			return humanizeNumber(value);

		case "string": {
			// TODO heuristics for when to use literal strings
			let quote: EscapeStringQuoteChar = '"';
			let str = value;
			if (value.includes("\n")) {
				quote = '"""';
				str = `\n${value}`;
			}
			return escapeJSString(
				str,
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

function stringifyRoot(consumer: Consumer, helper: StringifyHelper): string {
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

	for (const [key, consumer] of map) {
		let propKey = stringifyKey(key);
		const possibleValue = consumer.asUnknown();
		const comments = helper.getComments(consumer);

		const reducedProp = printSingleObjects(possibleValue);

		if (reducedProp === undefined) {
			if (isObject(possibleValue)) {
				stringifyPropComments(comments, buffTables, possibleValue);

				buffTables.push(`[${stringifyKeys(consumer.keyPath)}]`);
				buffTables.push(stringifyRoot(consumer, helper));
				buffTables.push("");
				continue;
			} else if (isArrayOfObjects(possibleValue)) {
				stringifyPropComments(comments, buffTables, possibleValue);
				for (const elem of consumer.asIterable()) {
					buffTables.push(`[[${stringifyKeys(consumer.keyPath)}]]`);
					buffTables.push(stringifyRoot(elem, helper));
					buffTables.push("");
				}
				continue;
			}
		}

		let propValue;
		if (reducedProp === undefined) {
			propValue = stringifyValue(consumer, helper, true);
		} else {
			propKey = `${propKey}.${reducedProp[0]}`;
			propValue = reducedProp[1];
		}

		buff.push(`${propKey} = ${propValue}`);
	}

	return [...buff, ...buffTables].join("\n");
}

function stringifyObject(consumer: Consumer, helper: StringifyHelper): string {
	const value = consumer.asUnknown();

	if (Array.isArray(value)) {
		return stringifyArray(consumer, helper);
	}

	throw new Error("Regular objects should have been printed as tables");
}

export function stringifyTOMLFromConsumer(
	consumer: Consumer,
	pathToComments: ConfigCommentMap,
): string {
	const helper = createStringifyHelper(pathToComments);
	return stringifyRoot(consumer, helper);
}
