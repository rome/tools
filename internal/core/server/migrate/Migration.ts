import {Consumer} from "@internal/consume";
import {
	SemverRange,
	SemverVersion,
	parseSemverRange,
	parseSemverVersion,
	satisfiesSemver,
} from "@internal/codec-semver";
import {Markup} from "@internal/markup";

export type RunMigration = (consumer: Consumer) => Promise<void>;
export type ShouldMigrate = (
	currentVersion: SemverVersion,
	config: Consumer,
) => boolean;

export interface MigrationOptions {
	/**
	 * The actual function that changes the configuration
	 */
	runMigration: RunMigration;
	/**
	 * An optional function to make further checks when to run a migrations.
	 *
	 * For instance, check if the user configuration has actually a specific field
	 */
	shouldMigrate?: ShouldMigrate;
	/**
	 * This makes sure that the migration should run on a specific range.
	 *
	 * If the given version is <11 and Rome's version is 15, this migration won't run
	 */
	versionRange: string;
	/**
	 * The version when this migration is created. This information is important because migrations
	 * will run from the oldest version to the newest
	 */
	addedVersion: string;
	/**
	 * An arbitrary name for the migration
	 */
	name: string;
	/**
	 * An optional description to show to the user
	 */
	description?: Markup;

	/**
	 * If true, the migration will be removed in the next major release
	 */
	deprecated?: boolean;
}

export class Migration {
	constructor(opts: MigrationOptions) {
		this.runMigration = opts.runMigration;
		this.runOnVersion = parseSemverRange({input: opts.versionRange});
		this.addedVersion = parseSemverVersion({input: opts.addedVersion});
		this.name = opts.name;
		this.description = opts?.description;
		this.shouldMigrate = opts?.shouldMigrate;
		this._shouldMigrate = undefined;
		this.checkShouldMigrate = this.checkShouldMigrate.bind(this);
		this.deprecated = opts.deprecated;
	}

	public runMigration: RunMigration;
	private readonly runOnVersion: SemverRange;
	public readonly addedVersion: SemverVersion;
	public shouldMigrate: ShouldMigrate | undefined;
	public name: string;
	public description?: Markup | undefined;
	private readonly deprecated?: boolean;
	// cache value
	private _shouldMigrate: boolean | undefined;

	public checkShouldMigrate(currentVersion: SemverVersion, config: Consumer) {
		if (this._shouldMigrate === undefined) {
			this._shouldMigrate =
				satisfiesSemver(currentVersion, this.runOnVersion) &&
				this.shouldMigrate?.(currentVersion, config);
		}
		return this._shouldMigrate;
	}

	get isDeprecated() {
		return this.deprecated;
	}
}

export function createMigration(opts: MigrationOptions) {
	return new Migration(opts);
}
