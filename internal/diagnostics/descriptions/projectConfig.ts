import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";

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

	TOO_MANY_WILDCARDS: (pattern: string) => ({
		message: markup`Pattern <emphasis>${pattern}</emphasis> contains more than one wildacard.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Alias patterns should have the following format <emphasis>\\<prefix>[*]\\<sufix></emphasis>.`,
			},
		],
	}),

	EMPTY_PATTERN: {
		message: markup`Alias pattern can't be empty.`,
	},
});
