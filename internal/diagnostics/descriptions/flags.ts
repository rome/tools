import {createDiagnosticsCategory} from "./index";
import {StaticMarkup, markup} from "@internal/markup";
import {toKebabCase} from "@internal/string-utils";
import {buildSuggestionAdvice} from "../helpers";

export const flags = createDiagnosticsCategory({
	UNSUPPORTED_SHORTHANDS: {message: markup`Shorthand flags are not supported`},
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
	UNKNOWN_ACTION: (action: string) => ({
		message: markup`Unknown action ${action}`,
	}),
	NO_FILES_FOUND: (noun: undefined | string) => ({
		message: noun === undefined
			? markup`No files found`
			: markup`No files to ${noun} found`,
	}),
	UNKNOWN_COMMAND_SUGGESTED: (
		unknownCommandName: string,
		commandName: string,
		description: undefined | StaticMarkup,
		command: string,
	) => ({
		category: "flags/invalid",
		message: markup`Unknown command <emphasis>${unknownCommandName}</emphasis>`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Did you mean <emphasis>${commandName}</emphasis> instead? ${description ===
				undefined
					? ""
					: markup` ${description}`}`,
			},
			{
				type: "code",
				language: "shell",
				sourceText: command,
			},
		],
	}),
	COMMAND_REQUIRED: (
		programName: string,
		commandName: string,
		knownCommands: Array<string>,
	) => ({
		category: "flags/invalid",
		message: commandName === ""
			? markup`No command specified`
			: markup`Unknown command <emphasis>${commandName}</emphasis>`,
		advice: [
			...buildSuggestionAdvice(commandName, knownCommands),
			{
				type: "log",
				category: "info",
				text: markup`To see available commands run`,
			},
			{
				type: "command",
				command: `${programName} --help`,
			},
		],
	}),
});
