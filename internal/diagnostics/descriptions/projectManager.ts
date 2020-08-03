import {createDiagnosticsCategory} from "./index";
import {DiagnosticLocation} from "../types";
import {filePathToMarkup, markup} from "@internal/markup";
import {buildSuggestionAdvice} from "../helpers";
import {AbsoluteFilePath} from "@internal/path";

export const projectManager = createDiagnosticsCategory({
	NO_VCS: (rootConfigLocation: undefined | DiagnosticLocation) => ({
		category: "projectManager/vscMissing",
		message: markup`Can't find any version control for this project`,
		advice: rootConfigLocation === undefined
			? [
					{
						type: "log",
						category: "info",
						text: markup`Version control root was set to the project root as it was not configured. To configure a different directory run`,
					},
					{
						type: "code",
						language: "shell",
						sourceText: "rome config set-directory vcs.root DIRECTORY_HERE",
					},
				]
			: [
					{
						type: "log",
						category: "info",
						text: markup`Version control root was set here`,
					},
					{
						type: "frame",
						location: rootConfigLocation,
					},
				],
	}),
	DUPLICATE_PACKAGE: (packageName: string, existing: string) => ({
		category: "projectManager/nameCollision",
		message: markup`Duplicate package name <emphasis>${packageName}</emphasis>`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Defined already by <filelink target="${existing}" />`,
			},
		],
	}),
	NOT_FOUND: {
		category: "projectManager/missing",
		message: markup`Couldn't find a project`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Run <code>rome init</code> in this directory to initialize a project`,
			},
		],
	},
	INITING_SENSITIVE: (directory: AbsoluteFilePath) => ({
		category: "projectManager/sensitiveDirectory",
		message: markup`Cannot create a project config in sensitive directory <emphasis>${filePathToMarkup(
			directory,
			true,
		)}</emphasis>`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`A project created here is always a mistake. Try another folder.`,
			},
		],
	}),
	LOADING_SENSITIVE: (directory: AbsoluteFilePath) => ({
		category: "projectManager/sensitiveDirectory",
		message: markup`Cannot load a project config in sensitive directory <emphasis>${filePathToMarkup(
			directory,
			true,
		)}</emphasis>`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`A project created here is always a mistake. Check why this config exists and if necessary delete and try again.`,
			},
		],
	}),
	TYPO_CONFIG_FILENAME: (invalidFilename: string, validFilenames: Array<string>) => ({
		category: "projectManager/typoConfigFilename",
		message: markup`Invalid Rome config filename <emphasis>${invalidFilename}</emphasis>`,
		advice: buildSuggestionAdvice(invalidFilename, validFilenames),
	}),
	MISPLACED_CONFIG: (misplacedName: string) => ({
		category: "projectManager/misplacedConfig",
		message: markup`Misplaced project config <emphasis>${misplacedName}</emphasis>`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`This should be inside of a <emphasis>.config</emphasis> folder`,
			},
		],
	}),
});
