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
	target: AnyTargetBrowser;
	and: AnyTargetBrowser;
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

export function resolveTargets(
	targets: AnyTargetBrowser | AnyTargetBrowser[],
): Set<Browser> {
	const browsers: Set<Browser> = new Set([]);
	const toRemove: Set<Browser> = new Set([]);

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

				const newBrowser = getBrowser({
					name: target.browser,
					version: target.version,
				});

				if (newBrowser) {
					browsers.add(newBrowser);
				}
				break;
			}
			case "TargetBrowserState": {
				const browserNames: string[] = [];

				if (target.browser) {
					browserNames.push(target.browser);
				} else {
					browserNames.push(...getAllBrowserNames());
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
							getBrowser({name}).getVersions().filter((version) =>
								!getBrowser({name, version}).isReleased()
							).forEach((version) => browsers.add(getBrowser({name, version})));
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
					switch (target.operator) {
						case "GT": {
							if (version > target.version) {
								browsers.add(getBrowser({name: target.browser, version}));
							}
							break;
						}
						case "LT": {
							if (version < target.version) {
								browsers.add(getBrowser({name: target.browser, version}));
							}
							break;
						}
						case "GE": {
							if (version >= target.version) {
								browsers.add(getBrowser({name: target.browser, version}));
							}
							break;
						}
						case "LE": {
							if (version <= target.version) {
								browsers.add(getBrowser({name: target.browser, version}));
							}
							break;
						}
					}
				}
				break;
			}
			case "TargetBrowserPreset": {
				switch (target.preset) {
					case "modern": {
						resolveTargets(
							parseBrowserQuery({
								input: modern,
							}),
						).forEach((b) => browsers.add(b));
						break;
					}
					case "dead": {
						resolveTargets(
							parseBrowserQuery({
								input: dead,
							}),
						).forEach((b) => browsers.add(b));
						break;
					}
				}
				break;
			}
			case "TargetBrowserCombination": {
				const andTarget = resolveTargets(target.and);
				Array.from(resolveTargets(target.target)).filter((browser) =>
					andTarget.has(browser)
				).forEach((browser) => browsers.add(browser));
				break;
			}
			case "TargetBrowserCoverage": {
				const usages = getAllBrowserUsages().sort((a, b) => b.usage - a.usage);
				let coverage = 0;

				for (let i = 0; i < usages.length; i++) {
					if (usages[i].usage === 0) {
						break;
					}
					browsers.add(
						getBrowser({name: usages[i].id, version: usages[i].version}),
					);
					coverage += usages[i].usage;
					if (coverage >= target.coverage) {
						break;
					}
				}
				break;
			}
			case "TargetBrowserUsage": {
				for (const name of getAllBrowserNames()) {
					for (const version of getBrowser({name}).getVersions()) {
						let usage = target.region
							? getBrowser({name, version}).getRegionUsage(target.region)
							: getBrowser({name, version}).getGlobalUsage();

						switch (target.operator) {
							case "GT": {
								if (usage && usage > target.usage) {
									browsers.add(getBrowser({name, version}));
								}
								break;
							}
							case "LT": {
								if (usage && usage < target.usage) {
									browsers.add(getBrowser({name, version}));
								}
								break;
							}
							case "GE": {
								if (usage && usage >= target.usage) {
									browsers.add(getBrowser({name, version}));
								}
								break;
							}
							case "LE": {
								if (usage && usage <= target.usage) {
									browsers.add(getBrowser({name, version}));
								}
								break;
							}
						}
					}
				}
				break;
			}

			case "TargetBrowserSince": {
				getAllBrowserNames().forEach((name) => {
					const releaseDate = getBrowser({name}).getRawReleaseDate();
					if (releaseDate && releaseDate > target.since) {
						browsers.add(getBrowser({name}));
					}
				});
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
							const date = new Date();

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
							if (releaseDate && releaseDate > date.getTime()) {
								browsers.add(getBrowser({name}));
							}
							break;
						}
						case "versions": {
							// `b - a` reverses the list
							if (getBrowser({name})) {
								getBrowser({name}).getVersions().sort((a, b) => b - a).slice(
									0,
									target.qty,
								).forEach((version) => browsers.add(getBrowser({name, version})));
							}
							break;
						}
						case "majorversions": {
							// `b - a` reverses the list
							// v % 1 checks if the number is whole
							if (getBrowser({name})) {
								getBrowser({name}).getVersions().filter((v) => v % 1 === 0).sort((
									a,
									b,
								) => b - a).slice(0, target.qty).forEach((version) =>
									browsers.add(getBrowser({name, version}))
								);
							}
							break;
						}
					}
				}
				break;
			}
			case "TargetBrowserInversion": {
				resolveTargets(target.target).forEach((b) => toRemove.add(b));
				break;
			}
		}
	}

	return new Set(
		Array.from(browsers).filter((browser) => !toRemove.has(browser)),
	);
}
