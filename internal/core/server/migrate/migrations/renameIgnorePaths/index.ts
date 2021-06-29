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
		lint.set("ignore", ignorePaths.copy().getValue());
		lint.delete("ignorePaths");
	},
	shouldMigrate: (currentVersion, config) => {
		let should = false;
		if (config.has("lint")) {
			const lint = config.get("lint");
			should = lint.has("ignorePaths");
		}

		return should;
	},
});
