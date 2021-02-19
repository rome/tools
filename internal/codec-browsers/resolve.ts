import {Browser} from "@internal/browser-features/Browser";

export type AnyTargetBrowser = TargetBrowser
  | TargetBrowserState
	| TargetBrowserRange
	| TargetBrowserRangeOperator
	| TargetBrowserPreset
	| TargetBrowserCombination
	| TargetBrowserCoverage
	| TargetBrowserInversion;

interface TargetBrowser {
	readonly type: "TargetBrowser",
	browser: string,
	version: number
}

interface TargetBrowserState {
	readonly type: "TargetBrowserState",
	browser?: string,
	state: "current" | "unreleased" | "maintained"
}

interface TargetBrowserRange {
	readonly type: "TargetBrowserRange",
	browser: string,
	version: number,
	to: number,
}

interface TargetBrowserRangeOperator {
	readonly type: "TargetBrowserRangeOperator",
	browser: string,
	version: number,
	operator: "GT" | "LT" | "GE" | "LE",
}

interface TargetBrowserPreset {
	readonly type: "TargetBrowserPreset"
	preset: "modern" | "dead"
}

interface TargetBrowserCombination {
	readonly type: "TargetBrowserCombination",
	target: AnyTargetBrowser,
	and: AnyTargetBrowser
}

interface TargetBrowserCoverage {
	readonly type: "TargetBrowserCoverage",
	coverage: number,
	operator: "GT" | "LT" | "GE" | "LE",
	region?: string
}

interface TargetBrowserInversion {
	readonly type: "TargetBrowserInversion",
	target: Exclude<AnyTargetBrowser, TargetBrowserCombination | TargetBrowserInversion>
}

export function resolveTarget(targets: AnyTargetBrowser[]): Browser[] {


	return [];
}
