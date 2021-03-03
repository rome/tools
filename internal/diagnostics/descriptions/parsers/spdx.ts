import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";
import {buildSuggestionAdvice} from "../../helpers";
import {SPDXLicenseParserExceptions} from "@internal/codec-spdx-license";
import {DiagnosticAdvice} from "@internal/diagnostics/types";

// @internal/codec-spdx-license

type UnknownLicenseArg = {
	id: string;
	knownLicenses: string[];
	exceptions?: SPDXLicenseParserExceptions;
};

type UnknowLicensePresentUnsatisfiedExceptionArg = {
	id: string;
	packageVersionInConfig: string;
	packageName: string;
	packageVersion: string;
};

export const spdx = createDiagnosticsCategory({
	UNKNOWN_LICENSE: (
		{
			id,
			knownLicenses,
			exceptions,
		}: UnknownLicenseArg,
	) => {
		const advice: DiagnosticAdvice[] = [
			...buildSuggestionAdvice(id, knownLicenses, {ignoreCase: true}),
			{
				type: "log",
				category: "info",
				text: markup`The <emphasis>SPDX</emphasis> registry is used to ensure valid and legal licenses. See <hyperlink target="https://spdx.org/licenses/" /> for more information.`,
			},
		];

		if (exceptions !== undefined) {
			advice.push({
				type: "action",
				command: "config set",
				description: markup`Add this license as an exception`,
				args: [
					`dependencies.exceptions.invalidLicenses.${id}`,
					`${exceptions.packageName}@${exceptions.packageVersion}`,
				],
			});
		}

		return {
			message: markup`Unknown license <emphasis>${id}</emphasis>`,
			advice,
		};
	},

	UNKNOWN_LICENSE_PRESENT_UNSATISFIED_EXCEPTION: (
		{
			id,
			packageVersionInConfig,
			packageName,
			packageVersion,
		}: UnknowLicensePresentUnsatisfiedExceptionArg,
	) => ({
		message: markup`The dependency <emphasis>${packageName}@${packageVersion}</emphasis> doesn't satisfy the version inside your configuration file <emphasis>(${packageVersionInConfig})</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`To automatically update the exception for this license, run:`,
			},
			{
				type: "action",
				description: markup`Add an exception for this license`,
				command: "config push",
				args: [
					`dependencies.exceptions.invalidLicenses.${id}`,
					`${packageName}@${packageVersion}`,
				],
			},
		],
	}),
	VALID_LICENSE_WITH_MISSING_DASH: (possibleCorrectLicense: string) => ({
		message: markup`Missing dash between license name and version`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Did you mean <emphasis>${possibleCorrectLicense}</emphasis>?`,
			},
		],
	}),
	WITH_RIGHT_LICENSE_ONLY: {
		message: markup`Only a license id can be on the right side of a WITH`,
	},
	OPERATOR_NOT_BETWEEN_EXPRESSION: {
		message: markup`Can only use AND/OR in between an expression`,
	},
	PLUS_NOT_AFTER_LICENSE: {
		message: markup`A plus can only come after a license id`,
	},
	UNOPENED_PAREN: {message: markup`Nothing open to close`},
});
