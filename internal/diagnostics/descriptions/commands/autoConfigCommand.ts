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

/**
 * {@link internal/core/server/commands/autoConfig}
 */
export const autoConfigCommand = createDiagnosticsCategory({
	UNCOMMITTED_CHANGES: {
		category: DIAGNOSTIC_CATEGORIES["commands/auto-config/uncommittedChanges"],
		advice: [...IGNORE_ADVICE],
	},
	EXPECTED_REPO: {
		category: DIAGNOSTIC_CATEGORIES["commands/auto-config/expectedRepo"],
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Are you sure this is where you wanted to create a project?`,
			},
			...IGNORE_ADVICE,
		],
	},
});
