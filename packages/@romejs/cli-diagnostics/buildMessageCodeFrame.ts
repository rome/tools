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
import {Position} from "@romejs/parser-core";
import {
	ToLines,
	cleanEquivalentString,
	joinNoBreak,
	normalizeTabs,
} from "./utils";
import {
	Number0,
	ob1Coerce0,
	ob1Coerce0To1,
	ob1Coerce1To0,
	ob1Get0,
	ob1Inc,
	ob1Number0,
	ob1Number1Neg1,
	ob1Sub,
} from "@romejs/ob1";
import {markupToPlainTextString} from "@romejs/string-markup";

function createPointer(
	markerMessage: string,
	line: string,
	markerStart: Number0,
	markerEnd: Number0,
): undefined | string {
	let result = "";

	let markerSize = ob1Get0(ob1Sub(markerEnd, markerStart));

	// If the range contains tabs then increase the marker size
	for (let i = ob1Get0(markerStart); i < ob1Get0(markerEnd); i++) {
		const char = line[i];
		if (char === "\t") {
			markerSize++;
		}
	}

	const pointerLength: number = Math.max(markerSize, 1);

	// Skip the pointer if it's pointing at the last character
	let skipPointer = pointerLength === 1 && ob1Get0(markerEnd) >= line.length;

	if (!skipPointer) {
		// Add indentation, handling hard tabs as two soft spaces
		for (let i = 0; i < ob1Get0(markerStart); i++) {
			const char = line[i];
			if (char === "\t") {
				// normalizeTabs will be called on this line and this replacement made
				result += "  ";
			} else {
				result += " ";
			}
		}

		// Add pointer
		result += `<error><emphasis>${"^".repeat(pointerLength)}</emphasis></error>`;
	}

	// Add marker
	if (markerMessage !== "") {
		result += ` ${markerMessage}`;
	}

	if (result === "") {
		return undefined;
	} else {
		return result;
	}
}

export default function buildMessageCodeFrame(
	sourceText: string,
	allLines: ToLines,
	start: undefined | Position,
	end: undefined | Position,
	markerMessage: string,
): string {
	if (allLines.length === 0 || start === undefined || end === undefined) {
		if (markerMessage === "") {
			return "";
		} else {
			return `<nobr>${markerMessage}</nobr>`;
		}
	}

	const startLineIndex = ob1Coerce1To0(start.line);
	let endLineIndex = ob1Coerce1To0(end.line);

	// Increase the amount of lines we should show for "context"
	let contextStartIndex = ob1Coerce0(
		Math.max(0, ob1Get0(startLineIndex) - CODE_FRAME_CONTEXT_LINES),
	);
	let contextEndIndex = ob1Coerce0(
		Math.min(
			allLines.length - 1,
			ob1Get0(endLineIndex) + CODE_FRAME_CONTEXT_LINES,
		),
	);

	let maxVisibleLineNo = 0;

	let formattedLines: Array<
		| {
				pointer: undefined | string;
				gutter: string;
				line: string;
			}
		| undefined
	> = [];
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

		let pointer: undefined | string;

		// If this is within the highlighted line range
		const shouldHighlight: boolean = i >= startLineIndex && i <= endLineIndex;

		if (shouldHighlight) {
			if (i === startLineIndex && i === endLineIndex) {
				// Only line in the selection
				pointer = createPointer(markerMessage, rawLine, start.column, end.column);
			} else if (i === startLineIndex) {
				// First line in selection
				pointer = createPointer(
					"",
					rawLine,
					start.column,
					// line could be highlighted
					ob1Coerce0(rawLine.length),
				);
			} else if (i === endLineIndex) {
				// Last line in selection
				pointer = createPointer(markerMessage, rawLine, ob1Number0, end.column);
			}
		}

		// Replace hard tabs with two spaces
		highlightLine = normalizeTabs(highlightLine);

		const lineNo = ob1Coerce0To1(i);
		let gutter = `${String(lineNo)}${GUTTER}`;

		if (shouldHighlight) {
			gutter = `${CODE_FRAME_SELECTED_INDENT}${gutter}`;
		} else {
			gutter = `${CODE_FRAME_INDENT}${gutter}`;
		}

		formattedLines.push({
			pointer,
			gutter,
			line: highlightLine,
		});

		maxVisibleLineNo = ob1Get0(i) + 1;
	}

	// If we have too many lines in our selection, then collapse them to an ellipsis
	const pruned = formattedLines.length > MAX_CODE_FRAME_LINES + 2;
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
	if (
		formattedLines.length === 0 ||
		end.line === ob1Number1Neg1 ||
		start.line === ob1Number1Neg1
	) {
		if (markerMessage === "") {
			return "";
		} else {
			return `<nobr>${markerMessage}</nobr>`;
		}
	}

	// Calculate max size of gutter, this is the maximum visible line plus the futter length plus the frame indent
	const lastLine = formattedLines[formattedLines.length - 1];
	if (lastLine === undefined) {
		throw new Error("Expected there to be a last line");
	}

	const maxGutterLength =
		String(maxVisibleLineNo).length + GUTTER.length + CODE_FRAME_INDENT.length;

	// If what the marker is highlighting equals the marker message then it's redundant so don't show the message
	if (markerMessage !== "") {
		const text = sourceText.slice(ob1Get0(start.index), ob1Get0(end.index));
		if (
			cleanEquivalentString(text) ===
			cleanEquivalentString(markupToPlainTextString(markerMessage))
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

		const result = [`${CODE_FRAME_INDENT}${selection.line}`];
		if (selection.pointer !== undefined) {
			result.push(`${CODE_FRAME_INDENT}${selection.pointer}`);
		}

		return joinNoBreak(result);
	}

	// Build up the line we display when source lines are omitted
	const omittedLine =
		`<emphasis><pad align="right" width="${maxGutterLength}">...</pad></emphasis>` +
		GUTTER;

	// Build the frame
	const result = [];
	for (const selection of formattedLines) {
		if (!selection) {
			result.push(omittedLine);
			continue;
		}

		const {pointer, gutter, line} = selection;

		result.push(
			`<pad align="right" width="${maxGutterLength}"><emphasis>${gutter}</emphasis></pad>` +
			line,
		);

		if (pointer !== undefined) {
			result.push(
				`<pad align="right" width="${maxGutterLength}"><emphasis>${GUTTER}</emphasis></pad>${pointer}`,
			);
		}
	}

	return joinNoBreak(result);
}
