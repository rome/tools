import {createMigration} from "@internal/core/server/migrate/Migration";
import {markup} from "@internal/markup";

export default createMigration({
	versionRange: ">=10",
	name: "renameIgnorePaths",
	addedVersion: "10.0.0",
	deprecated: true,
	description: markup`Rename <emphasis>ignorePaths</emphasis>  to <emphasis>ignore</emphasis>`,
	runMigration: async (consumer) => {
		const lint = consumer.get("lint");
		const ignorePaths = lint.get("ignorePaths");
		lint.set("ignore", ignorePaths.copy().asPlainArray());
		lint.delete("ignorePaths");
	},
	shouldMigrate: (currentVersion, config) => {
		if (config.has("lint")) {
			const lint = config.get("lint");
			return lint.has("ignorePaths");
		}

		return false;
	},
});
