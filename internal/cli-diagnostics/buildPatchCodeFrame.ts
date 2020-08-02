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
import {Diffs, diffConstants, stringDiffUnified} from "@internal/string-diff";
import {
	AnyMarkups,
	StaticMarkup,
	concatMarkup,
	markup,
	markupTag,
	readMarkup,
} from "@internal/markup";
import {DiagnosticAdviceDiff} from "@internal/diagnostics";

function formatDiffLine(diffs: Diffs) {
	let atLineStart = true;

	return concatMarkup(
		diffs.map(([type, text], i) => {
			const escapedText = markup`${text}`;

			const {hadNonWhitespace, value} = showInvisibles(
				readMarkup(escapedText),
				{
					ignoreLeadingTabs: false,
					ignoreLoneSpaces: false,
					atLineStart,
					atLineEnd: i === diffs.length - 1,
				},
			);
			if (hadNonWhitespace) {
				atLineStart = false;
			}

			if (type === diffConstants.EQUAL) {
				return value;
			} else {
				return markupTag("emphasis", value);
			}
		}),
	);
}

const DELETE_MARKER = markupTag("error", markup`-`);
const ADD_MARKER = markupTag("success", markup`+`);

function formatSingleLineMarker(text: string): StaticMarkup {
	return markup`<emphasis>${text}</emphasis>: `;
}

export default function buildPatchCodeFrame(
	item: DiagnosticAdviceDiff,
	truncate: boolean,
): {
	truncated: boolean;
	frame: StaticMarkup;
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
	const frame: AnyMarkups = [];
	let displayedLines = 0;
	let truncated = false;
	let lastDisplayedLine = -1;

	const lineNoLength = beforeNoLength + afterNoLength + 1;
	const skippedLine = markup`<emphasis>${CODE_FRAME_INDENT}${"\xb7".repeat(
		lineNoLength,
	)}${GUTTER}</emphasis>`;

	function createLineNos(
		beforeLine?: string | number,
		afterLine?: string | number,
	): StaticMarkup {
		let parts: AnyMarkups = [];
		parts.push(
			markup`<emphasis>${CODE_FRAME_INDENT}<pad align="right" width="${String(
				beforeNoLength,
			)}">`,
		);
		if (beforeLine !== undefined) {
			parts.push(markup`${String(beforeLine)}`);
		}
		parts.push(
			markup`</pad> <pad align="right" width="${String(afterNoLength)}">`,
		);
		if (afterLine !== undefined) {
			parts.push(markup`${String(afterLine)}`);
		}
		parts.push(markup`</pad></emphasis>`);
		return concatMarkup(parts);
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
		let marker = markup` `;

		if (beforeLine === undefined) {
			marker = ADD_MARKER;
			lineType = "ADD";
		}

		if (afterLine === undefined) {
			marker = DELETE_MARKER;
			lineType = "DELETE";
		}

		if (lastDisplayedLine !== i - 1 && lastDisplayedLine !== -1) {
			frame.push(skippedLine);
		}

		const logType = lineType === "ADD" ? "success" : "error";

		if (singleLine) {
			let legendPrefix = markup``;

			if (legend !== undefined) {
				if (lineType === "DELETE") {
					legendPrefix = formatSingleLineMarker(legend.delete);
				} else if (lineType === "ADD") {
					legendPrefix = formatSingleLineMarker(legend.add);
				}
			}

			if (lineType === "DELETE" || lineType === "ADD") {
				frame.push(
					markup`${legendPrefix}<view><viewLinePrefix>${marker} </viewLinePrefix><${logType}>${formatDiffLine(
						diffs,
					)}</${logType}></view>`,
				);
			} else {
				frame.push(
					markup`${legendPrefix}<view extraSoftWrapIndent="2"><viewLinePrefix>  </viewLinePrefix>${formatDiffLine(
						diffs,
					)}</view>`,
				);
			}
		} else {
			let prefixes = concatMarkup([
				markup`<viewLinePrefix type="first">${createLineNos(
					beforeLine,
					afterLine,
				)}${GUTTER}${marker} </viewLinePrefix>`,
				markup`<viewLinePrefix type="middle"><dim>${createLineNos(
					beforeLine === undefined ? undefined : "\u2192",
					afterLine === undefined ? undefined : "\u2192",
				)}</dim>${GUTTER}${marker} </viewLinePrefix>`,
			]);
			let line = formatDiffLine(diffs);

			if (lineType === "DELETE" || lineType === "ADD") {
				line = markup`<${logType}>${line}</${logType}>`;
			}

			frame.push(
				markup`<view extraSoftWrapIndent="2">${prefixes}${line}</view>`,
			);
		}

		lastDisplayedLine = i;
	}

	if (truncated) {
		frame.push(
			markup`${skippedLine} <dim><number>${String(
				displayedLines - MAX_PATCH_LINES,
			)}</number> more lines truncated</dim>`,
		);
	}

	if (legend !== undefined && !singleLine) {
		frame.push(markup``);
		frame.push(markup`<error>- ${legend.delete}</error>`);
		frame.push(markup`<success>+ ${legend.add}</success>`);
		frame.push(markup``);
	}

	return {
		truncated,
		frame: concatMarkup(frame, markup`\n`),
	};
}
