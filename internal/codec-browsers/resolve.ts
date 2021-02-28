import {Browser} from "@internal/browser-features/Browser";
import {
	getAllBrowserNames,
	getAllBrowserUsages,
	getBrowser,
} from "@internal/browser-features";
import {parseBrowserQuery} from "@internal/codec-browsers/parse";
import {dead, modern} from "@internal/codec-browsers/presets";

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

interface TargetBrowser {
	readonly type: "TargetBrowser";
	browser: string;
	version: number | "all";
}

interface TargetBrowserState {
	readonly type: "TargetBrowserState";
	browser?: string;
	state: "current" | "unreleased" | "maintained";
}

interface TargetBrowserRange {
	readonly type: "TargetBrowserRange";
	browser: string;
	version: number;
	to: number;
}

interface TargetBrowserRangeOperator {
	readonly type: "TargetBrowserRangeOperator";
	browser: string;
	version: number;
	operator: "GT" | "LT" | "GE" | "LE";
}

interface TargetBrowserPreset {
	readonly type: "TargetBrowserPreset";
	preset: "modern" | "dead";
}

interface TargetBrowserCombination {
	readonly type: "TargetBrowserCombination";
	target: AnyTargetBrowser;
	and: AnyTargetBrowser;
}

interface TargetBrowserCoverage {
	readonly type: "TargetBrowserCoverage";
	coverage: number;
	region?: string;
}

interface TargetBrowserUsage {
	readonly type: "TargetBrowserUsage";
	usage: number;
	operator: "GT" | "LT" | "GE" | "LE";
	region?: string;
}

interface TargetBrowserSince {
	readonly type: "TargetBrowserSince";
	since: number;
}

interface TargetBrowserLast {
	readonly type: "TargetBrowserLast";
	qty: number;
	unit: "years" | "months" | "days" | "versions" | "majorversions";
	browser?: string;
}

interface TargetBrowserInversion {
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

	targets.forEach((target) => {
		switch (target.type) {
			case "TargetBrowser": {
				if (target.version === "all") {
					(getBrowser({name: target.browser})?.getVersions()).forEach((version) => {
						browsers.add(
							getBrowser({
								name: target.browser,
								version,
							})!,
						);
					});

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

				browserNames.forEach((name) => {
					if (getBrowser({name})) {
						switch (target.state) {
							case "current": {
								if (getBrowser({name})) {
									browsers.add(getBrowser({name})!);
								}
								break;
							}
							case "unreleased": {
								getBrowser({name})!.getVersions().filter((version) =>
									!getBrowser({name, version})?.isReleased()
								).forEach((version) =>
									browsers.add(getBrowser({name, version})!)
								);
								break;
							}
							case "maintained": {
								// Not supported yet, will be once we add javascript feature support
								break;
							}
						}
					}
				});

				break;
			}
			case "TargetBrowserRange": {
				const versions = getBrowser({name: target.browser})
					? getBrowser({name: target.browser})!.getVersions()
					: [];

				versions.forEach((v) => {
					if (v >= target.version && v <= target.to) {
						browsers.add(getBrowser({name: target.browser, version: v})!);
					}
				});
				break;
			}
			case "TargetBrowserRangeOperator": {
				if (getBrowser({name: target.browser})) {
					const versions = getBrowser({name: target.browser})!.getVersions();

					versions.forEach((version) => {
						switch (target.operator) {
							case "GT": {
								if (version > target.version) {
									browsers.add(getBrowser({name: target.browser, version})!);
								}
								break;
							}
							case "LT": {
								if (version < target.version) {
									browsers.add(getBrowser({name: target.browser, version})!);
								}
								break;
							}
							case "GE": {
								if (version >= target.version) {
									browsers.add(getBrowser({name: target.browser, version})!);
								}
								break;
							}
							case "LE": {
								if (version <= target.version) {
									browsers.add(getBrowser({name: target.browser, version})!);
								}
								break;
							}
						}
					});
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
						getBrowser({name: usages[i].id, version: usages[i].version})!,
					);
					coverage += usages[i].usage;
					if (coverage >= target.coverage) {
						break;
					}
				}
				break;
			}
			case "TargetBrowserUsage": {
				getAllBrowserNames().forEach((name) => {
					getBrowser({name})!.getVersions().forEach((version) => {
						let usage = target.region
							? getBrowser({name, version})?.getRegionUsage(target.region)
							: getBrowser({name, version})?.getGlobalUsage();

						switch (target.operator) {
							case "GT": {
								if (usage && usage > target.usage) {
									browsers.add(getBrowser({name, version})!);
								}
								break;
							}
							case "LT": {
								if (usage && usage < target.usage) {
									browsers.add(getBrowser({name, version})!);
								}
								break;
							}
							case "GE": {
								if (usage && usage >= target.usage) {
									browsers.add(getBrowser({name, version})!);
								}
								break;
							}
							case "LE": {
								if (usage && usage <= target.usage) {
									browsers.add(getBrowser({name, version})!);
								}
								break;
							}
						}
					});
				});
				break;
			}

			case "TargetBrowserSince": {
				getAllBrowserNames().forEach((name) => {
					const releaseDate = getBrowser({name})?.getRawReleaseDate();
					if (releaseDate && releaseDate > target.since) {
						browsers.add(getBrowser({name})!);
					}
				});
				break;
			}
			case "TargetBrowserLast": {
				let names = target.browser ? [target.browser] : getAllBrowserNames();

				names.forEach((name) => {
					switch (target.unit) {
						case "years":
						case "months":
						case "days": {
							const releaseDate = getBrowser({name})?.getRawReleaseDate();
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
								browsers.add(getBrowser({name})!);
							}
							break;
						}
						case "versions": {
							// `b - a` reverses the list
							(getBrowser({name})?.getVersions()).sort((a, b) => b - a).slice(
								0,
								target.qty,
							).forEach((version) => browsers.add(getBrowser({name, version})!));
							break;
						}
						case "majorversions": {
							// `b - a` reverses the list
							// v % 1 checks if the number is whole
							(getBrowser({name})?.getVersions()).filter((v) => v % 1 === 0).sort((
								a,
								b,
							) => b - a).slice(0, target.qty).forEach((version) =>
								browsers.add(getBrowser({name, version})!)
							);
							break;
						}
					}
				});
				break;
			}
			case "TargetBrowserInversion": {
				resolveTargets(target.target).forEach((b) => toRemove.add(b));
				break;
			}
		}
	});

	return new Set(
		Array.from(browsers).filter((browser) => !toRemove.has(browser)),
	);
}
