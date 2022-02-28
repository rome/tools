import {INTERNAL, createDirectory, reporter, writeFile} from "./_utils";
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
		reporter.error(markup`./script core-create-migration [migrationName]`);
		return 1;
	}

	const migrationName = toCamelCase(migration);

	const migrationPath = migrationsPath.append(migrationName);

	await createDirectory(migrationPath);

	await writeFile(
		migrationPath.append("index.ts"),
		dedent`
			import {createMigration} from "@internal/core/server/migrate/Migration";

			export default createMigration({
				versionRange: "",
				name: "${migrationName}",
				addedVersion: "${VERSION}",
				deprecated: false,
				runMigration: async (consumer) => {},
				shouldMigrate: (config, currentVersion) => {}
			});
		`,
	);

	await generateMigrations();

	return 0;
}
