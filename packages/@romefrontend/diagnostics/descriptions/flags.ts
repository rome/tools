import {createDiagnosticsCategory} from "./index";
import {markup} from "@romefrontend/string-markup";
import {toKebabCase} from "@romefrontend/string-utils";
import {buildSuggestionAdvice} from "../helpers";

export const flags = createDiagnosticsCategory({
	UNSUPPORTED_SHORTHANDS: `Shorthand flags are not supported`,
	INCORRECT_CASED_FLAG: (flag: string) => ({
		message: "Incorrect cased flag name",
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Use <emphasis>${toKebabCase(flag)}</emphasis> instead`,
			},
		],
	}),
	INCORRECT_ARG_COUNT: (excessive: boolean, message: string) => ({
		message: excessive ? "Too many arguments" : "Missing arguments",
		advice: [
			{
				type: "log",
				category: "info",
				text: message,
			},
		],
	}),
	DISALLOWED_REVIEW_FLAG: (key: string) => ({
		message: `Flag <emphasis>${key}</emphasis> is not allowed with <emphasis>review</emphasis>`,
	}),
	DISALLOWED_REQUEST_FLAG: (key: string) => ({
		message: `This command does not support the <emphasis>${key}</emphasis> flag`,
	}),
	UNKNOWN_ACTION: (action: string) => ({
		message: `Unknown action ${action}`,
	}),
	NO_FILES_FOUND: (noun: undefined | string) => ({
		message: noun === undefined ? "No files found" : `No files to ${noun} found`,
	}),
	UNKNOWN_COMMAND_SUGGESTED: (
		unknownCommandName: string,
		commandName: string,
		description: undefined | string,
		command: string,
	) => ({
		category: "flags/invalid",
		message: markup`Unknown command <emphasis>${unknownCommandName}</emphasis>`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Did you mean <emphasis>${commandName}</emphasis> instead?` +
				(description === undefined ? "" : ` ${description}`),
			},
			{
				type: "code",
				language: "shell",
				code: command,
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
			? "No command specified"
			: markup`Unknown command <emphasis>${commandName}</emphasis>`,
		advice: [
			...buildSuggestionAdvice(commandName, knownCommands),
			{
				type: "log",
				category: "info",
				text: "To see available commands run",
			},
			{
				type: "command",
				command: `${programName} --help`,
			},
		],
	}),
});
