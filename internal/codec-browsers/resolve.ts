import {Browser} from "@internal/browser-features/Browser";

export type AnyTargetBrowser = TargetBrowser
	| TargetBrowserRange
	| TargetBrowserPreset
	| TargetBrowserCombination
	| TargetBrowserCoverage;

interface TargetBrowser {
	readonly type: "TargetBrowserRange",
	browser: string,
	version: string
}

interface TargetBrowserRange {
	readonly type: "TargetBrowserRange",
	browser: string,
	from: string,
	to: string,
}

interface TargetBrowserPreset {
	readonly type: "TargetBrowserPreset"
}

interface TargetBrowserCombination {
	readonly type: "TargetBrowserCombination",
	of: AnyTargetBrowser,
	and: AnyTargetBrowser
}

interface TargetBrowserCoverage {
	readonly type: "TargetBrowserCoverage",
	coverage: number,
	region?: string
}

export function resolveTarget(targets: AnyTargetBrowser[]): Browser[] {


	return [];
}
