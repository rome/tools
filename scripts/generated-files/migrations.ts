import {INTERNAL, modifyGeneratedFile} from "../_utils";
import {toCamelCase} from "@internal/string-utils";

const migrationsPath = INTERNAL.append(
	"core",
	"server",
	"migrate",
	"migrations",
);

interface MigrationDefinition {
	name: string;
}

async function getMigrations(): Promise<MigrationDefinition[]> {
	let defs: MigrationDefinition[] = [];

	for (const path of await migrationsPath.readDirectory()) {
		const migrationName = path.getBasename();
		if ((await path.lstat()).isFile()) {
			continue;
		}
		for (const mPath of await path.readDirectory()) {
			if (
				mPath.getBasename()[0] !== "." &&
				mPath.hasEndExtension("ts") &&
				!mPath.hasEndExtension("test.ts")
			) {
				defs.push({
					name: migrationName,
				});
			}
		}
	}

	defs = defs.sort((a, b) => {
		return a.name.localeCompare(b.name);
	});

	return defs;
}

export async function main() {
	const defs = await getMigrations();

	await modifyGeneratedFile(
		{
			path: migrationsPath.append("index.ts"),
			scriptName: "generated-files/migrations",
		},
		async () => {
			let lines = [];
			for (const {name} of defs) {
				lines.push(`import ${toCamelCase(name)} from "./${name}";`);
			}
			lines.push(`import {SemverVersion} from "@internal/codec-semver";`);
			lines.push(`import {Migration} from "../Migration";`);
			lines.push("");
			lines.push(
				"export const migrations: Map<SemverVersion, Migration> = new Map();",
			);
			for (const {name} of defs) {
				lines.push(`migrations.set(${name}.addedVersion, ${name});`);
			}

			return {lines};
		},
	);
}
