import {createDiagnosticsCategory} from "./index";
import {escapeMarkup, markup} from "@romejs/string-markup";
import {buildSuggestionAdvice} from "../helpers";
import {UnknownNumber} from "@romejs/ob1";

// @romejs/consume
export const consume = createDiagnosticsCategory({
	SET_PROPERTY_NON_OBJECT: "Attempted to set a property on a non-object",
	EXPECTED_JSON_VALUE: "Expected a JSON value",
	EXPECTED_OBJECT: "Expected object",
	EXPECTED_ARRAY: "Expected array",
	EXPECTED_DATE: "Expected a date",
	EXPECTED_BOOLEAN: "Expected a boolean",
	EXPECTED_STRING: "Expected a string",
	EXPECTED_BIGINT: "Expected a bigint",
	EXPECTED_NUMBER: "Expected a number",
	EXPECTED_URL: "Expected a URL",
	EXPECTED_VALID_NUMBER: "Expected valid number",
	EXPECTED_ABSOLUTE_PATH: "Expected an absolute file path",
	EXPECTED_RELATIVE_PATH: "Expected a relative file path",
	EXPECTED_EXPLICIT_RELATIVE_PATH: "Expected an explicit relative file path. This is one that starts with <emphasis>./</emphasis> or <emphasis>../</emphasis>",
	INVALID: "Invalid value",
	EXPECTED_NUMBER_BETWEEN: (min: UnknownNumber, max: UnknownNumber) => ({
		message: `Expected number between ${min} and ${max}`,
	}),
	EXPECTED_NUMBER_HIGHER: (num: UnknownNumber) => ({
		message: `Expected number higher than ${num}`,
	}),
	EXPECTED_NUMBER_LOWER: (num: UnknownNumber) => ({
		message: `Expected number lower than ${num}`,
	}),
	INVALID_STRING_SET_VALUE: (value: string, validValues: Array<string>) => ({
		message: markup`Invalid value <emphasis>${value}</emphasis>`,
		advice: [
			{
				type: "log",
				category: "info",
				text: "Possible values are",
			},
			{
				type: "list",
				list: validValues.map((str) => escapeMarkup(str)),
			},
		],
	}),
	UNUSED_PROPERTY: (key: string, type: string, knownProperties: Array<string>) => ({
		message: markup`Unknown <emphasis>${key}</emphasis> ${type}`,
		advice: buildSuggestionAdvice(
			key,
			knownProperties,
			{
				ignoreCase: true,
			},
		),
	}),
});
