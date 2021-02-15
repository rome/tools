/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticAdviceItem,
	DiagnosticDependencies,
	DiagnosticDescription,
	DiagnosticLocation,
	DiagnosticSuppression,
	DiagnosticTags,
} from "./types";
import {SourceMapConsumerCollection} from "@internal/codec-source-map";
import {
	MarkupFormatNormalizeOptions,
	StaticMarkup,
	markup,
	normalizeMarkup,
} from "@internal/markup";
import {OneIndexed, ZeroIndexed} from "@internal/math";
import {mergeObjects} from "@internal/typescript-helpers";
import {AnyPath, MixedPathMap, MixedPathSet} from "@internal/path";

export type DiagnosticsNormalizerOptions = {
	tags?: DiagnosticTags;
	label?: StaticMarkup;
};

function maybeMerge<T extends object>(a: T, b: Partial<T>): T {
	for (let key in b) {
		if (a[key] !== b[key]) {
			return {
				...a,
				...b,
			};
		}
	}

	return a;
}

function maybeMap<T>(arr: T[], callback: (item: T) => T): T[] {
	let modified = false;

	const normalized = arr.map((item) => {
		const mapped = callback(item);
		if (mapped !== item) {
			modified = true;
		}
		return mapped;
	});

	return modified ? normalized : arr;
}

export default class DiagnosticsNormalizer {
	constructor(
		normalizeOptions: DiagnosticsNormalizerOptions = {},
		markupOptions?: MarkupFormatNormalizeOptions,
		sourceMaps?: SourceMapConsumerCollection,
	) {
		this.sourceMaps = sourceMaps;
		this.inlineSourceText = new MixedPathMap();
		this.options = normalizeOptions;
		this.inlinedSourceTextFilenames = new MixedPathSet();
		this.markupOptions = this.createMarkupOptions(markupOptions);
	}

	private sourceMaps: undefined | SourceMapConsumerCollection;

	private options: DiagnosticsNormalizerOptions;
	private markupOptions: MarkupFormatNormalizeOptions;

	private inlineSourceText: MixedPathMap<string>;
	private inlinedSourceTextFilenames: MixedPathSet;

	public removePath(path: AnyPath) {
		this.inlineSourceText.delete(path);
		this.inlinedSourceTextFilenames.delete(path);
	}

	private createMarkupOptions(
		markupOptions: MarkupFormatNormalizeOptions = {},
	): MarkupFormatNormalizeOptions {
		const {sourceMaps} = this;

		return {
			...markupOptions,
			normalizePosition: (path, line, column) => {
				if (markupOptions?.normalizePosition !== undefined) {
					({path, line, column} = markupOptions.normalizePosition(
						path,
						line,
						column,
					));
				}

				if (sourceMaps !== undefined) {
					// line and column can be undefined so we do some weirdness to try and get only the filename if possible
					// using some default positions and then we'll toss whatever positions they return
					const resolved = sourceMaps.approxOriginalPositionFor(
						path,
						line ?? new OneIndexed(),
						column ?? new ZeroIndexed(),
					);
					if (resolved !== undefined) {
						return {
							path: resolved.source,
							line: line === undefined ? undefined : resolved.line,
							column: column === undefined ? undefined : resolved.column,
						};
					}
				}

				return {path, line, column};
			},
		};
	}

	public setInlineSourceText(path: AnyPath, sourceText: string) {
		this.inlineSourceText.set(path, sourceText);
	}

	private normalizePath(path: AnyPath): AnyPath;
	private normalizePath(path: undefined | AnyPath): undefined | AnyPath;
	private normalizePath(path: undefined | AnyPath): undefined | AnyPath {
		const {markupOptions} = this;
		if (markupOptions === undefined || path === undefined) {
			return path;
		}

		const {normalizePosition} = markupOptions;
		if (normalizePosition === undefined) {
			return path;
		}

		let normalizedPath = normalizePosition(path, undefined, undefined).path;
		if (normalizedPath.equal(path)) {
			return path;
		} else {
			return normalizedPath;
		}
	}

	private normalizePositionValue<T>(value: T): undefined | T {
		if (this.markupOptions !== undefined && this.markupOptions.stripPositions) {
			return undefined;
		} else {
			return value;
		}
	}

	public normalizeLocation(location: DiagnosticLocation): DiagnosticLocation {
		const {sourceMaps} = this;
		if (sourceMaps === undefined) {
			return location;
		}

		let {marker, path, start, end, integrity} = location;
		let origPath = path;

		if (path !== undefined && origPath !== undefined && sourceMaps !== undefined) {
			if (start !== undefined) {
				const resolved = sourceMaps.approxOriginalPositionFor(
					origPath,
					start.line,
					start.column,
				);
				if (resolved !== undefined) {
					path = resolved.source;
					start = mergeObjects(
						start,
						{
							line: resolved.line,
							column: resolved.column,
						},
					);
				}
			}

			if (end !== undefined) {
				const resolved = sourceMaps.approxOriginalPositionFor(
					origPath,
					end.line,
					end.column,
				);
				if (resolved !== undefined) {
					// TODO confirm this is the same as `start` if it resolved
					path = resolved.source;
					end = mergeObjects(
						end,
						{
							line: resolved.line,
							column: resolved.column,
						},
					);
				}
			}
		}

		const normalizedPath = this.normalizePath(path);

		// Inline sourceText. We keep track of filenames we've already inlined to avoid duplicating sourceText
		// During printing we'll fill it back in
		let {sourceText} = location;
		if (path !== undefined && !this.inlinedSourceTextFilenames.has(path)) {
			sourceText = sourceText ?? this.inlineSourceText.get(path) ?? this.inlineSourceText.get(normalizedPath);

			if (location.sourceText !== undefined && location.sourceText !== sourceText) {
				throw new Error(
					`Found multiple sourceText entries for ${path.join()} that did not match`,
				);
			}

			// Remove sourceText if it's not pointing anywhere
			if (start === undefined && end === undefined) {
				sourceText = undefined;
			}

			// Register filename as inlined if necessary
			if (sourceText !== undefined) {
				this.inlinedSourceTextFilenames.add(path);

				if (normalizedPath !== undefined) {
					this.inlinedSourceTextFilenames.add(normalizedPath);
				}
			}
		}

		marker = this.maybeNormalizeMarkup(marker);
		start = this.normalizePositionValue(start);
		end = this.normalizePositionValue(end);

		return maybeMerge(
			location,
			{
				integrity,
				sourceText,
				path: normalizedPath,
				marker,
				start,
				end,
			},
		);
	}

	private normalizeMarkup(markup: StaticMarkup): StaticMarkup {
		return normalizeMarkup(markup, this.markupOptions).text;
	}

	private maybeNormalizeMarkup(
		markup: undefined | StaticMarkup,
	): undefined | StaticMarkup {
		return markup === undefined ? undefined : this.normalizeMarkup(markup);
	}

	private normalizeDependencies(
		deps: DiagnosticDependencies,
	): DiagnosticDependencies {
		return maybeMap(
			deps,
			(dep) => {
				return maybeMerge(
					dep,
					{
						path: this.normalizePath(dep.path),
					},
				);
			},
		);
	}

	private normalizeAdvice(advice: DiagnosticAdvice): DiagnosticAdvice {
		return maybeMap(
			advice,
			(item) => {
				return this.normalizeAdviceItem(item);
			},
		);
	}

	private normalizeAdviceItem(item: DiagnosticAdviceItem): DiagnosticAdviceItem {
		const {sourceMaps} = this;

		switch (item.type) {
			case "frame":
				return maybeMerge(
					item,
					{
						location: this.normalizeLocation(item.location),
					},
				);

			case "list":
				return maybeMerge(
					item,
					{
						list: maybeMap(item.list, (markup) => this.normalizeMarkup(markup)),
					},
				);

			case "log":
				return maybeMerge(
					item,
					{
						text: this.normalizeMarkup(item.text),
					},
				);

			case "action":
				if (
					this.markupOptions.stripPositions &&
					item.commandFlags !== undefined &&
					Object.keys(item.commandFlags).length > 0
				) {
					return {
						...item,
						// Command flags could have position information
						commandFlags: {},
					};
				} else {
					return item;
				}

			case "stacktrace": {
				let importantPaths: undefined | MixedPathSet = item.importantPaths;

				if (importantPaths !== undefined) {
					const existingPaths = Array.from(importantPaths);
					const newPaths = maybeMap(
						existingPaths,
						(path) => this.normalizePath(path),
					);

					if (newPaths !== existingPaths) {
						importantPaths = new MixedPathSet(newPaths);
					}
				}

				return maybeMerge(
					item,
					{
						importantPaths,
						frames: maybeMap(
							item.frames,
							(frame) => {
								const {path, line, column} = frame;

								if (
									path === undefined ||
									line === undefined ||
									column === undefined ||
									(sourceMaps !== undefined && !sourceMaps.has(path))
								) {
									return maybeMerge(
										frame,
										{
											line: this.normalizePositionValue(line),
											column: this.normalizePositionValue(column),
											path: this.normalizePath(path),
										},
									);
								}

								if (sourceMaps !== undefined) {
									const resolved = sourceMaps.approxOriginalPositionFor(
										path,
										line,
										column,
									);
									if (resolved !== undefined) {
										return maybeMerge(
											frame,
											{
												path: this.normalizePath(resolved.source),
												line: this.normalizePositionValue(resolved.line),
												column: this.normalizePositionValue(resolved.column),
											},
										);
									}
								}

								return frame;
							},
						),
					},
				);
			}
		}

		return item;
	}

	public normalizeSuppression(
		suppression: DiagnosticSuppression,
	): DiagnosticSuppression {
		return maybeMerge(
			suppression,
			{
				path: this.normalizePath(suppression.path),
			},
		);
	}

	private hasNormalize(): boolean {
		const {sourceMaps, markupOptions} = this;
		if (sourceMaps !== undefined && sourceMaps.hasAny()) {
			return true;
		}

		if (this.inlineSourceText.size > 0) {
			return true;
		}

		if (markupOptions.stripFilelinkText || markupOptions.stripPositions) {
			return true;
		}

		if (
			markupOptions.humanizeFilename !== undefined ||
			markupOptions.normalizePosition !== undefined
		) {
			return true;
		}

		return false;
	}

	private normalizeDescription(
		description: DiagnosticDescription,
	): DiagnosticDescription {
		const advice = this.normalizeAdvice(description.advice);
		return maybeMerge(
			description,
			{
				message: this.normalizeMarkup(description.message),
				advice,
			},
		);
	}

	public normalizeDiagnostic(diag: Diagnostic): Diagnostic {
		// Fast path for a common case
		if (!this.hasNormalize()) {
			return diag;
		}

		let merge: Partial<Diagnostic> = {
			location: this.normalizeLocation(diag.location),
			description: this.normalizeDescription(diag.description),
		};

		if (diag.label !== undefined) {
			merge.label = this.normalizeMarkup(diag.label);
		}

		if (diag.dependencies !== undefined) {
			merge.dependencies = this.normalizeDependencies(diag.dependencies);
		}

		// Add on any specified tags
		if (this.options.tags) {
			if (diag.tags === undefined) {
				merge.tags = this.options.tags;
			} else {
				merge.tags = {
					...this.options.tags,
					...diag.tags,
				};
			}
		}

		// Add on any specified tags
		const {label} = this.options;
		if (label) {
			merge.label = diag.label ? markup`${label} (${diag.label})` : label;
		}

		return maybeMerge(diag, merge);
	}
}
