/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	CODE_FRAME_CONTEXT_LINES,
	CODE_FRAME_INDENT,
	CODE_FRAME_SELECTED_INDENT,
	GUTTER,
	HALF_MAX_CODE_FRAME_LINES,
	MAX_CODE_FRAME_LINES,
} from "./constants";
import {Position} from "@internal/parser-core";
import {ToLines, cleanEquivalentString, showInvisibles} from "./utils";
import {
	Number0,
	ob1Coerce0,
	ob1Coerce0To1,
	ob1Coerce1To0,
	ob1Get0,
	ob1Inc,
	ob1Number0,
	ob1Number1Neg1,
} from "@internal/ob1";
import {
	AnyMarkups,
	StaticMarkup,
	concatMarkup,
	isEmptyMarkup,
	markup,
	markupTag,
	readMarkup,
} from "@internal/markup";
import {Dict} from "@internal/typescript-helpers";

function formatLineView(
	{marker, line, gutter}: FormattedLine,
	gutterLength: number,
): StaticMarkup {
	const attributes: Dict<string | number> = {
		extraSoftWrapIndent: 2,
		// NB: The `word-break` default is probably better? lineWrap: "char-break",
	};

	const parts: AnyMarkups = [line];

	if (gutterLength > 0) {
		parts.push(
			markupTag(
				"viewLinePrefix",
				markup`<pad align="right" width="${String(gutterLength)}"><emphasis>${gutter}</emphasis></pad>${GUTTER}`,
				{
					type: "first",
				},
			),
		);

		parts.push(
			markupTag(
				"viewLinePrefix",
				markup`<dim>→</dim>${GUTTER}`,
				{
					align: "right",
					type: "middle",
				},
			),
		);

		parts.push(
			markupTag(
				"viewLinePrefix",
				GUTTER,
				{
					align: "right",
					type: "pointer",
				},
			),
		);
	}

	if (marker !== undefined) {
		parts.push(
			markupTag(
				"viewPointer",
				marker.message,
				{
					char: "<error><emphasis>^</emphasis></error>",
					line: "1",
					start: String(marker.start),
					end: String(marker.end),
				},
			),
		);
	}

	return markupTag("view", concatMarkup(parts), attributes);
}

type Marker = {
	message: StaticMarkup;
	start: Number0;
	end: Number0;
};

type FormattedLine = {
	marker: undefined | Marker;
	gutter: StaticMarkup;
	line: StaticMarkup;
};

export default function buildCodeFrame(
	{
		lines: allLines,
		truncateLines,
		start,
		end,
		type,
		markerMessage = markup``,
	}: {
		lines: ToLines;
		type: "pointer" | "all";
		truncateLines?: number;
		start?: Position;
		end?: Position;
		markerMessage?: StaticMarkup;
	},
): {
	frame: StaticMarkup;
	truncated: boolean;
} {
	// Bail if we have negative line references, we have no lines, or we expected positions and don't have one
	let shouldBail = allLines.length === 0;
	if (type === "pointer" && (start === undefined || end === undefined)) {
		shouldBail = true;
	}
	if (start !== undefined && start.line === ob1Number1Neg1) {
		shouldBail = true;
	}
	if (end !== undefined && end.line === ob1Number1Neg1) {
		shouldBail = true;
	}
	if (shouldBail) {
		return {
			frame: markerMessage,
			truncated: false,
		};
	}

	// Whether we truncated lines
	let truncated = false;

	const startLineIndex =
		start === undefined ? ob1Number0 : ob1Coerce1To0(start.line);
	let endLineIndex =
		end === undefined
			? ob1Coerce0(allLines.length - 1)
			: ob1Coerce1To0(end.line);

	// Increase the amount of lines we should show for "context"
	let contextStartIndex =
		start === undefined
			? startLineIndex
			: ob1Coerce0(
					Math.max(0, ob1Get0(startLineIndex) - CODE_FRAME_CONTEXT_LINES),
				);
	let contextEndIndex =
		end === undefined
			? endLineIndex
			: ob1Coerce0(
					Math.min(
						allLines.length - 1,
						ob1Get0(endLineIndex) + CODE_FRAME_CONTEXT_LINES,
					),
				);

	let maxVisibleLineNo = 0;

	let formattedLines: Array<FormattedLine | undefined> = [];
	for (let i = contextStartIndex; i <= contextEndIndex; i = ob1Inc(i)) {
		let line = allLines[ob1Get0(i)];
		if (line === undefined) {
			continue;
		}

		let [rawLine, highlightLine] = line;

		// Ensure that the frame doesn't start with whitespace
		if (
			rawLine.trim() === "" &&
			formattedLines.length === 0 &&
			i !== startLineIndex
		) {
			continue;
		}

		// If this is within the highlighted line range
		const shouldHighlight: boolean =
			type === "pointer" && i >= startLineIndex && i <= endLineIndex;

		let marker: undefined | Marker;

		if (shouldHighlight && start !== undefined && end !== undefined) {
			if (i === startLineIndex && i === endLineIndex) {
				// Only line in the selection
				marker = {
					message: markerMessage,
					start: start.column,
					end: end.column,
				};
			} else if (i === startLineIndex) {
				// First line in selection
				marker = {
					message: markup``,
					start: start.column,
					// line could be highlighted
					end: ob1Coerce0(rawLine.length),
				};
			} else if (i === endLineIndex) {
				// Last line in selection
				marker = {
					message: markerMessage,
					start: ob1Number0,
					end: end.column,
				};
			}
		}

		// Show invisible characters
		highlightLine = showInvisibles(
			readMarkup(highlightLine),
			{
				ignoreLeadingTabs: true,
				ignoreLoneSpaces: true,
				atLineStart: true,
				atLineEnd: true,
			},
		).value;

		const lineNo = ob1Coerce0To1(i);
		let gutter = markup`${String(lineNo)}`;

		if (shouldHighlight) {
			gutter = markup`${CODE_FRAME_SELECTED_INDENT}${gutter}`;
		} else {
			gutter = markup`${CODE_FRAME_INDENT}${gutter}`;
		}

		formattedLines.push({
			marker,
			gutter,
			line: highlightLine,
		});

		maxVisibleLineNo = ob1Get0(i) + 1;

		if (truncateLines !== undefined && maxVisibleLineNo > truncateLines) {
			truncated = true;
			break;
		}
	}

	// If we have too many lines in our selection, then collapse them to an ellipsis
	const pruned =
		type === "pointer" && formattedLines.length > MAX_CODE_FRAME_LINES + 2;
	if (pruned) {
		const start = formattedLines.slice(0, HALF_MAX_CODE_FRAME_LINES);
		const end = formattedLines.slice(-HALF_MAX_CODE_FRAME_LINES);
		formattedLines = start.concat([undefined], end);
	}

	// Remove trailing blank lines
	for (let i = formattedLines.length - 1; i >= 0; i--) {
		const info = formattedLines[i];
		if (info !== undefined && isEmptyMarkup(info.line)) {
			formattedLines.pop();
		} else {
			break;
		}
	}

	// If there's no lines to target then return the normal marker
	if (formattedLines.length === 0) {
		return {
			frame: markerMessage,
			truncated: false,
		};
	}

	// Calculate max size of gutter, this is the maximum visible line plus the futter length plus the frame indent
	const lastLine = formattedLines[formattedLines.length - 1];
	if (lastLine === undefined) {
		throw new Error("Expected there to be a last line");
	}

	const maxGutterLength =
		String(maxVisibleLineNo).length + CODE_FRAME_INDENT.length;

	// If what the marker is highlighting equals the marker message then it's redundant so don't show the message
	if (!isEmptyMarkup(markerMessage) && start !== undefined && end !== undefined) {
		const line = allLines[ob1Get0(ob1Coerce1To0(start.line))];
		if (line !== undefined) {
			const text = line[0].slice(ob1Get0(start.column), ob1Get0(end.column));
			if (cleanEquivalentString(text) === cleanEquivalentString(markerMessage)) {
				for (const selection of formattedLines) {
					if (
						selection !== undefined &&
						selection.marker !== undefined &&
						selection.marker.message === markerMessage
					) {
						selection.marker.message = markup``;
					}
				}
			}
		}
	}

	// Output no gutter with a soft indent if this is true
	if (formattedLines.length === 1) {
		const selection = formattedLines[0];
		if (selection === undefined) {
			throw new Error(
				"Expected a selection? undefined is only valid here as an omitted line signifier",
			);
		}

		return {
			frame: markup`${CODE_FRAME_INDENT}${formatLineView(selection, 0)}`,
			truncated,
		};
	}

	// Build up the line we display when source lines are omitted
	const omittedLine = markup`<emphasis><pad align="right" width="${String(
		maxGutterLength,
	)}">...</pad></emphasis>${GUTTER}`;

	// Build the frame
	const result: AnyMarkups = [];
	for (const selection of formattedLines) {
		if (!selection) {
			result.push(omittedLine);
			continue;
		}

		result.push(formatLineView(selection, maxGutterLength));
	}

	if (truncated) {
		result.push(
			markup`${omittedLine} <dim><number>${String(
				maxVisibleLineNo - truncateLines!,
			)}</number> more lines truncated</dim>`,
		);
	}

	return {
		truncated,
		frame: concatMarkup(result, markup`\n`),
	};
}
