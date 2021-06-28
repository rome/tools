import {createMigration} from "@internal/core/server/migrate/Migration";
import {markup} from "@internal/markup";

export default createMigration({
	version: ">=10",
	name: "renameIgnore",
	description: markup`Rename <emphasis>ignorePaths</emphasis>  to <emphasis>ignore</emphasis>`,
	runMigration: async (consumer) => {
		if (consumer.has("lint")) {
			const lint = consumer.get("lint");
			if (lint.has("ignorePaths")) {
				const ignorePaths = lint.get("ignorePaths");
				lint.set("ignore", ignorePaths.copy().getValue());
				lint.delete("ignorePaths");
			}
		}
	},
	shouldMigrate: () => true
})
