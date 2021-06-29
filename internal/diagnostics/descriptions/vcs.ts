import {
	DIAGNOSTIC_CATEGORIES,
	createDiagnosticsCategory,
} from "@internal/diagnostics";
import {markup} from "@internal/markup";

export const vcs = createDiagnosticsCategory({
	UNCOMMITTED_CHANGES: {
		category: DIAGNOSTIC_CATEGORIES["vcs/uncommittedChanges"],
		message: markup`Uncommitted changes in repository`,
		advice: [
			{
				type: "log",
				category: "warn",
				text: markup`We recommend committing your changes so you can recover them if you don't like the changes.`,
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
		message: markup`Directory is not a repository.`,
		advice: [
			{
				type: "log",
				category: "warn",
				text: markup`This command is destructive and will format and autofix all files within.`,
			},
			{
				type: "code",
				language: "shell",
				sourceText: `git init"`,
			},
		],
	},
});
