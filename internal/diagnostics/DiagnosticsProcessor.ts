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
import {DiagnosticCategoryPrefix} from "./categories";
import {descriptions} from "./descriptions";
import {matchesSuppression} from "@internal/compiler";
import {SourceMapConsumerCollection} from "@internal/codec-source-map";
import DiagnosticsNormalizer, {DiagnosticsNormalizerOptions} from "./DiagnosticsNormalizer";
import {MarkupFormatNormalizeOptions, readMarkup} from "@internal/markup";
import {ExtendedMap} from "@internal/collections";

type UniquePart =
	| "filename"
	| "message"
	| "start.line"
	| "start.column"
	| "category"
	| "label";

type UniqueRule = Array<UniquePart>;

type UniqueRules = Array<UniqueRule>;

export type DiagnosticsProcessorOptions = {
	filters?: Array<DiagnosticFilterWithTest>;
	unique?: UniqueRules;
	max?: number;
	onDiagnostics?: (diags: Diagnostics) => void;
	origins?: Array<DiagnosticOrigin>;
	markupOptions?: MarkupFormatNormalizeOptions;
	normalizeOptions?: DiagnosticsNormalizerOptions;
	sourceMaps?: SourceMapConsumerCollection;
};

const DEFAULT_UNIQUE: UniqueRules = [
	["label", "category", "filename", "message", "start.line", "start.column"],
];

type DiagnosticsByFilename = ExtendedMap<undefined | string, Diagnostics>;

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

		this.diagnostics = new Set();
		this.cachedDiagnostics = undefined;
	}

	public normalizer: DiagnosticsNormalizer;

	private sourceMaps: SourceMapConsumerCollection;
	private unique: UniqueRules;
	private includedKeys: Set<string>;
	private diagnostics: Set<Diagnostic>;
	private filters: Array<DiagnosticFilterWithTest>;
	private allowedUnusedSuppressionPrefixes: Set<string>;
	private usedSuppressions: Set<DiagnosticSuppression>;
	private suppressions: Set<DiagnosticSuppression>;
	private options: DiagnosticsProcessorOptions;
	private throwAfter: undefined | number;
	private origins: Array<DiagnosticOrigin>;
	private cachedDiagnostics: undefined | Diagnostics;

	private assertEmpty() {
		if (this.hasDiagnostics()) {
			throw new Error("Expected no diagnostics for this operation");
		}
	}

	public static createImmediateThrower(
		origins: Array<DiagnosticOrigin>,
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

	public addFilters(filters: Array<DiagnosticFilterWithTest>) {
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
				filter.filename !== undefined &&
				filter.filename !== diag.location.filename
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

	private buildDedupeKeys(diag: Diagnostic): Array<string> {
		if (diag.tags && diag.tags.unique) {
			return [];
		}

		// We don't do anything with `end` in this method, it's fairly meaningless for deduping errors
		let {start} = diag.location;

		const keys: Array<string> = [];

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
				parts.push(`filename:${String(diag.location.filename)}`);
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

	public deleteDiagnostic(diag: Diagnostic) {
		this.diagnostics.delete(diag);
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

		const {max} = this.options;
		const added: Diagnostics = [];

		// Add origins to diagnostics
		const origins: Array<DiagnosticOrigin> = [...this.origins];
		if (origin !== undefined) {
			origins.push(origin);
		}
		diags = addOriginsToDiagnostics(origins, diags);

		// Filter diagnostics
		diagLoop: for (let diag of diags) {
			if (!force && max !== undefined && this.diagnostics.size > max) {
				break;
			}

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

			this.diagnostics.add(diag);
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
		if (throwAfter !== undefined && this.diagnostics.size >= throwAfter) {
			this.maybeThrowDiagnosticsError();
		}

		return added;
	}

	public getDiagnosticsByFilename(): DiagnosticsByFilename {
		const byFilename: DiagnosticsByFilename = new ExtendedMap(
			"diagnosticsByFilename",
			() => [],
		);

		for (const diag of this.getDiagnostics()) {
			const {filename} = diag.location;

			const filenameDiagnostics = byFilename.assert(filename);
			filenameDiagnostics.push(diag);
		}

		return byFilename;
	}

	public getDiagnostics(): Diagnostics {
		const {cachedDiagnostics} = this;
		if (cachedDiagnostics !== undefined) {
			return cachedDiagnostics;
		}

		const diagnostics: Diagnostics = [...this.diagnostics];

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
					location: suppression.commentLocation,
					description: descriptions.SUPPRESSIONS.UNUSED(suppression),
				}),
			);
		}

		this.cachedDiagnostics = diagnostics;

		return diagnostics;
	}
}
