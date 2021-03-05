/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticEliminationFilterWithTest,
	DiagnosticOrigin,
	DiagnosticSuppression,
	DiagnosticSuppressions,
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
import {MixedPathMap, MixedPathSet, Path, equalPaths} from "@internal/path";
import {Event} from "@internal/events";
import {formatCategoryDescription} from "./helpers";
import {markupToJoinedPlainText} from "@internal/cli-layout";

export type DiagnosticsProcessorOptions = {
	mutable?: boolean;
	filters?: DiagnosticEliminationFilterWithTest[];
	origins?: DiagnosticOrigin[];
	markupOptions?: MarkupFormatNormalizeOptions;
	normalizeOptions?: DiagnosticsNormalizerOptions;
	sourceMaps?: SourceMapConsumerCollection;
	filter?: DiagnosticsProcessorFilterOptions;
};

export type DiagnosticsProcessorFilterOptions = {
	grep: string;
	inverseGrep: boolean;
	maxDiagnostics: number;
};

// Recommended defaults. We do not use these actually by default when constructing bare instances.
export const DEFAULT_PROCESSOR_FILTER_OPTIONS: DiagnosticsProcessorFilterOptions = {
	maxDiagnostics: 20,
	grep: "",
	inverseGrep: false,
};

type DiagnosticsMapEntry = {
	cachedCalculated:
		| undefined
		| {
				raw: boolean;
				value: DiagnosticsProcessorCalculatedPath;
			};

	dependencies: MixedPathSet;
	dependents: MixedPathSet;
	suppressDependents: boolean;

	possibleCount: number;
	filteredCount: number;
	truncatedCount: number;
	guaranteedCount: number;

	dedupeKeys: SeenKeys;

	guaranteedDiagnostics: Diagnostic[];
	maybeDiagnostics: Diagnostic[];
	hiddenDiagnostics: Diagnostic[];

	suppressions: DiagnosticSuppression[];
	unusedSuppressions: Set<DiagnosticSuppression>;
};

type DiagnosticsMap = MixedPathMap<DiagnosticsMapEntry>;

export type DiagnosticsProcessorCalculatedPath = {
	suppressions: DiagnosticSuppressions;
	guaranteed: Diagnostic[];
	complete: Diagnostic[];
};

type DiagnosticVisibility = {
	// Eliminated by an explicit programmatic filter. Pretend it doesn't exist.
	eliminated: boolean;
	// Hidden from view, dependent on a file with an existing important diagnostic, deduped etc.
	hidden: boolean;
	// User comment suppression
	suppressed: boolean;
	// Filtered by display flag
	filtered: boolean;
	// Truncated due to too many elements
	truncated: boolean;
	// Possibility of being displayed, not enough information to determine or guaranteed
	maybe: boolean;
	// Will definitely be in the final output as it does not satisfy any of the previous conditions
	guaranteed: boolean;
};

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

export type DiagnosticsProcessorCalculated = {
	total: number;
	filtered: number;
	truncated: number;
	diagnostics: Diagnostic[];
};

export default class DiagnosticsProcessor {
	constructor(options: DiagnosticsProcessorOptions = {}) {
		this.eliminationFilters = [];
		this.throwAfter = undefined;
		this.origins = options.origins === undefined ? [] : [...options.origins];
		this.allowedUnusedSuppressionPrefixes = new Set();
		this.normalizer = new DiagnosticsNormalizer(
			options.normalizeOptions,
			options.markupOptions,
			options.sourceMaps,
		);

		this.options = options;
		this.filter = options.filter;
		this.maxDiagnostics = options.filter?.maxDiagnostics ?? Infinity;

		this.map = new MixedPathMap();
		this.cachedDump = undefined;

		this.possibleCount = 0;
		this.guaranteedCount = 0;
		this.guaranteedTruncation = false;

		this.insertDiagnosticsEvent = new Event(
			"DiagnosticsProcessor.insertDiagnosticsEvent",
		);
		this.guaranteedDiagnosticsEvent = new Event(
			"DiagnosticsProcessor.visibleDiagnosticsEvent",
		);
		this.guaranteedTruncationEvent = new Event(
			"DiagnosticsProcessor.guaranteedTruncationEvent",
		);
		this.modifiedDiagnosticsForPathEvent = new Event(
			"DiagnosticsProcessor.modifiedDiagnosticsForPathEvent",
		);
	}

	public normalizer: DiagnosticsNormalizer;
	public insertDiagnosticsEvent: Event<void>;
	public guaranteedTruncationEvent: Event<boolean>;
	public guaranteedDiagnosticsEvent: Event<Diagnostic[]>;
	public modifiedDiagnosticsForPathEvent: Event<Path>;
	public filter: undefined | DiagnosticsProcessorFilterOptions;
	public guaranteedTruncation: boolean;

	private options: DiagnosticsProcessorOptions;
	private maxDiagnostics: number;
	private eliminationFilters: DiagnosticEliminationFilterWithTest[];
	private allowedUnusedSuppressionPrefixes: Set<string>;
	private throwAfter: undefined | number;
	private origins: DiagnosticOrigin[];

	private map: DiagnosticsMap;
	private cachedDump: undefined | DiagnosticsProcessorCalculated;
	private possibleCount: number;
	private guaranteedCount: number;

	private setGuaranteedCount(count: number): void {
		this.guaranteedCount = count;

		const prevTruncation = this.guaranteedTruncation;
		const newTruncation = count > this.maxDiagnostics;
		this.guaranteedTruncation = newTruncation;
		if (prevTruncation !== newTruncation) {
			this.guaranteedTruncationEvent.send(newTruncation);
		}
	}

	private getMapEntry(path: Path): DiagnosticsMapEntry {
		let entry: undefined | DiagnosticsMapEntry = this.map.get(path);
		if (entry === undefined) {
			entry = {
				cachedCalculated: undefined,
				dependencies: new MixedPathSet(),
				dependents: new MixedPathSet(),
				suppressDependents: false,
				guaranteedCount: 0,
				possibleCount: 0,
				truncatedCount: 0,
				filteredCount: 0,
				dedupeKeys: new Set(),
				suppressions: [],
				unusedSuppressions: new Set(),
				maybeDiagnostics: [],
				hiddenDiagnostics: [],
				guaranteedDiagnostics: [],
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
			console.trace();
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

		this.cachedDump = undefined;
		for (const rawSuppression of suppressions) {
			const suppression = this.normalizer.normalizeSuppression(rawSuppression);
			const entry = this.getMapEntry(suppression.path);
			entry.suppressions.push(suppression);
			entry.cachedCalculated = undefined;
		}
	}

	public addEliminationFilter(filter: DiagnosticEliminationFilterWithTest) {
		if (this.map.size > 0) {
			throw new Error(
				"DiagnosticsProcessor: Filters cannot be added after diagnostics already injected",
			);
		}
		this.eliminationFilters.push(filter);
	}

	private doesMatchFilter(diag: Diagnostic): boolean {
		for (const filter of this.eliminationFilters) {
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
			unusedSuppressions,
			allowTruncation,
			ignoreSuppressionsAndFilter,
		}: {
			dedupeKeys: SeenKeys;
			allowTruncation: boolean;
			unusedSuppressions: Set<DiagnosticSuppression>;
			ignoreSuppressionsAndFilter: boolean;
		},
	): DiagnosticVisibility {
		const entry = this.getMapEntry(diag.location.path);

		const visibility: DiagnosticVisibility = {
			eliminated: false,
			hidden: false,
			suppressed: false,
			guaranteed: false,
			maybe: false,
			filtered: false,
			truncated: false,
		};

		if (this.doesMatchFilter(diag)) {
			visibility.eliminated = true;
		}

		for (const suppression of entry.suppressions) {
			if (
				matchesSuppression(
					diag.description.category,
					diag.description.categoryValue,
					diag.location,
					suppression,
				)
			) {
				unusedSuppressions.delete(suppression);
				if (!ignoreSuppressionsAndFilter) {
					visibility.suppressed = true;
				}
			}
		}

		if (!ignoreSuppressionsAndFilter && this.shouldFilter(diag)) {
			visibility.filtered = true;
		}

		if (isDeduped(diag, dedupeKeys)) {
			visibility.hidden = true;
		}

		if (diag.dependencies !== undefined) {
			for (const dep of diag.dependencies) {
				if (
					this.hasDiagnosticsForPath(dep.path) &&
					this.getMapEntry(dep.path).suppressDependents
				) {
					visibility.hidden = true;
				}
			}
		}

		if (allowTruncation && this.guaranteedTruncation) {
			if (this.options.mutable) {
				// We aren't sure until the final calculate call, whether there will be removed paths that make this untruncated
				visibility.maybe = true;
			} else {
				visibility.truncated = true;
			}
		}

		if (diag.dependencies !== undefined) {
			// We know this diagnostic wont always be visible and could be hidden by dependencies
			visibility.maybe = true;
		}

		// Whether we have been rendered invisible
		const invisible =
			visibility.eliminated ||
			visibility.truncated ||
			visibility.suppressed ||
			visibility.filtered ||
			visibility.hidden;

		// If we have another flag then there's no way we're possibly being displayed
		if (invisible) {
			visibility.maybe = false;
		}

		// If we have no other flag, and have not hit a condition that explicitly makes us possibly invisible later, we will always be displayed
		if (!(invisible || visibility.maybe)) {
			visibility.guaranteed = true;
			visibility.maybe = true;
		}

		return visibility;
	}

	public addDiagnostic(diag: Diagnostic, origin?: DiagnosticOrigin): void {
		this.addDiagnostics([diag], origin);
	}

	public addDiagnostics(diags: Diagnostic[], origin?: DiagnosticOrigin): void {
		if (diags.length === 0) {
			return;
		}

		this.cachedDump = undefined;

		// Add origins to diagnostics
		const origins: DiagnosticOrigin[] = [...this.origins];
		if (origin !== undefined) {
			origins.push(origin);
		}
		diags = addOriginsToDiagnostics(origins, diags);

		// Normalize
		diags = diags.map((diag) => this.normalizer.normalizeDiagnostic(diag));

		let guaranteed: undefined | Diagnostic[];
		if (
			this.guaranteedDiagnosticsEvent.hasSubscriptions() &&
			!this.guaranteedTruncation
		) {
			guaranteed = [];
		}

		for (let diag of diags) {
			const {category} = diag.description;
			const {path} = diag.location;

			const entry = this.getMapEntry(path);

			entry.cachedCalculated = undefined;

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
				{
					dedupeKeys: entry.dedupeKeys,
					unusedSuppressions: entry.unusedSuppressions,
					allowTruncation: true,
					ignoreSuppressionsAndFilter: false,
				},
			);
			if (visibility.eliminated) {
				continue;
			}

			if (visibility.filtered) {
				entry.filteredCount++;
			} else if (visibility.truncated) {
				entry.truncatedCount++;
			}

			if (visibility.maybe) {
				this.possibleCount++;
				entry.possibleCount++;
			}

			if (visibility.guaranteed) {
				entry.guaranteedDiagnostics.push(diag);
				this.setGuaranteedCount(this.guaranteedCount + 1);
				entry.guaranteedCount++;
				if (guaranteed !== undefined) {
					guaranteed.push(diag);
				}
			} else if (visibility.maybe) {
				entry.maybeDiagnostics.push(diag);
			} else {
				entry.hiddenDiagnostics.push(diag);
			}
		}

		if (guaranteed !== undefined && guaranteed.length > 0) {
			this.guaranteedDiagnosticsEvent.send(guaranteed);
		}
		this.insertDiagnosticsEvent.send();

		const {throwAfter} = this;
		if (throwAfter !== undefined && this.guaranteedCount >= throwAfter) {
			this.maybeThrowDiagnosticsError();
		}
	}

	public getPaths(): Iterable<Path> {
		return this.map.keys();
	}

	public hasDiagnosticsForPath(path: Path): boolean {
		return this.map.has(path);
	}

	public getSuppressionsForPath(path: Path): undefined | DiagnosticSuppressions {
		if (this.map.has(path)) {
			return Array.from(this.getMapEntry(path).suppressions);
		} else {
			return undefined;
		}
	}

	public getDiagnosticsForPath(path: Path): Diagnostic[] {
		const calculated = this.calculatePath(path, {raw: false});
		if (calculated === undefined) {
			return [];
		} else {
			return calculated.complete;
		}
	}

	public calculatePath(
		path: Path,
		{raw}: {
			raw: boolean;
		},
	): undefined | DiagnosticsProcessorCalculatedPath {
		const entry = this.map.get(path);
		if (entry === undefined) {
			return undefined;
		}

		if (entry.cachedCalculated?.raw === raw) {
			return entry.cachedCalculated.value;
		}

		const unusedSuppressions = new Set(entry.unusedSuppressions);
		const guaranteed: Diagnostic[] = [...entry.guaranteedDiagnostics];
		let complete: Diagnostic[] = [...entry.guaranteedDiagnostics];

		const dedupeKeys: SeenKeys = new Set();

		let recheckDiagnostics = entry.maybeDiagnostics;
		if (raw) {
			recheckDiagnostics = [...recheckDiagnostics, ...entry.hiddenDiagnostics];
		}

		for (const diag of recheckDiagnostics) {
			const visibility = this.getDiagnosticVisibility(
				diag,
				{
					dedupeKeys,
					allowTruncation: false,
					unusedSuppressions,
					ignoreSuppressionsAndFilter: raw,
				},
			);

			// A maybe at this point is guaranteed
			if (visibility.maybe) {
				complete.push(diag);
			}
		}

		// Add errors for unused suppressions
		if (!raw) {
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
		}

		const calculated: DiagnosticsProcessorCalculatedPath = {
			complete,
			guaranteed,
			suppressions: Array.from(entry.suppressions),
		};
		entry.cachedCalculated = {
			value: calculated,
			raw,
		};
		return calculated;
	}

	public removePath(path: Path) {
		if (!this.options.mutable) {
			throw new Error(
				"DiagnosticsProcessor: `options.mutable` must be set in order to remove a path",
			);
		}

		if (!this.map.has(path)) {
			return;
		}

		const entry = this.getMapEntry(path);

		this.possibleCount -= entry.possibleCount;
		this.setGuaranteedCount(this.guaranteedCount - entry.guaranteedCount);
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

	private shouldFilter(diag: Diagnostic): boolean {
		if (this.filter === undefined) {
			return false;
		}

		const {grep, inverseGrep} = this.filter;

		// An empty grep pattern means show everything
		if (grep === undefined || grep === "") {
			return false;
		}

		// Match against the supplied grep pattern
		let ignored =
			markupToJoinedPlainText(diag.description.message).toLowerCase().includes(
				grep,
			) === false;
		if (inverseGrep) {
			ignored = !ignored;
		}
		return ignored;
	}

	public calculate(): DiagnosticsProcessorCalculated {
		const cached = this.cachedDump;
		if (cached !== undefined) {
			return cached;
		}

		let calculated: DiagnosticsProcessorCalculated = {
			total: 0,
			filtered: 0,
			truncated: 0,
			diagnostics: [],
		};

		const {maxDiagnostics} = this;
		let truncated = false;

		for (const [path, entry] of this.map) {
			const pathCalculated = this.calculatePath(
				path,
				{
					raw: false,
				},
			);
			if (pathCalculated === undefined) {
				continue;
			}

			const diagnostics = pathCalculated.complete;
			calculated.total += diagnostics.length;
			calculated.total += entry.filteredCount;
			calculated.total += entry.truncatedCount;
			calculated.filtered += entry.filteredCount;
			calculated.truncated += entry.truncatedCount;

			if (truncated) {
				calculated.truncated += diagnostics.length;
			} else {
				calculated.diagnostics = [...calculated.diagnostics, ...diagnostics];

				const newLength = calculated.diagnostics.length;
				if (newLength > maxDiagnostics) {
					calculated.truncated += newLength - maxDiagnostics;
					calculated.diagnostics = calculated.diagnostics.slice(
						0,
						maxDiagnostics,
					);
					truncated = true;
				}
			}
		}

		this.cachedDump = calculated;

		return calculated;
	}

	public getDiagnostics(): Diagnostic[] {
		return this.calculate().diagnostics;
	}
}
