import {createDiagnosticsCategory, DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";
import {markup} from "@internal/markup";

export const vcs = createDiagnosticsCategory({
	UNCOMMITTED_CHANGES: {
		category: DIAGNOSTIC_CATEGORIES["vcs/uncommittedChanges"],
		message: markup`Uncommitted changes in repository`,
		advice: [
			{
				type: "log",
				category: "warn",
				text: markup`This command is destructive and will format and autofix all files within. We recommend committing your changes so you can recover them if you don't like the changes.`,
			},
			{
				type: "code",
				language: "shell",
				sourceText: `git add -A && git commit -m "chore: rome init backup"`,
			},
		],
	},
	EXPECTED_REPO: {
		category: DIAGNOSTIC_CATEGORIES["vcs/expectedRepo"],
		message: markup`Directory is not a repository. Are you sure this is where you wanted to create a project?`,
		advice: [
			{
				type: "log",
				category: "warn",
				text: markup`This command is destructive and will format and autofix all files within.`,
			},
		],
	},
});
