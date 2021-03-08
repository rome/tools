/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticDependency,
	DiagnosticDescription,
	DiagnosticLocation,
	DiagnosticOrigin,
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
import {OneIndexed, ZeroIndexed} from "@internal/numbers";
import {mergeObjects} from "@internal/typescript-helpers";
import {MixedPathMap, MixedPathSet, Path} from "@internal/path";
import {DiagnosticsProcessorOptions} from "./DiagnosticsProcessor";

export type DiagnosticsNormalizerOptions = {
	defaultTags?: DiagnosticTags;
	defaultLabel?: StaticMarkup;
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
		{
			origin,
			normalizeOptions = {},
			sourceMaps,
			markupOptions,
		}: DiagnosticsProcessorOptions = {},
	) {
		this.sourceMaps = sourceMaps;
		this.inlineSourceText = new MixedPathMap();
		this.options = normalizeOptions;
		this.origins = origin === undefined ? [] : [origin];
		this.inlinedSourceTextPaths = new MixedPathSet();
		this.markupOptions = this.createMarkupOptions(markupOptions);
	}

	private sourceMaps: undefined | SourceMapConsumerCollection;

	private options: DiagnosticsNormalizerOptions;
	private markupOptions: MarkupFormatNormalizeOptions;

	private origins: DiagnosticOrigin[];
	private inlineSourceText: MixedPathMap<string>;
	private inlinedSourceTextPaths: MixedPathSet;

	public unshiftOrigin(origin: DiagnosticOrigin) {
		this.origins = [origin, ...this.origins];
	}

	private hasSourceMaps(): boolean {
		const {sourceMaps} = this;
		return sourceMaps !== undefined && sourceMaps.hasAny();
	}

	private couldNormalizeMarkup(): boolean {
		const {markupOptions} = this;

		return (
			this.hasSourceMaps() ||
			markupOptions.stripFilelinkText ||
			markupOptions.stripPositions ||
			markupOptions.humanizeFilename !== undefined ||
			markupOptions.normalizePosition !== undefined
		);
	}

	private couldNormalizeLocation(): boolean {
		return (
			this.inlinedSourceTextPaths.size > 0 ||
			this.hasSourceMaps() ||
			this.couldNormalizePath()
		);
	}

	private couldNormalizePath(): boolean {
		return this.markupOptions.normalizePosition !== undefined;
	}

	private couldNormalizeDiagnostic(): boolean {
		const {options} = this;
		return (
			this.origins.length > 0 ||
			this.couldNormalizeMarkup() ||
			this.couldNormalizeLocation() ||
			options.defaultLabel !== undefined ||
			options.defaultTags !== undefined
		);
	}

	public removePath(path: Path) {
		this.inlineSourceText.delete(path);
		this.inlinedSourceTextPaths.delete(path);
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

	public setInlineSourceText(path: Path, sourceText: string) {
		this.inlineSourceText.set(path, sourceText);
	}

	private normalizePath(path: Path): Path;
	private normalizePath(path: undefined | Path): undefined | Path;
	private normalizePath(path: undefined | Path): undefined | Path {
		if (path === undefined) {
			return path;
		}

		const {normalizePosition} = this.markupOptions;
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
		if (!this.couldNormalizeLocation()) {
			return location;
		}

		const {sourceMaps} = this;

		let {marker, path, start, end, integrity} = location;
		let origPath = path;

		if (sourceMaps !== undefined && sourceMaps.hasAny()) {
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
		if (!this.inlinedSourceTextPaths.has(path)) {
			sourceText =
				sourceText ??
				this.inlineSourceText.get(path) ??
				this.inlineSourceText.get(normalizedPath);

			if (
				location.sourceText !== undefined &&
				location.sourceText !== sourceText
			) {
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
				this.inlinedSourceTextPaths.add(normalizedPath);
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
		if (this.couldNormalizeMarkup()) {
			return normalizeMarkup(markup, this.markupOptions).text;
		} else {
			return markup;
		}
	}

	private maybeNormalizeMarkup(
		markup: undefined | StaticMarkup,
	): undefined | StaticMarkup {
		return markup === undefined ? undefined : this.normalizeMarkup(markup);
	}

	private normalizeDependencies(
		deps: DiagnosticDependency[],
	): DiagnosticDependency[] {
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

	private normalizeAdvice(advice: DiagnosticAdvice[]): DiagnosticAdvice[] {
		const newAdvice: DiagnosticAdvice[] = [];
		let normalized = false;

		for (const item of advice) {
			const newItem = this.normalizeAdviceItem(item);
			if (newItem === undefined) {
				normalized = true;
				continue;
			}
			if (newItem !== item) {
				normalized = true;
			}
			newAdvice.push(newItem);
		}

		return normalized ? newAdvice : advice;
	}

	private normalizeAdviceItem(
		item: DiagnosticAdvice,
	): undefined | DiagnosticAdvice {
		const {sourceMaps} = this;

		switch (item.type) {
			case "group":
				return maybeMerge(
					item,
					{
						title: this.normalizeMarkup(item.title),
						advice: this.normalizeAdvice(item.advice),
					},
				);

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
						commandFlags: undefined,
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
		if (this.couldNormalizePath()) {
			return maybeMerge(
				suppression,
				{
					path: this.normalizePath(suppression.path),
				},
			);
		} else {
			return suppression;
		}
	}

	private normalizeDescription(
		description: DiagnosticDescription,
	): DiagnosticDescription {
		const advice = this.normalizeAdvice(description.advice);
		const verboseAdvice =
			description.verboseAdvice === undefined
				? undefined
				: this.normalizeAdvice(description.verboseAdvice);
		return maybeMerge(
			description,
			{
				message: this.normalizeMarkup(description.message),
				advice,
				verboseAdvice,
			},
		);
	}

	public normalizeDiagnostic(diag: Diagnostic): Diagnostic {
		if (!this.couldNormalizeDiagnostic()) {
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

		if (this.origins.length > 0) {
			if (diag.origins === undefined) {
				merge.origins = this.origins;
			} else {
				const origins = [...diag.origins];
				const entities: Set<string> = new Set(
					origins.map((origin) => origin.entity),
				);
				let modified = false;
				for (const origin of this.origins) {
					if (!entities.has(origin.entity)) {
						entities.add(origin.entity);
						origins.push(origin);
					}
				}
				if (modified) {
					merge.origins = origins;
				}
			}
		}

		const {defaultTags, defaultLabel} = this.options;
		if (defaultTags !== undefined) {
			if (diag.tags === undefined) {
				merge.tags = defaultTags;
			} else {
				merge.tags = {
					...defaultTags,
					...diag.tags,
				};
			}
		}

		if (defaultLabel !== undefined) {
			merge.label = diag.label
				? markup`${defaultLabel} (${diag.label})`
				: defaultLabel;
		}

		return maybeMerge(diag, merge);
	}
}
