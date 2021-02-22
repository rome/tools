import {createDiagnosticsCategory} from "@internal/diagnostics";
import {markup} from "@internal/markup";

export const browserquery = createDiagnosticsCategory({
	EXPECTED_OPERATOR_OR_VERSION: {
		message: markup`Expected an operator such as <emphasis>></emphasis> <emphasis><</emphasis> <emphasis>>=</emphasis> <emphasis><=</emphasis> or a version`
	},
	EXPECTED_VERSION: {
		message: markup`Expected a browser version`
	},
	EXPECTED_NUMBER: {
		message: markup`Expected a number`
	},
	EXPECTED_PERCENTAGE: {
		message: markup`Expected a percentage`
	},
	EXPECTED_REGION: {
		message: markup`Expected a region`
	},
	EXPECTED_DATE: {
		message: markup`Expected a date such as <emphasis>2021-02-21</emphasis>`
	},
	EXPECTED_UNIT: {
		message: markup`Expected a unit such as <emphasis>years</emphasis>, <emphasis>months</emphasis>, <emphasis>days</emphasis>, <emphasis>versions</emphasis> or <emphasis>major versions</emphasis>`
	},
	EXPECTED_NEW_QUERY: {
		message: markup`Expected a new browser query`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Available browser queries can be found at <hyperlink target="https://github.com/rome/tools/blob/main/internal/codec-browsers/README.md"/>`,
			},
		],
	},
	AND_WITHOUT_TARGET: {
		message: markup`Expected a target before <emphasis>and</emphasis>`
	}
});
