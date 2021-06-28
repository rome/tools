import migrations from "./index";
import {Migration} from "@internal/core/server/migrate/Migration";
import {parseSemverVersion, SemverVersion} from "@internal/codec-semver";
import {Reporter} from "@internal/cli-reporter";
import {markup} from "@internal/markup";
import {Consumer} from "@internal/consume";

interface MigratorConstructor {
	reporter :Reporter
	version: string;
}

export class Migrator {
	private version: SemverVersion;
	private migrations: Set<Migration>;
	private readonly reporter: Reporter;

	// TODO order migrations from oldest to newest
	// TODO deprecate migration
	// TODO suppressions for deprecated migrations inside tests

	constructor({ reporter, version }: MigratorConstructor) {
		this.version = parseSemverVersion({input: version});
		this.migrations = migrations;
		this.reporter = reporter
	}

	async run(config: Consumer) {
		this.reporter.log(markup`Start migrating the configuration file.`);

		for (const {runMigration, checkShouldMigrate} of this.migrations) {
			// check if the current configuration has the property
			if (checkShouldMigrate(this.version, config)) {
				await config.handleAsyncThrownDiagnostics(async () => {
					await runMigration(config)
				})
			}
		}

	}
}
