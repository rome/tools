import {migrations} from "./migrations/index";
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

/**
 * This class is responsible to gather the migrations inside Rome and run them in
 * order of their addedVersion
 */
export class Migrator {
	private readonly version: SemverVersion;
	private migrations: Map<SemverVersion, Migration[]>;
	private readonly reporter: Reporter;
	private readonly orderedVersions: SemverVersion[];

	constructor({reporter, version}: MigratorConstructor) {
		this.version = parseSemverVersion({input: version});
		this.migrations = Migrator.mapMigrations(migrations);
		this.reporter = reporter;
		this.orderedVersions = this.orderMigrations();
	}

	private static mapMigrations(migrations: Set<Migration>) {
		const mappedMigrations: Map<SemverVersion, Migration[]> = new Map();
		for (const migration of migrations) {
			if (mappedMigrations.has(migration.addedVersion)) {
				const currentMigrations = mappedMigrations.get(migration.addedVersion);
				currentMigrations?.push(migration);
			} else {
				mappedMigrations.set(migration.addedVersion, [migration]);
			}
		}

		return mappedMigrations;
	}

	private orderMigrations() {
		const versions = Array.from(this.migrations.keys());
		return sortSemverVersions(versions);
	}

	async run(config: Consumer) {
		let migrationsExecuted = 0;
		this.reporter.log(markup`Start migrating the configuration file.`);

		for (const version of this.orderedVersions) {
			const migrations = this.migrations.get(version);
			if (migrations) {
				for (const migration of migrations) {
					const {runMigration, checkShouldMigrate, name, description} = migration;

					// check if the current configuration has the property
					if (checkShouldMigrate(this.version, config)) {
						if (migration.isDeprecated === true) {
							this.reporter.warn(
								markup`The migration <emphasis>${name}</emphasis> has been marked as <emphasis>deprecated</emphasis> and it will be deleted in the next major release.`,
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
						migrationsExecuted += 1;
					}
				}
			}
		}

		if (migrationsExecuted === 0) {
			this.reporter.success(
				markup`No migrations executed. Your configuration file is up to date!`,
			);
		} else {
			this.reporter.success(
				markup`Rome run a total number of <emphasis>${migrationsExecuted}</emphasis> executed. Now your configuration file si updated!`,
			);
		}
	}
}
