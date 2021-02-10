/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticAdviceItem,
	DiagnosticDependencies,
	DiagnosticIntegrity,
	DiagnosticLocation,
	DiagnosticTags,
} from "./types";
import {SourceMapConsumerCollection} from "@internal/codec-source-map";
import {
	MarkupFormatNormalizeOptions,
	StaticMarkup,
	markup,
	normalizeMarkup,
} from "@internal/markup";
import {ob1Number0, ob1Number1} from "@internal/ob1";
import {RequiredProps} from "@internal/typescript-helpers";
import {AnyPath, UnknownPathMap, UnknownPathSet} from "@internal/path";

type NormalizeOptionsRequiredPosition = RequiredProps<
	MarkupFormatNormalizeOptions,
	"normalizePosition"
>;

export type DiagnosticsNormalizerOptions = {
	getIntegrity?: (path: AnyPath) => undefined | DiagnosticIntegrity;
	tags?: DiagnosticTags;
	label?: StaticMarkup;
};

export default class DiagnosticsNormalizer {
	constructor(
		normalizeOptions?: DiagnosticsNormalizerOptions,
		markupOptions?: MarkupFormatNormalizeOptions,
		sourceMaps?: SourceMapConsumerCollection,
	) {
		this.sourceMaps = sourceMaps;
		this.inlineSourceText = new UnknownPathMap();
		this.hasMarkupOptions = markupOptions !== undefined;

		this.hasOptions = normalizeOptions !== undefined;
		this.options = normalizeOptions ?? {};

		this.inlinedSourceTextFilenames = new UnknownPathSet();

		this.markupOptions = this.createMarkupOptions(markupOptions);
	}

	private sourceMaps: undefined | SourceMapConsumerCollection;

	private options: DiagnosticsNormalizerOptions;
	private hasOptions: boolean;

	private markupOptions: NormalizeOptionsRequiredPosition;
	private hasMarkupOptions: boolean;

	private inlineSourceText: UnknownPathMap<string>;
	private inlinedSourceTextFilenames: UnknownPathSet;

	private createMarkupOptions(
		markupOptions: MarkupFormatNormalizeOptions = {},
	): NormalizeOptionsRequiredPosition {
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
						line ?? ob1Number1,
						column ?? ob1Number0,
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

	private normalizePath(path: AnyPath): AnyPath
	private normalizePath(path: undefined | AnyPath): undefined | AnyPath
	private normalizePath(path: undefined | AnyPath): undefined | AnyPath {
		const {markupOptions} = this;
		if (markupOptions === undefined || path === undefined) {
			return path;
		}

		const {normalizePosition} = markupOptions;
		if (normalizePosition === undefined) {
			return path;
		}

		return normalizePosition(path, undefined, undefined).path;
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
		const {getIntegrity} = this.options;
		if (sourceMaps === undefined && getIntegrity === undefined) {
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
					start = {
						...start,
						line: resolved.line,
						column: resolved.column,
					};
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
					end = {
						...end,
						line: resolved.line,
						column: resolved.column,
					};
				}
			}
		}

		const normalizedPath = this.normalizePath(path);

		// Inline sourceText. We keep track of filenames we've already inlined to avoid duplicating sourceText
		// During printing we'll fill it back in
		let {sourceText} = location;
		if (path !== undefined && !this.inlinedSourceTextFilenames.has(path)) {
			if (sourceText === undefined) {
				sourceText = this.inlineSourceText.get(path);
			}
			if (sourceText === undefined && normalizedPath !== undefined) {
				sourceText = this.inlineSourceText.get(normalizedPath);
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

		if (
			integrity === undefined &&
			getIntegrity !== undefined &&
			normalizedPath !== undefined
		) {
			integrity = getIntegrity(normalizedPath);
		}

		return {
			...location,
			integrity,
			sourceText,
			path: normalizedPath,
			marker: this.maybeNormalizeMarkup(marker),
			start: this.normalizePositionValue(start),
			end: this.normalizePositionValue(end),
		};
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
		return deps.map((dep) => {
			return {
				...dep,
				path: this.normalizePath(dep.path),
			};
		});
	}

	private normalizeDiagnosticAdviceItem(
		item: DiagnosticAdviceItem,
	): DiagnosticAdviceItem {
		const {sourceMaps} = this;

		switch (item.type) {
			case "frame":
				return {
					...item,
					location: this.normalizeLocation(item.location),
				};

			case "list":
				return {
					...item,
					list: item.list.map((markup) => this.normalizeMarkup(markup)),
				};

			case "log":
				return {
					...item,
					text: this.normalizeMarkup(item.text),
				};

			case "action":
				if (this.markupOptions.stripPositions) {
					return {
						...item,
						// Command flags could have position information
						commandFlags: {},
					};
				} else {
					return item;
				}

			case "stacktrace":
				return {
					...item,
					importantPaths: new UnknownPathSet(
						Array.from(
							item.importantPaths ?? [],
							(path) => this.normalizePath(path),
						),
					),
					frames: item.frames.map((frame) => {
						const {path, line, column} = frame;

						if (
							path === undefined ||
							line === undefined ||
							column === undefined ||
							(sourceMaps !== undefined && !sourceMaps.has(path))
						) {
							return {
								...frame,
								start: this.normalizePositionValue(line),
								column: this.normalizePositionValue(column),
								path: this.normalizePath(path),
							};
						}

						if (sourceMaps !== undefined) {
							const resolved = sourceMaps.approxOriginalPositionFor(
								path,
								line,
								column,
							);
							if (resolved !== undefined) {
								return {
									...frame,
									path: this.normalizePath(resolved.source),
									line: this.normalizePositionValue(resolved.line),
									column: this.normalizePositionValue(resolved.column),
								};
							}
						}

						return frame;
					}),
				};
		}

		return item;
	}

	public normalizeDiagnostic(diag: Diagnostic): Diagnostic {
		const {sourceMaps} = this;

		// Fast path for a common case
		if (
			!this.hasMarkupOptions &&
			(sourceMaps === undefined || !sourceMaps.hasAny()) &&
			this.inlineSourceText.size === 0 &&
			!this.hasOptions
		) {
			return diag;
		}

		const {description} = diag;

		const advice = description.advice.map((item) => {
			return this.normalizeDiagnosticAdviceItem(item);
		});

		let merge: Partial<Diagnostic> = {
			location: this.normalizeLocation(diag.location),
			description: {
				...description,
				message: this.normalizeMarkup(description.message),
				advice,
			},
		};

		if (diag.label !== undefined) {
			merge.label = this.normalizeMarkup(diag.label);
		}

		if (diag.dependencies !== undefined) {
			merge.dependencies = this.normalizeDependencies(diag.dependencies);
		}

		// Add on any specified tags
		if (this.options.tags) {
			merge.tags = {
				...this.options.tags,
				...diag.tags,
			};
		}

		// Add on any specified tags
		const {label} = this.options;
		if (label) {
			merge.label = diag.label ? markup`${label} (${diag.label})` : label;
		}

		diag = {
			...diag,
			...merge,
		};

		return diag;
	}
}
