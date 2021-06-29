import {
	DIAGNOSTIC_CATEGORIES,
	createDiagnosticsCategory,
} from "@internal/diagnostics";
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
			},
		],
	},
	EXPECT_REPO: {
		category: DIAGNOSTIC_CATEGORIES["commands/migrate"],
		message: markup`The migrate command should be run inside a repository.`,
	},
	UNCOMMITTED_CHANGES: {
		category: DIAGNOSTIC_CATEGORIES["commands/migrate"],
		message: markup`The migrate command could probably change you configuration file, it is recommended to stash your local changes.`,
	},
});
