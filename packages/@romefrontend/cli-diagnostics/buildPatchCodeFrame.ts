/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	CODE_FRAME_CONTEXT_LINES,
	CODE_FRAME_INDENT,
	GUTTER,
	MAX_PATCH_LINES,
} from "./constants";
import {joinNoBreak, showInvisibles} from "./utils";
import {
	Diffs,
	diffConstants,
	stringDiffUnified,
} from "@romefrontend/string-diff";
import {escapeMarkup, markup, markupTag} from "@romefrontend/string-markup";
import {DiagnosticAdviceDiff} from "@romefrontend/diagnostics";

function formatDiffLine(diffs: Diffs) {
	return diffs.map(([type, text], i) => {
		const escaped = escapeMarkup(text);
		if (type === diffConstants.EQUAL) {
			return escaped;
		} else {
			return markupTag("emphasis", showInvisibles(escaped, i === 0));
		}
	}).join("");
}

const DELETE_MARKER = markupTag("error", "-");
const ADD_MARKER = markupTag("success", "+");

function formatSingleLineMarker(text: string): string {
	return markup`<emphasis>${text}</emphasis>: `;
}

export default function buildPatchCodeFrame(
	item: DiagnosticAdviceDiff,
	truncate: boolean,
): {
	truncated: boolean;
	frame: string;
} {
	const {diffsByLine, beforeLineCount, afterLineCount} = stringDiffUnified(
		item.diff,
	);

	// Calculate the parts of the diff we should show
	const shownLineIndexes: Set<number> = new Set();
	for (let i = 0; i < diffsByLine.length; i++) {
		const {beforeLine, afterLine} = diffsByLine[i];

		if (beforeLine === undefined || afterLine === undefined) {
			for (
				let visible = i - CODE_FRAME_CONTEXT_LINES;
				visible < i + CODE_FRAME_CONTEXT_LINES;
				visible++
			) {
				shownLineIndexes.add(visible);
			}
		}
	}

	// Calculate width of line no column
	const beforeNoLength = String(beforeLineCount).length;
	const afterNoLength = String(afterLineCount).length;

	const singleLine = beforeLineCount === 1 && afterLineCount === 1;

	const {legend} = item;
	const frame = [];
	let displayedLines = 0;
	let truncated = false;
	let lastDisplayedLine = -1;

	// Add 1 for the space separator
	const lineNoLength = beforeNoLength + afterNoLength + 1;
	const skippedLine = `<emphasis>${CODE_FRAME_INDENT}${"\xb7".repeat(
		lineNoLength,
	)}${GUTTER}</emphasis>`;

	function createGutter(beforeLine?: number, afterLine?: number) {
		let gutter = `<emphasis>${CODE_FRAME_INDENT}<pad align="right" width="${beforeNoLength}">`;
		if (beforeLine !== undefined) {
			gutter += String(beforeLine);
		}
		gutter += `</pad> <pad align="right" width="${afterNoLength}">`;
		if (afterLine !== undefined) {
			gutter += String(afterLine);
		}
		gutter += `</pad>${GUTTER}</emphasis>`;
		return gutter;
	}

	// Build the actual frame
	for (let i = 0; i < diffsByLine.length; i++) {
		if (!shownLineIndexes.has(i)) {
			continue;
		}

		displayedLines++;

		if (!truncate && displayedLines > MAX_PATCH_LINES) {
			truncated = true;
			continue;
		}

		const {beforeLine, afterLine, diffs} = diffsByLine[i];

		let lineType: "EQUAL" | "ADD" | "DELETE" = "EQUAL";

		if (beforeLine === undefined) {
			lineType = "ADD";
		}

		if (afterLine === undefined) {
			lineType = "DELETE";
		}

		if (lastDisplayedLine !== i - 1 && lastDisplayedLine !== -1) {
			frame.push(skippedLine);
		}

		let gutter = "";

		if (singleLine) {
			if (legend !== undefined) {
				if (lineType === "DELETE") {
					gutter = formatSingleLineMarker(legend.delete);
				} else if (lineType === "ADD") {
					gutter = formatSingleLineMarker(legend.add);
				}
			}
		} else {
			gutter = createGutter(beforeLine, afterLine);
		}

		if (lineType === "DELETE") {
			frame.push(
				`${gutter}${DELETE_MARKER} <error>${formatDiffLine(diffs)}</error>`,
			);
		} else if (lineType === "ADD") {
			frame.push(
				`${gutter}${ADD_MARKER} <success>${formatDiffLine(diffs)}</success>`,
			);
		} else {
			frame.push(`${gutter}  ${formatDiffLine(diffs)}`);
		}

		lastDisplayedLine = i;
	}

	if (truncated) {
		frame.push(
			`${skippedLine} <dim><number>${displayedLines - MAX_PATCH_LINES}</number> more lines truncated</dim>`,
		);
	}

	if (legend !== undefined) {
		frame.push("");
		frame.push(`<error>- ${escapeMarkup(legend.delete)}</error>`);
		frame.push(`<success>+ ${escapeMarkup(legend.add)}</success>`);
		frame.push("");
	}

	return {
		truncated,
		frame: joinNoBreak(frame),
	};
}
