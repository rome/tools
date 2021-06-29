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
				runMigration: async (consumer) => {
					const lint = consumer.get("lint");
					const ignorePaths = lint.get("ignorePaths");
					lint.set("ignore", ignorePaths.copy().getValue());
					lint.delete("ignorePaths");
				},
			});
		`,
	);

	await generateMigrations();

	return 0;
}
