import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";
import {buildSuggestionAdvice} from "../../helpers";

// @internal/codec-spdx-license

type UnknownLicense = {
	id: string;
	knownLicenses: string[];
	packageName: string;
	packageVersion: string;
};

type UnknowLicenseInVersion = {
	id: string;
	packageVersionInConfig: string;
	newPackageVersion: string;
	packageName: string;
};

export const spdx = createDiagnosticsCategory({
	UNKNOWN_LICENSE: (
		{
			id,
			knownLicenses,
			packageName,
			packageVersion,
		}: UnknownLicense,
	) => ({
		message: markup`Unknown license <emphasis>${id}</emphasis>`,
		advice: [
			...buildSuggestionAdvice(id, knownLicenses, {ignoreCase: true}),
			{
				type: "log",
				category: "info",
				text: markup`The <emphasis>SPDX</emphasis> registry is used to ensure valid and legal licenses. See <hyperlink target="https://spdx.org/licenses/" /> for more information.`,
			},
			{
				type: "action",
				command: "config set",
				noun: markup`Add this license to the exceptions`,
				instruction: markup`To automatically add an exception for this license, run:`,
				args: [
					`dependencies.exceptions.invalidLicenses.${id}`,
					`${packageName}@${packageVersion}`,
				],
			},
		],
	}),

	UNKNOWN_LICENSE_IN_VERSION: (
		{
			id,
			packageName,
			packageVersionInConfig,
			newPackageVersion,
		}: UnknowLicenseInVersion,
	) => ({
		message: markup`The dependency <emphasis>${packageName}@${newPackageVersion}</emphasis> doesn't satisfy the version inside your configuration file <emphasis>(${packageVersionInConfig})</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`To automatically update the exception for this license, run:`,
			},
			{
				type: "action",
				instruction: markup`To automatically add an exception for this license, run:`,
				noun: markup`Fix invalid licenses`,
				command: "config push",
				args: [
					`dependencies.exceptions.invalidLicenses.${id}`,
					`${packageName}@${newPackageVersion}`,
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
