import {createDiagnosticsCategory} from "./index";
import {DiagnosticLocation} from "../types";
import {markup} from "@romefrontend/markup";

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
						type: "command",
						command: "rome config set-directory vcs.root DIRECTORY_HERE",
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
	INCORRECT_CONFIG_FILENAME: (validFilenames: Array<string>) => ({
		category: "projectManager/incorrectConfigFilename",
		message: markup`Invalid rome config filename, <emphasis>${validFilenames.join(
			" or ",
		)}</emphasis> are the only valid filename`,
	}),
});
