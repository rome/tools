import {DiagnosticAdvice} from "@internal/diagnostics";
import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics/categories";

const IGNORE_ADVICE: DiagnosticAdvice[] = [
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
		category: DIAGNOSTIC_CATEGORIES["commands/auto-config/uncommittedChanges"],
		advice: [
			...IGNORE_ADVICE,
		],
	},
	EXPECTED_REPO: {
		category: DIAGNOSTIC_CATEGORIES["commands/auto-config/expectedRepo"],
		advice: [
			...IGNORE_ADVICE,
		],
	},
});
