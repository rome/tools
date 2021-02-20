import {Consumer} from "@internal/consume";
import {ConfigCommentMap} from "@internal/codec-config";
import {escapeJSString} from "@internal/string-escape";
import {isValidWord} from "@internal/codec-config/util";

function convertArray(consumer: Consumer, value: unknown): string {
	value;
	let buff: Array<string | boolean | number> = [];
	const arr = consumer.asIterable();
	for (const consumer of arr) {
		const element = stringifyConsumer(consumer);
		buff.push(element);
	}

	return ["[", buff.join(", "), "]"].join("");
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

function stringifyPrimitives(
	value: unknown,
): undefined | string | boolean | number {
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
			return value;

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
			return value;
	}

	return undefined;
}

function stringifyConsumer(consumer: Consumer): string | boolean | number {
	const value = consumer.asUnknown();

	// Stringify primitives
	const asPrim = stringifyPrimitives(value);
	if (asPrim !== undefined) {
		return asPrim;
	}

	return convertObject(consumer, value);
}

function convertPlainObject(consumer: Consumer, value: unknown): string {
	value;
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

	for (const [key, consumer] of map) {
		const propKey = stringifyKey(key);
		const possibleValue = consumer.getValue();
		if (typeof possibleValue === "object" && !Array.isArray(possibleValue)) {
			buff.push(`[${propKey}]`);
			const element = stringifyConsumer(consumer);
			buff.push(`${element}`);
		} else {
			const propValue = stringifyConsumer(consumer);
			buff.push(`${propKey} = ${propValue}`);
		}
	}

	return `${buff.join("\n")}`;
}

function convertObject(consumer: Consumer, value: unknown): string {
	if (Array.isArray(value) || value instanceof Set) {
		return convertArray(consumer, value);
	}
	return convertPlainObject(consumer, value);
}

export default function convertToTomlFromConsumer(
	consumer: Consumer,
	pathToComments: ConfigCommentMap,
): string {
	// trick usage
	pathToComments;
	const value = consumer.asUnknown();
	return convertObject(consumer, value);
}
