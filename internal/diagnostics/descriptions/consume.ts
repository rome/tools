import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";
import {buildSuggestionAdvice} from "../helpers";
import {UnknownNumber} from "@internal/numbers";

// @internal/consume
export const consume = createDiagnosticsCategory({
	SET_PROPERTY_NON_OBJECT: {
		message: markup`Attempted to set a property on a non-object`,
	},
	EXPECTED_JSON_VALUE: {message: markup`Expected a JSON value`},
	EXPECTED_OBJECT: {message: markup`Expected object`},
	EXPECTED_ARRAY: {message: markup`Expected array`},
	EXPECTED_DATE: {message: markup`Expected a date`},
	EXPECTED_BOOLEAN: {message: markup`Expected a boolean`},
	EXPECTED_STRING: {message: markup`Expected a string`},
	EXPECTED_FUNCTION: {message: markup`Expected a function`},
	EXPECTED_BIGINT: {message: markup`Expected a bigint`},
	EXPECTED_NUMBER: {message: markup`Expected a number`},
	EXPECTED_URL: {message: markup`Expected a URL`},
	EXPECTED_VALID_NUMBER: {message: markup`Expected valid number`},
	EXPECTED_FILE_PATH: {message: markup`Expected a file path`},
	EXPECTED_ABSOLUTE_PATH: {message: markup`Expected an absolute file path`},
	EXPECTED_RELATIVE_PATH: {message: markup`Expected a relative file path`},
	EXPECTED_EXPLICIT_RELATIVE_PATH: {
		message: markup`Expected an explicit relative file path. This is one that starts with <emphasis>./</emphasis> or <emphasis>../</emphasis>`,
	},
	INVALID: {message: markup`Invalid value`},
	EXPECTED_NUMBER_BETWEEN: (min: UnknownNumber, max: UnknownNumber) => ({
		message: markup`Expected number between ${String(min.valueOf())} and ${String(
			max.valueOf(),
		)}`,
	}),
	EXPECTED_NUMBER_HIGHER: (num: UnknownNumber) => ({
		message: markup`Expected number higher than ${String(num.valueOf())}`,
	}),
	EXPECTED_NUMBER_LOWER: (num: UnknownNumber) => ({
		message: markup`Expected number lower than ${String(num.valueOf())}`,
	}),
	INVALID_NUMBER_SET_VALUE: (value: number, validValues: number[]) => ({
		message: markup`Invalid number <emphasis>${value}</emphasis>`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Possible values are`,
			},
			{
				type: "list",
				list: validValues.map((num) => markup`${num}`),
			},
		],
	}),
	INVALID_STRING_SET_VALUE: (value: string, validValues: string[]) => ({
		message: markup`Invalid value <emphasis>${value}</emphasis>`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Possible values are`,
			},
			{
				type: "list",
				list: validValues.map((str) => markup`${str}`),
			},
		],
	}),
	UNUSED_PROPERTY: (key: string, type: string, knownProperties: string[]) => ({
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
