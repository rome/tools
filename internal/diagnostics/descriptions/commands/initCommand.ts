import {DiagnosticAdvice} from "@internal/diagnostics";
import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";

const IGNORE_ADVICE: DiagnosticAdvice = [
	{
		type: "log",
		category: "info",
		text: markup`If you still really want to do this, you can bypass the restriction and auto configurate the project with <code>--allow-dirty</code>:`,
	},
	{
		type: "code",
		language: "shell",
		sourceText: "rome auto-config --allow-dirty",
	},
];

// @internal/core/server/commands/init.ts
export const initCommand = createDiagnosticsCategory({
	UNCOMMITTED_CHANGES: {
		category: "commands/auto-config/uncommittedChanges",
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
			...IGNORE_ADVICE,
		],
	},
	EXPECTED_REPO: {
		category: "commands/auto-config/expectedRepo",
		message: markup`Directory is not a repository. Are you sure this is where you wanted to create a project?`,
		advice: [
			{
				type: "log",
				category: "warn",
				text: markup`This command is destructive and will format and autofix all files within.`,
			},

			...IGNORE_ADVICE,
		],
	},
});
