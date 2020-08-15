/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticAdviceItem,
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

type NormalizeOptionsRequiredPosition = RequiredProps<
	MarkupFormatNormalizeOptions,
	"normalizePosition"
>;

export type DiagnosticsNormalizerOptions = {
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
		this.inlineSourceText = new Map();
		this.hasMarkupOptions = markupOptions !== undefined;

		this.hasOptions = normalizeOptions !== undefined;
		this.options = normalizeOptions ?? {};

		this.inlinedSourceTextFilenames = new Set();

		this.markupOptions = this.createMarkupOptions(markupOptions);
	}

	private sourceMaps: undefined | SourceMapConsumerCollection;

	private options: DiagnosticsNormalizerOptions;
	private hasOptions: boolean;

	private markupOptions: NormalizeOptionsRequiredPosition;
	private hasMarkupOptions: boolean;

	private inlineSourceText: Map<string, string>;
	private inlinedSourceTextFilenames: Set<string>;

	private createMarkupOptions(
		markupOptions: MarkupFormatNormalizeOptions = {},
	): NormalizeOptionsRequiredPosition {
		const {sourceMaps} = this;

		return {
			...markupOptions,
			normalizePosition: (filename, line, column) => {
				if (
					markupOptions !== undefined &&
					markupOptions.normalizePosition !== undefined
				) {
					({filename, line, column} = markupOptions.normalizePosition(
						filename,
						line,
						column,
					));
				}

				if (sourceMaps !== undefined) {
					// line and column can be undefined so we do some weirdness to try and get only the filename if possible
					// using some default positions and then we'll toss whatever positions they return
					const resolved = sourceMaps.approxOriginalPositionFor(
						filename,
						line ?? ob1Number1,
						column ?? ob1Number0,
					);
					if (resolved !== undefined) {
						return {
							filename: resolved.source,
							line: line === undefined ? undefined : resolved.line,
							column: column === undefined ? undefined : resolved.column,
						};
					}
				}

				return {filename, line, column};
			},
		};
	}

	public setInlineSourceText(filename: string, sourceText: string) {
		this.inlineSourceText.set(filename, sourceText);
	}

	private normalizeFilename(filename: string): string
	private normalizeFilename(filename: undefined | string): undefined | string
	private normalizeFilename(filename: undefined | string): undefined | string {
		const {markupOptions} = this;
		if (markupOptions === undefined || filename === undefined) {
			return filename;
		}

		const {normalizePosition} = markupOptions;
		if (normalizePosition === undefined) {
			return filename;
		}

		return normalizePosition(filename, undefined, undefined).filename;
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

		let {marker, filename, start, end} = location;
		let origFilename = filename;

		if (filename !== undefined && origFilename !== undefined) {
			if (start !== undefined) {
				const resolved = sourceMaps.approxOriginalPositionFor(
					origFilename,
					start.line,
					start.column,
				);
				if (resolved !== undefined) {
					filename = resolved.source;
					start = {
						...start,
						line: resolved.line,
						column: resolved.column,
					};
				}
			}

			if (end !== undefined) {
				const resolved = sourceMaps.approxOriginalPositionFor(
					origFilename,
					end.line,
					end.column,
				);
				if (resolved !== undefined) {
					// TODO confirm this is the same as `start` if it resolved
					filename = resolved.source;
					end = {
						...end,
						line: resolved.line,
						column: resolved.column,
					};
				}
			}
		}

		const normalizedFilename = this.normalizeFilename(filename);

		// Inline sourceText. We keep track of filenames we've already inlined to avoid duplicating sourceText
		// During printing we'll fill it back in
		let {sourceText} = location;
		if (filename !== undefined && !this.inlinedSourceTextFilenames.has(filename)) {
			if (sourceText === undefined) {
				sourceText = this.inlineSourceText.get(filename);
			}
			if (sourceText === undefined && normalizedFilename !== undefined) {
				sourceText = this.inlineSourceText.get(normalizedFilename);
			}

			// Remove sourceText if it's not pointing anywhere
			if (start === undefined && end === undefined) {
				sourceText = undefined;
			}

			// Register filename as inlined if necessary
			if (sourceText !== undefined) {
				this.inlinedSourceTextFilenames.add(filename);

				if (normalizedFilename !== undefined) {
					this.inlinedSourceTextFilenames.add(normalizedFilename);
				}
			}
		}

		return {
			...location,
			sourceText,
			filename: normalizedFilename,
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
					importantFilenames: (item.importantFilenames ?? []).map((filename) =>
						this.normalizeFilename(filename)
					),
					frames: item.frames.map((frame) => {
						const {filename, line, column} = frame;

						if (
							filename === undefined ||
							line === undefined ||
							column === undefined ||
							(sourceMaps !== undefined && !sourceMaps.has(filename))
						) {
							return {
								...frame,
								start: this.normalizePositionValue(line),
								column: this.normalizePositionValue(column),
								filename: this.normalizeFilename(filename),
							};
						}

						if (sourceMaps !== undefined) {
							const resolved = sourceMaps.approxOriginalPositionFor(
								filename,
								line,
								column,
							);
							if (resolved !== undefined) {
								return {
									...frame,
									filename: this.normalizeFilename(resolved.source),
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
			label: this.maybeNormalizeMarkup(diag.label),
			location: this.normalizeLocation(diag.location),
			description: {
				...description,
				message: this.normalizeMarkup(description.message),
				advice,
			},
		};

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
