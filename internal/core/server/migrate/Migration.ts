import {Consumer} from "@internal/consume";
import {parseSemverRange, satisfiesSemver, SemverRange, SemverVersion} from "@internal/codec-semver";
import {Markup} from "@internal/markup";


export type RunMigration =  (consumer: Consumer) => Promise<void>;
export type ProjectKey =  string
export type ShouldMigrate = (currentVersion: SemverVersion, config: Consumer) => boolean;


export interface MigrationOptions {
	runMigration: RunMigration;
	shouldMigrate: ShouldMigrate
	version: string;
	name: string;
	description: Markup;
}

export class Migration {

	constructor(opts: MigrationOptions) {
		this.runMigration = opts.runMigration;
		this.onVersion = parseSemverRange({ input: opts.version });
		this.shouldMigrate = opts.shouldMigrate;
		this._shouldMigrate = undefined;
		this.checkShouldMigrate = this.checkShouldMigrate.bind(this);
	}

	public runMigration: RunMigration
	private readonly onVersion: SemverRange;
	public shouldMigrate: ShouldMigrate
	// cache value
	private _shouldMigrate: boolean | undefined;

	public checkShouldMigrate(currentVersion: SemverVersion, config: Consumer) {
		if (this._shouldMigrate === undefined) {
			this._shouldMigrate = this.shouldMigrate(currentVersion, config) && satisfiesSemver(currentVersion, this.onVersion);
		}
		return this._shouldMigrate;
	}
}


export function createMigration(opts: MigrationOptions) {
	return new Migration(opts)
}
