import {Consumer} from "@internal/consume";
import {ConfigCommentMap} from "@internal/codec-config";
import {EscapeStringQuoteChar, escapeJSString} from "@internal/string-escape";
import {isValidWordKey} from "./tokenizer";
import StringifyHelper, {createStringifyHelper} from "../StringifyHelper";

function stringifyArray(consumer: Consumer, helper: StringifyHelper, inline: boolean): string {
	const buff: string[] = [];

	for (const elem of consumer.asIterable()) {
		buff.push(`${stringifyValue(elem, helper, true)},`);
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
			if (value.includes("\n")) {
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

function stringifyValue(consumer: Consumer, helper: StringifyHelper, inline: boolean): string {
	const asPrim = stringifyPrimitives(consumer);
	if (asPrim !== undefined) {
		return asPrim;
	}

	return stringifyObject(consumer, helper, inline);
}

function stringifyPlainObject(
	consumer: Consumer,
	helper: StringifyHelper,
	inline: boolean,
): string {
	const map = consumer.asMap();
	let buff: string[] = [];

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
		const propKey = stringifyKey(key);
		const possibleValue = consumer.asUnknown();
		if (typeof possibleValue === "object" && !Array.isArray(possibleValue)) {
			buff.push(`[${propKey}]`);
			const element = stringifyValue(consumer, innerHelper, true);
			buff.push(`${element}`);
		} else {
			const propValue = stringifyValue(consumer, innerHelper, true);
			buff.push(`${propKey} = ${propValue}`);
		}
	}

	if (inline) {
		return helper.wrap(`{`, buff, `}`);
	} else {
		return `${buff.join("\n")}`;
	}
}

function stringifyObject(consumer: Consumer, helper: StringifyHelper, inline: boolean): string {
	const value = consumer.asUnknown();

	if (Array.isArray(value)) {
		return stringifyArray(consumer, helper, inline);
	}

	return stringifyPlainObject(consumer, helper, inline);
}

export function stringifyTOMLFromConsumer(
	consumer: Consumer,
	pathToComments: ConfigCommentMap,
): string {
	const helper = createStringifyHelper(pathToComments);
	return stringifyValue(consumer, helper, false);
}
