import {migrations} from "./index";
import {Migration} from "@internal/core/server/migrate/Migration";
import {
	SemverVersion,
	parseSemverVersion,
	sortSemverVersions,
} from "@internal/codec-semver";
import {Reporter} from "@internal/cli-reporter";
import {markup} from "@internal/markup";
import {Consumer} from "@internal/consume";

interface MigratorConstructor {
	reporter: Reporter;
	version: string;
}

export class Migrator {
	private readonly version: SemverVersion;
	private migrations: Map<SemverVersion, Migration>;
	private readonly reporter: Reporter;
	private readonly orderedVersions: SemverVersion[];

	constructor({reporter, version}: MigratorConstructor) {
		this.version = parseSemverVersion({input: version});
		this.migrations = migrations;
		this.reporter = reporter;
		this.orderedVersions = this.orderMigrations();
	}

	private orderMigrations() {
		const versions = Array.from(this.migrations.keys());
		return sortSemverVersions(versions);
	}

	async run(config: Consumer) {
		this.reporter.log(markup`Start migrating the configuration file.`);

		for (const version of this.orderedVersions) {
			const migration = this.migrations.get(version);
			if (migration) {
				const {runMigration, checkShouldMigrate, name, description} = migration;

				// check if the current configuration has the property
				if (checkShouldMigrate(this.version, config)) {
					if (migration.isDeprecated === true) {
						this.reporter.warn(
							markup`The migration <emaphasis>${name}</emaphasis> has been marked as deprecated and it will be deleted in the next major release.`,
						);
					}
					this.reporter.log(
						markup`Currently running the migration <emphasis>${name}</emphasis>`,
					);
					if (description) {
						this.reporter.info(description);
					}
					await config.handleAsyncThrownDiagnostics(async () => {
						await runMigration(config);
					});
					this.reporter.br();
				}
			}
		}
	}
}
