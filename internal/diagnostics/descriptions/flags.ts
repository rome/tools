import {createDiagnosticsCategory} from "./index";
import {StaticMarkup, markup} from "@internal/markup";
import {toKebabCase} from "@internal/string-utils";
import {DiagnosticAdvice} from "../types";

export const flags = createDiagnosticsCategory({
	UNSUPPORTED_SHORTHAND: (flag: string) => ({
		message: markup`The <emphasis>-${flag}</emphasis> flag is not a valid shorthand flag`,
	}),
	INCORRECT_CASED_FLAG: (flag: string) => ({
		message: markup`Incorrect cased flag name`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Use <emphasis>${toKebabCase(flag)}</emphasis> instead`,
			},
		],
	}),
	INCORRECT_ARG_COUNT: (excessive: boolean, text: StaticMarkup) => ({
		message: excessive ? markup`Too many arguments` : markup`Missing arguments`,
		advice: [
			{
				type: "log",
				category: "info",
				text,
			},
		],
	}),
	DISALLOWED_REVIEW_FLAG: (key: string) => ({
		message: markup`Flag <emphasis>${key}</emphasis> is not allowed with <emphasis>review</emphasis>`,
	}),
	DISALLOWED_REQUEST_FLAG: (key: string) => ({
		message: markup`This command does not support the <emphasis>${key}</emphasis> flag`,
	}),
	UNKNOWN_SUBCOMMAND: (action: string) => ({
		message: markup`Unknown subcommand ${action}`,
	}),
	NO_FILES_FOUND: (noun: undefined | string) => ({
		message: noun === undefined
			? markup`No files found`
			: markup`No files to ${noun} found`,
	}),
	UNKNOWN_COMMAND: (
		{
			programName,
			commandName,
			suggestedName,
			suggestedDescription,
			suggestedCommand,
		}: {
			programName: string;
			commandName: string;
			suggestedName: undefined | string;
			suggestedDescription: undefined | StaticMarkup;
			suggestedCommand: undefined | string;
		},
	) => {
		const advice: DiagnosticAdvice = [];

		if (suggestedName !== undefined) {
			const description =
				suggestedDescription === undefined
					? ""
					: markup` ${suggestedDescription}`;
			advice.push({
				type: "log",
				category: "info",
				text: markup`Did you mean <emphasis>${suggestedName}</emphasis> instead?${description}`,
			});
		}

		if (suggestedCommand !== undefined) {
			advice.push({
				type: "code",
				language: "shell",
				sourceText: suggestedCommand,
			});
		}

		return {
			category: "flags/invalid",
			message: commandName === ""
				? markup`No command specified`
				: markup`Unknown command <emphasis>${commandName}</emphasis>`,
			advice: [
				...advice,
				{
					type: "log",
					category: "info",
					text: markup`To see all available commands run`,
				},
				{
					type: "code",
					language: "shell",
					sourceText: `${programName} --help`,
				},
			],
		};
	},
});
