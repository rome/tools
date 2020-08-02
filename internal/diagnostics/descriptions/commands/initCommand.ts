import {DiagnosticAdvice} from "@internal/diagnostics";
import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";

const IGNORE_ADVICE: DiagnosticAdvice = [
	{
		type: "log",
		category: "info",
		text: markup`If you still really want to do this, you can bypass the restriction and create a project in this directory with the flag <code>--allow-dirty</code>:`,
	},
	{
		type: "code",
		language: "shell",
		sourceText: "rome init --allow-dirty",
	},
];

// @internal/core/server/commands/init.ts
export const initCommand = createDiagnosticsCategory({
	UNCOMMITTED_CHANGES: {
		category: "commands/init/uncommittedChanges",
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
				sourceText: `git add -A && git commit -m "Rome init backup"`,
			},
			...IGNORE_ADVICE,
		],
	},
	EXPECTED_REPO: {
		category: "commands/init/expectedRepo",
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
