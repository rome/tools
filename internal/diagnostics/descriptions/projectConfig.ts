import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";
import {LintCategories} from "@internal/compiler/lint/rules/categories";
import {buildSuggestionAdvice} from "@internal/diagnostics";

// @internal/project
export const projectConfig = createDiagnosticsCategory({
	BOOLEAN_CATEGORY: (enabled: boolean) => ({
		message: markup`Expected an object here but got a boolean`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`You likely wanted \`{"enabled": ${String(enabled)}}\` instead`,
			},
		],
	}),
	RECURSIVE_CONFIG: {message: markup`Recursive config`},

	RULES_UNKNOWN_CATEGORY: (
		unknownCategory: string,
		availableCategories: LintCategories[],
	) => ({
		message: markup`The category <emphasis>${unknownCategory}</emphasis> is unknown`,
		advice: buildSuggestionAdvice(unknownCategory, availableCategories),
	}),

	RULES_UNKNOWN_RULE_NAME: (unknownRule: string, suggestions: string[]) => ({
		message: markup`The rule <emphasis>${unknownRule}</emphasis> is unknown`,
		advice: buildSuggestionAdvice(unknownRule, suggestions),
	}),
});
