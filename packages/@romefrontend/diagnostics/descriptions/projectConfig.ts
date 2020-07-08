import {createDiagnosticsCategory} from "./index";

// @romefrontend/project
export const projectConfig = createDiagnosticsCategory({
	BOOLEAN_CATEGORY: (enabled: boolean) => ({
		message: `Expected an object here but got a boolean`,
		advice: [
			{
				type: "log",
				category: "info",
				text: `You likely wanted \`{"enabled": ${String(enabled)}}\` instead`,
			},
		],
	}),
	RECURSIVE_CONFIG: "Recursive config",
});
