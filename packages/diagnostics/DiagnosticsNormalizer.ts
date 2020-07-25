/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostic, DiagnosticAdviceItem, DiagnosticLocation} from "./types";
import {SourceMapConsumerCollection} from "@romefrontend/codec-source-map";
import {
	Markup,
	MarkupFormatNormalizeOptions,
	normalizeMarkup,
} from "@romefrontend/cli-layout";
import {ob1Number0, ob1Number0Neg1, ob1Number1} from "@romefrontend/ob1";
import {RequiredProps} from "@romefrontend/typescript-helpers";
import {Position} from "@romefrontend/parser-core";

type NormalizeOptionsRequiredPosition = RequiredProps<
	MarkupFormatNormalizeOptions,
	"normalizePosition"
>;

export default class DiagnosticsNormalizer {
	constructor(
		markupOptions?: MarkupFormatNormalizeOptions,
		sourceMaps?: SourceMapConsumerCollection,
	) {
		this.sourceMaps = sourceMaps;
		this.inlineSourceText = new Map();
		this.hasMarkupOptions = markupOptions !== undefined;

		this.inlinedSourceTextFilenames = new Set();

		this.markupOptions = this.createMarkupOptions(markupOptions);
	}

	sourceMaps: undefined | SourceMapConsumerCollection;
	markupOptions: NormalizeOptionsRequiredPosition;
	hasMarkupOptions: boolean;
	inlineSourceText: Map<string, string>;
	inlinedSourceTextFilenames: Set<string>;

	createMarkupOptions(
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
						line === undefined ? ob1Number1 : line,
						column === undefined ? ob1Number0 : column,
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

	setInlineSourceText(filename: string, sourceText: string) {
		this.inlineSourceText.set(filename, sourceText);
	}

	normalizeFilename(filename: undefined | string): undefined | string {
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

	normalizePositionValue<T>(value: T): undefined | T {
		if (this.markupOptions !== undefined && this.markupOptions.stripPositions) {
			return undefined;
		} else {
			return value;
		}
	}

	resolvePosition(
		filename: string,
		position: Position,
	):
		| undefined
		| {
				filename: string;
				position: Position;
			} {
		const resolved = this.markupOptions.normalizePosition(
			filename,
			position.line,
			position.column,
		);
		if (resolved === undefined) {
			return undefined;
		}

		return {
			filename: resolved.filename,
			position: {
				line: resolved.line === undefined ? ob1Number1 : resolved.line,
				column: resolved.column === undefined ? ob1Number0 : resolved.column,
				index: ob1Number0Neg1,
			},
		};
	}

	normalizeLocation(location: DiagnosticLocation): DiagnosticLocation {
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
			if (sourceText === undefined && filename !== undefined) {
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
				if (filename !== undefined) {
					this.inlinedSourceTextFilenames.add(filename);
				}

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

	normalizeMarkup(markup: Markup): Markup {
		return normalizeMarkup(markup, this.markupOptions).text;
	}

	maybeNormalizeMarkup(markup: undefined | Markup): undefined | Markup {
		return markup === undefined ? undefined : this.normalizeMarkup(markup);
	}

	normalizeDiagnosticAdviceItem(
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

	normalizeDiagnostic(diag: Diagnostic): Diagnostic {
		const {sourceMaps} = this;

		// Fast path for a common case
		if (
			!this.hasMarkupOptions &&
			(sourceMaps === undefined || !sourceMaps.hasAny()) &&
			this.inlineSourceText.size === 0
		) {
			return diag;
		}

		const {description} = diag;

		const advice = description.advice.map((item) => {
			return this.normalizeDiagnosticAdviceItem(item);
		});

		diag = {
			...diag,
			label: this.maybeNormalizeMarkup(diag.label),
			location: this.normalizeLocation(diag.location),
			description: {
				...description,
				message: this.normalizeMarkup(description.message),
				advice,
			},
		};

		return diag;
	}
}
