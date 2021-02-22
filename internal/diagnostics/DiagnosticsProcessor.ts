/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticFilterWithTest,
	DiagnosticOrigin,
	DiagnosticSuppression,
	DiagnosticSuppressions,
	Diagnostics,
} from "./types";
import {addOriginsToDiagnostics} from "./derive";
import {DiagnosticsError} from "./error-wrappers";
import {
	DIAGNOSTIC_CATEGORIES_SUPPRESS_DEPENDENCIES,
	DiagnosticCategoryPrefix,
} from "./categories";
import {descriptions} from "./descriptions";
import {matchesSuppression} from "@internal/compiler";
import {SourceMapConsumerCollection} from "@internal/codec-source-map";
import DiagnosticsNormalizer, {DiagnosticsNormalizerOptions} from "./DiagnosticsNormalizer";
import {MarkupFormatNormalizeOptions, readMarkup} from "@internal/markup";
import {AnyPath, MixedPathMap, MixedPathSet, equalPaths} from "@internal/path";
import {Event} from "@internal/events";
import {formatCategoryDescription} from "./helpers";

export type DiagnosticsProcessorOptions = {
	filters?: DiagnosticFilterWithTest[];
	origins?: DiagnosticOrigin[];
	markupOptions?: MarkupFormatNormalizeOptions;
	normalizeOptions?: DiagnosticsNormalizerOptions;
	sourceMaps?: SourceMapConsumerCollection;
};

type DiagnosticsMapEntry = {
	cachedCalculated:
		| undefined
		| {
				includedSuppressions: boolean;
				value: CalculatedDiagnostics;
			};

	dependencies: MixedPathSet;
	dependents: MixedPathSet;
	suppressDependents: boolean;
	possibleCount: number;

	dedupeKeys: SeenKeys;

	diagnostics: Set<Diagnostic>;
	suppressions: Set<DiagnosticSuppression>;
};

type DiagnosticsMap = MixedPathMap<DiagnosticsMapEntry>;

type CalculatedDiagnostics = {
	suppressions: DiagnosticSuppressions;
	guaranteed: Diagnostics;
	complete: Diagnostics;
};

type DiagnosticVisibility = "hidden" | "guaranteed" | "maybe";

type SeenKeys = Set<string>;

function isDeduped(diag: Diagnostic, seenKeys: SeenKeys): boolean {
	const parts: string[] = [
		`label:${diag.label === undefined ? "" : readMarkup(diag.label)}`,
		`category:${formatCategoryDescription(diag.description)}`,
		`message:${readMarkup(diag.description.message)}`,
	];

	// We don't do anything with `end` in this method, it's fairly meaningless for deduping errors
	let {start} = diag.location;
	if (start !== undefined) {
		parts.push(`start.line:${start.line}`);
		parts.push(`start.column:${start.column}`);
	}

	const key = parts.join(",");
	if (seenKeys.has(key)) {
		return true;
	} else {
		seenKeys.add(key);
		return false;
	}
}

function doesMatchSuppression(
	diag: Diagnostic,
	suppressions: Iterable<DiagnosticSuppression>,
	unusedSuppressions?: Set<DiagnosticSuppression>,
): boolean {
	for (const suppression of suppressions) {
		if (
			matchesSuppression(
				diag.description.category,
				diag.description.categoryValue,
				diag.location,
				suppression,
			)
		) {
			if (unusedSuppressions !== undefined) {
				unusedSuppressions.delete(suppression);
			}
			return true;
		}
	}

	return false;
}

export default class DiagnosticsProcessor {
	constructor(options: DiagnosticsProcessorOptions = {}) {
		this.filters = [];
		this.throwAfter = undefined;
		this.origins = options.origins === undefined ? [] : [...options.origins];
		this.allowedUnusedSuppressionPrefixes = new Set();
		this.sourceMaps = options.sourceMaps ?? new SourceMapConsumerCollection();
		this.normalizer = new DiagnosticsNormalizer(
			options.normalizeOptions,
			options.markupOptions,
			this.sourceMaps,
		);

		this.map = new MixedPathMap();
		this.cachedFlatDiagnostics = undefined;
		this.possibleCount = 0;
		
		this.insertDiagnosticsEvent = new Event("DiagnosticsProcessor.insertDiagnosticsEvent");
		this.guaranteedDiagnosticsEvent = new Event("DiagnosticsProcessor.visibleDiagnosticsEvent");
		this.modifiedDiagnosticsForPathEvent = new Event("DiagnosticsProcessor.modifiedDiagnosticsForPathEvent");
	}

	public normalizer: DiagnosticsNormalizer;
	public insertDiagnosticsEvent: Event<void>;
	public guaranteedDiagnosticsEvent: Event<Diagnostics>;
	public modifiedDiagnosticsForPathEvent: Event<AnyPath>;

	private sourceMaps: SourceMapConsumerCollection;
	private filters: DiagnosticFilterWithTest[];
	private allowedUnusedSuppressionPrefixes: Set<string>;
	private throwAfter: undefined | number;
	private origins: DiagnosticOrigin[];

	private map: DiagnosticsMap;
	private cachedFlatDiagnostics: undefined | Diagnostics;
	private possibleCount: number;

	private getMapEntry(path: AnyPath): DiagnosticsMapEntry {
		let entry: undefined | DiagnosticsMapEntry = this.map.get(path);
		if (entry === undefined) {
			entry = {
				cachedCalculated: undefined,
				dependencies: new MixedPathSet(),
				dependents: new MixedPathSet(),
				suppressDependents: false,
				possibleCount: 0,
				dedupeKeys: new Set(),
				suppressions: new Set(),
				diagnostics: new Set(),
			};
			this.map.set(path, entry);
		}
		return entry;
	}

	private assertEmpty() {
		if (this.hasDiagnostics()) {
			throw new Error("Expected no diagnostics for this operation");
		}
	}

	public static createImmediateThrower(
		origins: DiagnosticOrigin[],
	): DiagnosticsProcessor {
		const processor = new DiagnosticsProcessor({
			origins,
		});

		processor.insertDiagnosticsEvent.subscribe(() => {
			processor.maybeThrowDiagnosticsError();
		});

		return processor;
	}

	public unshiftOrigin(origin: DiagnosticOrigin) {
		this.origins.unshift(origin);
	}

	public setThrowAfter(num: undefined | number) {
		this.throwAfter = num;
	}

	public maybeThrowDiagnosticsError() {
		if (this.hasDiagnostics()) {
			throw new DiagnosticsError(
				"Thrown by DiagnosticsProcessor",
				this.getDiagnostics(),
			);
		}
	}

	public hasDiagnostics(): boolean {
		if (this.map.size === 0) {
			return false;
		}

		if (this.possibleCount > 0) {
			return true;
		}

		return this.getDiagnostics().length > 0;
	}

	public addAllowedUnusedSuppressionPrefix(prefix: DiagnosticCategoryPrefix) {
		this.assertEmpty();
		this.allowedUnusedSuppressionPrefixes.add(prefix);
	}

	public addSuppressions(suppressions: DiagnosticSuppressions) {
		if (suppressions.length === 0) {
			return;
		}

		this.cachedFlatDiagnostics = undefined;
		for (const rawSuppression of suppressions) {
			const suppression = this.normalizer.normalizeSuppression(rawSuppression);
			const entry = this.getMapEntry(suppression.path);
			entry.suppressions.add(suppression);
			entry.cachedCalculated = undefined;
		}
	}

	public addFilters(filters: DiagnosticFilterWithTest[]) {
		if (this.map.size > 0) {
			throw new Error(
				"DiagnosticProcessor: Filters cannot be added after diagnostics already injected",
			);
		}
		this.filters = this.filters.concat(filters);
	}

	public addFilter(filter: DiagnosticFilterWithTest) {
		this.addFilters([filter]);
	}

	private doesMatchFilter(diag: Diagnostic): boolean {
		for (const filter of this.filters) {
			if (
				filter.message !== undefined &&
				readMarkup(filter.message) !== readMarkup(diag.description.message)
			) {
				continue;
			}

			if (
				filter.path !== undefined &&
				!equalPaths(filter.path, diag.location.path)
			) {
				continue;
			}

			if (
				filter.category !== undefined &&
				filter.category !== diag.description.category
			) {
				continue;
			}

			if (filter.start !== undefined && diag.location.start !== undefined) {
				if (
					filter.start.line !== diag.location.start.line ||
					filter.start.column !== diag.location.start.column
				) {
					continue;
				}
			}

			if (
				filter.line !== undefined &&
				diag.location.start !== undefined &&
				diag.location.start.line !== filter.line
			) {
				continue;
			}

			if (filter.test !== undefined && filter.test(diag)) {
				continue;
			}

			return true;
		}

		return false;
	}

	private getDiagnosticVisibility(
		diag: Diagnostic,
		{
			dedupeKeys,
			includeSuppressions,
			unusedSuppressions,
		}: {
			dedupeKeys: SeenKeys;
			includeSuppressions: boolean;
			unusedSuppressions?: Set<DiagnosticSuppression>;
		},
	): DiagnosticVisibility {
		const entry = this.getMapEntry(diag.location.path);

		if (this.doesMatchFilter(diag)) {
			return "hidden";
		}

		if (
			includeSuppressions &&
			doesMatchSuppression(diag, entry.suppressions, unusedSuppressions)
		) {
			return "hidden";
		}

		if (isDeduped(diag, dedupeKeys)) {
			return "hidden";
		}

		if (diag.dependencies !== undefined) {
			for (const dep of diag.dependencies) {
				if (
					this.hasDiagnosticsForPath(dep.path) &&
					this.getMapEntry(dep.path).suppressDependents
				) {
					return "hidden";
				}
			}

			// We know this diagnostic wont always be visible and could be hidden by dependencies
			return "maybe";
		}

		return "guaranteed";
	}

	public addDiagnostic(diag: Diagnostic, origin?: DiagnosticOrigin): void {
		this.addDiagnostics([diag], origin);
	}

	public addDiagnostics(diags: Diagnostics, origin?: DiagnosticOrigin): void {
		if (diags.length === 0) {
			return;
		}

		this.cachedFlatDiagnostics = undefined;

		// Add origins to diagnostics
		const origins: DiagnosticOrigin[] = [...this.origins];
		if (origin !== undefined) {
			origins.push(origin);
		}
		diags = addOriginsToDiagnostics(origins, diags);

		// Normalize
		diags = diags.map((diag) => this.normalizer.normalizeDiagnostic(diag));

		let guaranteed: undefined | Diagnostics;
		if (this.guaranteedDiagnosticsEvent.hasSubscriptions()) {
			guaranteed = [];
		}

		for (let diag of diags) {
			const {category} = diag.description;
			const {path} = diag.location;

			const entry = this.getMapEntry(path);

			entry.cachedCalculated = undefined;
			entry.diagnostics.add(diag);

			if (DIAGNOSTIC_CATEGORIES_SUPPRESS_DEPENDENCIES.has(category)) {
				entry.suppressDependents = true;
			}

			if (diag.dependencies !== undefined) {
				for (const dep of diag.dependencies) {
					entry.dependencies.add(dep.path);
					this.getMapEntry(dep.path).dependents.add(path);
				}
			}

			const visibility = this.getDiagnosticVisibility(
				diag,
				{dedupeKeys: entry.dedupeKeys, includeSuppressions: true},
			);
			if (visibility !== "hidden") {
				this.possibleCount++;
				entry.possibleCount++;
			}
			if (visibility === "guaranteed" && guaranteed !== undefined) {
				guaranteed.push(diag);
			}
		}

		if (guaranteed !== undefined && guaranteed.length > 0) {
			this.guaranteedDiagnosticsEvent.send(guaranteed);
		}
		this.insertDiagnosticsEvent.send();

		const {throwAfter} = this;
		if (throwAfter !== undefined) {
			this.maybeThrowDiagnosticsError();
		}
	}

	public getPaths(): Iterable<AnyPath> {
		return this.map.keys();
	}

	public hasDiagnosticsForPath(path: AnyPath): boolean {
		return this.map.has(path);
	}

	public getSuppressionsForPath(
		path: AnyPath,
	): undefined | DiagnosticSuppressions {
		if (this.map.has(path)) {
			return Array.from(this.getMapEntry(path).suppressions);
		} else {
			return undefined;
		}
	}

	public getAllDiagnosticsForPath(path: AnyPath): Diagnostics {
		const calculated = this.getDiagnosticsForPath(path, true);
		if (calculated === undefined) {
			return [];
		} else {
			return calculated.complete;
		}
	}

	public getDiagnosticsForPath(
		path: AnyPath,
		includeSuppressions: boolean = true,
	): undefined | CalculatedDiagnostics {
		const entry = this.map.get(path);
		if (entry === undefined) {
			return undefined;
		}

		if (entry.cachedCalculated?.includedSuppressions === includeSuppressions) {
			return entry.cachedCalculated.value;
		}

		const complete: Diagnostics = [];
		const guaranteed: Diagnostics = [];

		const unusedSuppressions: Set<DiagnosticSuppression> = new Set(
			entry.suppressions,
		);
		const dedupeKeys: SeenKeys = new Set();

		for (const diag of entry.diagnostics) {
			const visibility = this.getDiagnosticVisibility(
				diag,
				{dedupeKeys, unusedSuppressions, includeSuppressions},
			);
			if (visibility === "hidden") {
				continue;
			}

			if (visibility === "guaranteed") {
				guaranteed.push(diag);
			}

			complete.push(diag);
		}

		// Add errors for unused suppressions
		for (const suppression of unusedSuppressions) {
			const categoryPrefix = suppression.category[0];
			if (this.allowedUnusedSuppressionPrefixes.has(categoryPrefix)) {
				continue;
			}

			complete.push(
				this.normalizer.normalizeDiagnostic({
					location: suppression.loc,
					description: descriptions.SUPPRESSIONS.UNUSED(suppression),
				}),
			);
		}

		const calculated: CalculatedDiagnostics = {
			complete,
			guaranteed,
			suppressions: Array.from(entry.suppressions),
		};
		entry.cachedCalculated = {
			value: calculated,
			includedSuppressions: includeSuppressions,
		};
		return calculated;
	}

	public removePath(path: AnyPath) {
		if (!this.map.has(path)) {
			return;
		}

		const entry = this.getMapEntry(path);

		this.possibleCount -= entry.possibleCount;
		this.map.delete(path);
		this.normalizer.removePath(path);
		this.modifiedDiagnosticsForPathEvent.send(path);

		// Some diagnostics may now be visible on dependents
		if (entry.suppressDependents) {
			for (const path of entry.dependents) {
				this.modifiedDiagnosticsForPathEvent.send(path);
			}
		}

		// Remove us from our dependencies
		for (const depPath of entry.dependencies) {
			if (this.hasDiagnosticsForPath(depPath)) {
				this.getMapEntry(depPath).dependents.delete(path);
			}
		}
	}

	public getDiagnostics(): Diagnostics {
		const {cachedFlatDiagnostics: cachedDiagnostics} = this;
		if (cachedDiagnostics !== undefined) {
			return cachedDiagnostics;
		}

		let diagnostics: Diagnostics = [];

		for (const path of this.map.keys()) {
			const pathDiagnostics = this.getDiagnosticsForPath(path);
			if (pathDiagnostics !== undefined) {
				diagnostics = [...diagnostics, ...pathDiagnostics.complete];
			}
		}

		this.cachedFlatDiagnostics = diagnostics;

		return diagnostics;
	}
}
