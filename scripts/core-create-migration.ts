import {INTERNAL, reporter, writeFile} from "./_utils";
import {markup} from "@internal/markup";
import {dedent, toCamelCase} from "@internal/string-utils";
import {VERSION} from "@internal/core";
import {main as generateMigrations} from "./generated-files/migrations";

const migrationsPath = INTERNAL.append(
	"core",
	"server",
	"migrate",
	"migrations",
);

export async function main([migration]: string[]): Promise<number> {
	if (migration === undefined) {
		reporter.error(markup`./scripts core-create-migration [migrationName]`);
		return 1;
	}

	const migrationName = toCamelCase(migration);

	await writeFile(
		migrationsPath.append(migrationName).append("index.ts"),
		dedent`
			export default createMigration({
				versionRange: "",
				name: "${migrationName}",
				addedVersion: "${VERSION}",
				deprecated: false,
				runMigration: async (consumer) => {},
				shouldMigrate: (currentVersion, config) => {}
			});
		`,
	);

	await generateMigrations();

	return 0;
}
