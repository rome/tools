import {Browser} from "@internal/browser-features/Browser";
import {
	getAllBrowserNames,
	getAllBrowserUsages,
	getBrowser,
} from "@internal/browser-features";
import {parseBrowserQuery} from "@internal/codec-browsers/parse";
import {dead, modern} from "@internal/codec-browsers/presets";

export type TargetOperator = "GT" | "LT" | "GE" | "LE";
export type TargetState = "current" | "unreleased" | "maintained";
export type TargetUnit =
	| "years"
	| "months"
	| "days"
	| "versions"
	| "majorversions";

export type AnyTargetBrowser =
	| TargetBrowser
	| TargetBrowserState
	| TargetBrowserRange
	| TargetBrowserRangeOperator
	| TargetBrowserPreset
	| TargetBrowserCombination
	| TargetBrowserCoverage
	| TargetBrowserUsage
	| TargetBrowserSince
	| TargetBrowserLast
	| TargetBrowserInversion;

export interface TargetBrowser {
	readonly type: "TargetBrowser";
	browser: string;
	version: number | "all";
}

export interface TargetBrowserState {
	readonly type: "TargetBrowserState";
	browser?: string;
	state: TargetState;
}

export interface TargetBrowserRange {
	readonly type: "TargetBrowserRange";
	browser: string;
	version: number;
	to: number;
}

export interface TargetBrowserRangeOperator {
	readonly type: "TargetBrowserRangeOperator";
	browser: string;
	version: number;
	operator: TargetOperator;
}

export interface TargetBrowserPreset {
	readonly type: "TargetBrowserPreset";
	preset: "modern" | "dead";
}

export interface TargetBrowserCombination {
	readonly type: "TargetBrowserCombination";
	left: AnyTargetBrowser;
	right: AnyTargetBrowser;
}

export interface TargetBrowserCoverage {
	readonly type: "TargetBrowserCoverage";
	coverage: number;
	region?: string;
}

export interface TargetBrowserUsage {
	readonly type: "TargetBrowserUsage";
	usage: number;
	operator: TargetOperator;
	region?: string;
}

export interface TargetBrowserSince {
	readonly type: "TargetBrowserSince";
	since: number;
}

export interface TargetBrowserLast {
	readonly type: "TargetBrowserLast";
	qty: number;
	unit: TargetUnit;
	browser?: string;
}

export interface TargetBrowserInversion {
	readonly type: "TargetBrowserInversion";
	target: Exclude<
		AnyTargetBrowser,
		TargetBrowserCombination | TargetBrowserInversion
	>;
}

export interface ResolveOptions {
	fixedDate?: Date;
}

export function resolveTargets(
	targets: AnyTargetBrowser | AnyTargetBrowser[],
	resolveOptions?: ResolveOptions,
): Set<Browser> {
	const browsers: Set<Browser> = new Set();
	const toRemove: Set<Browser> = new Set();

	if (!Array.isArray(targets)) {
		targets = [targets];
	}

	for (const target of targets) {
		switch (target.type) {
			case "TargetBrowser": {
				if (target.version === "all") {
					for (const version of getBrowser({name: target.browser}).getVersions()) {
						browsers.add(
							getBrowser({
								name: target.browser,
								version,
							}),
						);
					}

					break;
				}

				browsers.add(
					getBrowser({
						name: target.browser,
						version: target.version,
					}),
				);

				break;
			}

			case "TargetBrowserState": {
				let browserNames: string[] = [];

				if (target.browser) {
					browserNames.push(target.browser);
				} else {
					browserNames = browserNames.concat(getAllBrowserNames());
				}

				for (const name of browserNames) {
					switch (target.state) {
						case "current": {
							if (getBrowser({name})) {
								browsers.add(getBrowser({name}));
							}
							break;
						}

						case "unreleased": {
							const allVersions = getBrowser({name}).getVersions();
							const releasedVersions = allVersions.filter((version) =>
								!getBrowser({name, version}).isReleased()
							);
							for (const version of releasedVersions) {
								browsers.add(getBrowser({name, version}));
							}
							break;
						}

						case "maintained": {
							// Not supported yet, will be once we add javascript feature support
							break;
						}
					}
				}

				break;
			}

			case "TargetBrowserRange": {
				const versions = getBrowser({name: target.browser})
					? getBrowser({name: target.browser}).getVersions()
					: [];

				for (const version of versions) {
					if (version >= target.version && version <= target.to) {
						browsers.add(getBrowser({name: target.browser, version}));
					}
				}
				break;
			}

			case "TargetBrowserRangeOperator": {
				const versions = getBrowser({name: target.browser}).getVersions();

				for (const version of versions) {
					if (evaluateTargetOperator(version, target.operator, target.version)) {
						browsers.add(getBrowser({name: target.browser, version}));
					}
				}
				break;
			}

			case "TargetBrowserPreset": {
				switch (target.preset) {
					case "modern": {
						const targets = resolveTargets(
							parseBrowserQuery({
								input: modern,
							}),
							resolveOptions,
						);

						for (const browser of targets) {
							browsers.add(browser);
						}
						break;
					}

					case "dead": {
						const targets = resolveTargets(
							parseBrowserQuery({
								input: dead,
							}),
							resolveOptions,
						);

						for (const browser of targets) {
							browsers.add(browser);
						}
						break;
					}
				}
				break;
			}

			case "TargetBrowserCombination": {
				const left = resolveTargets(target.left, resolveOptions);
				const right = resolveTargets(target.right, resolveOptions);

				for (const browser of left) {
					if (right.has(browser)) {
						browsers.add(browser);
					}
				}
				break;
			}

			case "TargetBrowserCoverage": {
				const usages = getAllBrowserUsages().sort((a, b) => b.usage - a.usage);
				let coverage = 0;

				for (const {id, usage, version} of usages) {
					if (usage === 0) {
						break;
					}

					browsers.add(getBrowser({name: id, version}));

					coverage += usage;
					if (coverage >= target.coverage) {
						break;
					}
				}
				break;
			}

			case "TargetBrowserUsage": {
				for (const name of getAllBrowserNames()) {
					for (const version of getBrowser({name}).getVersions()) {
						const browser = getBrowser({name, version});
						const usage = target.region
							? browser.getRegionUsage(target.region)
							: browser.getGlobalUsage();

						if (
							usage !== undefined &&
							evaluateTargetOperator(usage, target.operator, target.usage)
						) {
							browsers.add(getBrowser({name, version}));
						}
					}
				}
				break;
			}

			case "TargetBrowserSince": {
				for (const name of getAllBrowserNames()) {
					const releaseDate = getBrowser({name}).getRawReleaseDate();
					if (releaseDate !== undefined && releaseDate > target.since) {
						browsers.add(getBrowser({name}));
					}
				}
				break;
			}

			case "TargetBrowserLast": {
				let names = target.browser ? [target.browser] : getAllBrowserNames();

				for (const name of names) {
					switch (target.unit) {
						case "years":
						case "months":
						case "days": {
							const releaseDate = getBrowser({name}).getRawReleaseDate();
							const date = resolveOptions?.fixedDate ?? new Date();

							switch (target.unit) {
								case "years": {
									date.setFullYear(date.getFullYear() - target.qty);
									break;
								}
								case "months": {
									date.setMonth(date.getMonth() - target.qty);
									break;
								}
								case "days": {
									date.setDate(date.getDate() - target.qty);
									break;
								}
							}

							if (releaseDate !== undefined && releaseDate > date.getTime()) {
								browsers.add(getBrowser({name}));
							}
							break;
						}

						case "versions": {
							const allVersions = getBrowser({name}).getVersions();
							const desiredVersions = getLatestVersions(allVersions, target.qty);
							for (const version of desiredVersions) {
								browsers.add(getBrowser({name, version}));
							}
							break;
						}

						case "majorversions": {
							const allVersions = getBrowser({name}).getVersions();
							const majorVersions = allVersions.filter((v) =>
								Number.isInteger(v)
							);
							const desiredVersions = getLatestVersions(
								majorVersions,
								target.qty,
							);
							for (const version of desiredVersions) {
								browsers.add(getBrowser({name, version}));
							}
							break;
						}
					}
				}
				break;
			}

			case "TargetBrowserInversion": {
				for (const b of resolveTargets(target.target, resolveOptions)) {
					toRemove.add(b);
				}
				break;
			}
		}
	}

	for (const browser of toRemove) {
		browsers.delete(browser);
	}

	return browsers;
}

function evaluateTargetOperator(
	version: number,
	op: TargetOperator,
	targetVersion: number,
): boolean {
	switch (op) {
		case "GT":
			return version > targetVersion;

		case "LT":
			return version < targetVersion;

		case "GE":
			return version >= targetVersion;

		case "LE":
			return version <= targetVersion;
	}
}

function getLatestVersions(versions: number[], count: number): number[] {
	return versions.sort((a, b) => b - a).slice(0, count);
}
