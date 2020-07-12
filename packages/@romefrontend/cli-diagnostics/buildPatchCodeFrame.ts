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
import {showInvisibles} from "./utils";
import {
	Diffs,
	diffConstants,
	stringDiffUnified,
} from "@romefrontend/string-diff";
import {escapeMarkup, markup, markupTag} from "@romefrontend/string-markup";
import {DiagnosticAdviceDiff} from "@romefrontend/diagnostics";

function formatDiffLine(diffs: Diffs) {
	let atLineStart = true;
	return diffs.map(([type, text], i) => {
		const escaped = escapeMarkup(text);
		const res = showInvisibles(
			escaped,
			{
				atLineStart,
				atLineEnd: i === diffs.length - 1,
			},
		);
		if (res.hadNonWhitespace) {
			atLineStart = false;
		}
		const value = res.value;
		if (type === diffConstants.EQUAL) {
			return escaped;
		} else {
			return markupTag("emphasis", value);
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

	function createLineNos(beforeLine?: number, afterLine?: number) {
		let gutter = `<emphasis>${CODE_FRAME_INDENT}<pad align="right" width="${beforeNoLength}">`;
		if (beforeLine !== undefined) {
			gutter += String(beforeLine);
		}
		gutter += `</pad> <pad align="right" width="${afterNoLength}">`;
		if (afterLine !== undefined) {
			gutter += String(afterLine);
		}
		gutter += "</pad></emphasis>";
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
		let lineNo = "";

		if (singleLine) {
			if (legend !== undefined) {
				if (lineType === "DELETE") {
					lineNo = formatSingleLineMarker(legend.delete);
				} else if (lineType === "ADD") {
					lineNo = formatSingleLineMarker(legend.add);
				}
			}
		} else {
			lineNo = createLineNos(beforeLine, afterLine);
			gutter = GUTTER;
		}

		if (lineType === "DELETE" || lineType === "ADD") {
			const marker = lineType === "ADD" ? ADD_MARKER : DELETE_MARKER;
			const type = lineType === "ADD" ? "success" : "error";
			frame.push(
				`${lineNo}<${type}><view linePrefix="${gutter}${marker} ">${formatDiffLine(
					diffs,
				)}</view></${type}>`,
			);
		} else {
			frame.push(
				`${lineNo}<view linePrefix="${gutter}  ">${formatDiffLine(diffs)}</view>`,
			);
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
		frame: frame.join("\n"),
	};
}
