import {createDiagnosticsCategory, DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";
import {markup} from "@internal/markup";

//
/**
 * {@link internal/core/server/commands/migrate}
 */
export const migrateCommand = createDiagnosticsCategory({
	MISSING_CONFIGURATION: {
		category: DIAGNOSTIC_CATEGORIES["commands/migrate"],
		message: markup`Rome could not find a configuration.`,
		advice: [
			{
				type: "action",
				description: markup`Bootstrap the project first.`,
				command: "init",
			}
		]
	}
})
