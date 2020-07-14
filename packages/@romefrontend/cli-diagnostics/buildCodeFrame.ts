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
import {Position} from "@romefrontend/parser-core";
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
} from "@romefrontend/ob1";
import {markupTag, markupToPlainText} from "@romefrontend/cli-layout";
import {Dict} from "@romefrontend/typescript-helpers";
import {joinMarkupLines} from "@romefrontend/cli-layout/format";

function formatLineView(
	{marker, line, gutter}: FormattedLine,
	gutterLength: number,
): string {
	const attributes: Dict<string | number> = {
		extraSoftWrapIndent: 2,
		// NB: The `word-break` default is probably better? lineWrap: "char-break",
	};

	if (gutterLength > 0) {
		line += markupTag(
			"viewLinePrefix",
			`<pad align="right" width="${gutterLength}"><emphasis>${gutter}</emphasis></pad>${GUTTER}`,
			{
				type: "first",
			},
		);

		line += markupTag(
			"viewLinePrefix",
			`<dim>⇥</dim>${GUTTER}`,
			{
				align: "right",
				type: "middle",
			},
		);

		line += markupTag(
			"viewLinePrefix",
			GUTTER,
			{
				align: "right",
				type: "pointer",
			},
		);
	}

	if (marker !== undefined) {
		line += markupTag(
			"viewPointer",
			marker.message,
			{
				char: "<error><emphasis>^</emphasis></error>",
				line: "1",
				start: String(marker.start),
				end: String(marker.end),
			},
		);
	}

	return markupTag("view", line, attributes);
}

type Marker = {
	message: string;
	start: Number0;
	end: Number0;
};

type FormattedLine = {
	marker: undefined | Marker;
	gutter: string;
	line: string;
};

export default function buildCodeFrame(
	{
		sourceText,
		lines: allLines,
		truncateLines,
		start,
		end,
		type,
		markerMessage = "",
	}: {
		sourceText: string;
		lines: ToLines;
		type: "pointer" | "all";
		truncateLines?: number;
		start?: Position;
		end?: Position;
		markerMessage?: string;
	},
): {
	frame: string;
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
		let rawLine: undefined | string = allLines.raw[ob1Get0(i)];
		let highlightLine: undefined | string = allLines.highlighted[ob1Get0(i)];
		if (highlightLine === undefined || rawLine === undefined) {
			continue;
		}

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
					message: "",
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
			highlightLine,
			{
				atLineStart: true,
				atLineEnd: true,
			},
		).value;

		const lineNo = ob1Coerce0To1(i);
		let gutter = `${String(lineNo)}`;

		if (shouldHighlight) {
			gutter = `${CODE_FRAME_SELECTED_INDENT}${gutter}`;
		} else {
			gutter = `${CODE_FRAME_INDENT}${gutter}`;
		}

		formattedLines.push({
			marker,
			gutter,
			line: highlightLine,
		});

		maxVisibleLineNo = ob1Get0(i) + 1;

		if (truncateLines !== undefined && maxVisibleLineNo >= truncateLines) {
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
		if (info !== undefined && info.line === "") {
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
	if (markerMessage !== "" && start !== undefined && end !== undefined) {
		const text = sourceText.slice(ob1Get0(start.index), ob1Get0(end.index));
		if (
			cleanEquivalentString(text) ===
			cleanEquivalentString(joinMarkupLines(markupToPlainText(markerMessage)))
		) {
			markerMessage = "";
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

		const result = [`${CODE_FRAME_INDENT}${formatLineView(selection, 0)}`];

		return {
			frame: result.join("\n"),
			truncated,
		};
	}

	// Build up the line we display when source lines are omitted
	const omittedLine = `<emphasis><pad align="right" width="${maxGutterLength}">...</pad></emphasis>${GUTTER}`;

	// Build the frame
	const result = [];
	for (const selection of formattedLines) {
		if (!selection) {
			result.push(omittedLine);
			continue;
		}

		result.push(formatLineView(selection, maxGutterLength));
	}

	if (truncated) {
		result.push(
			`${omittedLine} <dim><number>${maxVisibleLineNo - truncateLines!}</number> more lines truncated</dim>`,
		);
	}

	return {
		truncated,
		frame: result.join("\n"),
	};
}
