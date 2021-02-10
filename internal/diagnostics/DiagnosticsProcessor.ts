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
import {DiagnosticsError} from "./errors";
import {
	DIAGNOSTIC_CATEGORIES_SUPPRESS_DEPENDENCIES,
	DiagnosticCategoryPrefix,
} from "./categories";
import {descriptions} from "./descriptions";
import {matchesSuppression} from "@internal/compiler";
import {SourceMapConsumerCollection} from "@internal/codec-source-map";
import DiagnosticsNormalizer, {DiagnosticsNormalizerOptions} from "./DiagnosticsNormalizer";
import {MarkupFormatNormalizeOptions, readMarkup} from "@internal/markup";
import {UnknownPathMap, UnknownPathSet, equalPaths} from "@internal/path";

type UniquePart =
	| "filename"
	| "message"
	| "start.line"
	| "start.column"
	| "category"
	| "label";

type UniqueRule = UniquePart[];

type UniqueRules = UniqueRule[];

export type DiagnosticsProcessorOptions = {
	filters?: DiagnosticFilterWithTest[];
	unique?: UniqueRules;
	onDiagnostics?: (diags: Diagnostics) => void;
	origins?: DiagnosticOrigin[];
	markupOptions?: MarkupFormatNormalizeOptions;
	normalizeOptions?: DiagnosticsNormalizerOptions;
	sourceMaps?: SourceMapConsumerCollection;
};

const DEFAULT_UNIQUE: UniqueRules = [
	["label", "category", "filename", "message", "start.line", "start.column"],
];

type DiagnosticsByPath = {
	map: UnknownPathMap<Diagnostics>;
	pathless: Diagnostics;
};

export default class DiagnosticsProcessor {
	constructor(options: DiagnosticsProcessorOptions = {}) {
		this.filters = [];
		this.options = options;
		this.includedKeys = new Set();
		this.unique = options.unique === undefined ? DEFAULT_UNIQUE : options.unique;
		this.throwAfter = undefined;
		this.origins = options.origins === undefined ? [] : [...options.origins];
		this.allowedUnusedSuppressionPrefixes = new Set();
		this.usedSuppressions = new Set();
		this.suppressions = new Set();
		this.sourceMaps = options.sourceMaps ?? new SourceMapConsumerCollection();
		this.normalizer = new DiagnosticsNormalizer(
			options.normalizeOptions,
			options.markupOptions,
			this.sourceMaps,
		);

		this.ignoreDiagnosticsForDependentsOf = new UnknownPathSet();

		this.diagnostics = [];
		this.cachedDiagnostics = undefined;
	}

	public normalizer: DiagnosticsNormalizer;

	private ignoreDiagnosticsForDependentsOf: UnknownPathSet;

	private sourceMaps: SourceMapConsumerCollection;
	private unique: UniqueRules;
	private includedKeys: Set<string>;
	private diagnostics: Diagnostic[];
	private filters: DiagnosticFilterWithTest[];
	private allowedUnusedSuppressionPrefixes: Set<string>;
	private usedSuppressions: Set<DiagnosticSuppression>;
	private suppressions: Set<DiagnosticSuppression>;
	private options: DiagnosticsProcessorOptions;
	private throwAfter: undefined | number;
	private origins: DiagnosticOrigin[];
	private cachedDiagnostics: undefined | Diagnostics;

	private assertEmpty() {
		if (this.hasDiagnostics()) {
			throw new Error("Expected no diagnostics for this operation");
		}
	}

	public static createImmediateThrower(
		origins: DiagnosticOrigin[],
	): DiagnosticsProcessor {
		const diagnostics = new DiagnosticsProcessor({
			origins,
			onDiagnostics() {
				diagnostics.maybeThrowDiagnosticsError();
			},
		});
		return diagnostics;
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
		return this.getDiagnostics().length > 0;
	}

	public addAllowedUnusedSuppressionPrefix(prefix: DiagnosticCategoryPrefix) {
		this.assertEmpty();
		this.allowedUnusedSuppressionPrefixes.add(prefix);
	}

	public addSuppressions(suppressions: DiagnosticSuppressions) {
		this.cachedDiagnostics = undefined;
		for (const suppression of suppressions) {
			this.suppressions.add(suppression);
		}
	}

	public addFilters(filters: DiagnosticFilterWithTest[]) {
		this.cachedDiagnostics = undefined;
		this.filters = this.filters.concat(filters);
	}

	public addFilter(filter: DiagnosticFilterWithTest) {
		this.cachedDiagnostics = undefined;
		this.filters.push(filter);
	}

	private doesMatchFilter(diag: Diagnostic): boolean {
		for (const suppression of this.suppressions) {
			if (
				matchesSuppression(
					diag.description.category,
					diag.description.categoryValue,
					diag.location,
					suppression,
				)
			) {
				this.usedSuppressions.add(suppression);
				return true;
			}
		}

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

	private buildDedupeKeys(diag: Diagnostic): string[] {
		if (diag.tags?.unique) {
			return [];
		}

		// We don't do anything with `end` in this method, it's fairly meaningless for deduping errors
		let {start} = diag.location;

		const keys: string[] = [];

		for (const rule of this.unique) {
			const parts = [];

			if (rule.includes("label")) {
				parts.push(
					`label:${diag.label === undefined ? "" : readMarkup(diag.label)}`,
				);
			}

			if (rule.includes("category")) {
				parts.push(`category:${diag.description.category}`);
			}

			if (rule.includes("filename")) {
				parts.push(`filename:${String(diag.location.path?.join())}`);
			}

			if (rule.includes("message")) {
				parts.push(`message:${readMarkup(diag.description.message)}`);
			}

			if (start !== undefined) {
				if (rule.includes("start.line")) {
					parts.push(`start.line:${start.line}`);
				}

				if (rule.includes("start.column")) {
					parts.push(`start.column:${start.column}`);
				}
			}

			const key = parts.join(",");
			keys.push(key);
		}

		return keys;
	}

	public addDiagnosticAssert(
		diag: Diagnostic,
		origin?: DiagnosticOrigin,
	): Diagnostic {
		return this.addDiagnostics([diag], origin, true)[0];
	}

	public addDiagnostic(
		diag: Diagnostic,
		origin?: DiagnosticOrigin,
	): undefined | Diagnostic {
		return this.addDiagnostics([diag], origin)[0];
	}

	public addDiagnostics(
		diags: Diagnostics,
		origin?: DiagnosticOrigin,
		force?: boolean,
	): Diagnostics {
		if (diags.length === 0) {
			return diags;
		}

		this.cachedDiagnostics = undefined;

		const added: Diagnostics = [];

		// Add origins to diagnostics
		const origins: DiagnosticOrigin[] = [...this.origins];
		if (origin !== undefined) {
			origins.push(origin);
		}
		diags = addOriginsToDiagnostics(origins, diags);

		// Filter diagnostics
		diagLoop: for (let diag of diags) {
			// Check before normalization
			if (!force && this.doesMatchFilter(diag)) {
				continue;
			}

			diag = this.normalizer.normalizeDiagnostic(diag);

			// Check after normalization
			if (!force && this.doesMatchFilter(diag)) {
				continue;
			}

			const keys = this.buildDedupeKeys(diag);

			if (!force) {
				for (const key of keys) {
					if (this.includedKeys.has(key)) {
						continue diagLoop;
					}
				}
			}

			if (
				DIAGNOSTIC_CATEGORIES_SUPPRESS_DEPENDENCIES.has(
					diag.description.category,
				)
			) {
				const {path} = diag.location;
				if (path !== undefined) {
					this.ignoreDiagnosticsForDependentsOf.add(path);
				}
			}

			this.diagnostics.push(diag);
			added.push(diag);

			for (const key of keys) {
				this.includedKeys.add(key);
			}
		}

		const {onDiagnostics} = this.options;
		if (onDiagnostics !== undefined && added.length > 0) {
			onDiagnostics(added);
		}

		const {throwAfter} = this;
		if (throwAfter !== undefined && this.diagnostics.length >= throwAfter) {
			this.maybeThrowDiagnosticsError();
		}

		return added;
	}

	public getDiagnosticsByPath(): DiagnosticsByPath {
		const byPath: DiagnosticsByPath = {
			map: new UnknownPathMap(),
			pathless: [],
		};

		for (const diag of this.getDiagnostics()) {
			const {path} = diag.location;

			if (path === undefined) {
				byPath.pathless.push(diag);
				continue;
			}

			let pathDiagnostics = byPath.map.get(path);
			if (pathDiagnostics === undefined) {
				pathDiagnostics = [];
				byPath.map.set(path, pathDiagnostics);
			}
			pathDiagnostics.push(diag);
		}

		return byPath;
	}

	public getDiagnostics(): Diagnostics {
		const {cachedDiagnostics} = this;
		if (cachedDiagnostics !== undefined) {
			return cachedDiagnostics;
		}

		const diagnostics: Diagnostics = [];

		for (const diag of this.diagnostics) {
			let ignore = false;

			// Ignore diagnostics that are dependents of a file with a prioritized diagnostic category
			if (diag.dependencies !== undefined) {
				for (const dep of diag.dependencies) {
					if (this.ignoreDiagnosticsForDependentsOf.has(dep.path)) {
						ignore = true;
						break;
					}
				}
			}

			if (!ignore) {
				diagnostics.push(diag);
			}
		}

		// Add errors for remaining suppressions
		for (const suppression of this.suppressions) {
			if (this.usedSuppressions.has(suppression)) {
				continue;
			}

			const [categoryPrefix] = suppression.category.split("/");
			if (this.allowedUnusedSuppressionPrefixes.has(categoryPrefix)) {
				continue;
			}

			diagnostics.push(
				this.normalizer.normalizeDiagnostic({
					location: suppression.loc,
					description: descriptions.SUPPRESSIONS.UNUSED(suppression),
				}),
			);
		}

		this.cachedDiagnostics = diagnostics;

		return diagnostics;
	}
}
