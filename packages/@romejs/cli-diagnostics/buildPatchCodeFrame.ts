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
} from './constants';
import {joinNoBreak, normalizeTabs, showInvisibles} from './utils';
import {Diffs, diffConstants, groupDiffByLines} from '@romejs/string-diff';
import {escapeMarkup, markup, markupTag} from '@romejs/string-markup';
import {DiagnosticAdviceDiff} from '@romejs/diagnostics';

function formatDiffLine(diffs: Diffs) {
	return diffs.map(([type, text]) => {
		if (type === diffConstants.DELETE) {
			return markupTag('error', escapeMarkup(showInvisibles(text)));
		} else if (type === diffConstants.ADD) {
			return markupTag('success', escapeMarkup(showInvisibles(text)));
		} else {
			// type === diffConstants.EQUAL
			return escapeMarkup(normalizeTabs(text));
		}
	}).join('');
}

const DELETE_MARKER = markupTag('error', '-');
const ADD_MARKER = markupTag('success', '+');

function formatSingleLineMarker(text: string): string {
	return markup`<emphasis>${text}</emphasis>:`;
}

export default function buildPatchCodeFrame(
	item: DiagnosticAdviceDiff,
	verbose: boolean,
): {
	truncated: boolean;
	frame: string;
} {
	const diffsByLine = groupDiffByLines(item.diff);
	let lastVisibleLine = -1;

	// Calculate the parts of the diff we should show
	const shownLines: Set<number> = new Set();
	for (let i = 0; i < diffsByLine.length; i++) {
		const diffs = diffsByLine[i];

		let hasChange = false;
		for (const [type] of diffs) {
			if (type === diffConstants.DELETE || type === diffConstants.ADD) {
				hasChange = true;
				break;
			}
		}

		if (hasChange) {
			for (
				let start = i - CODE_FRAME_CONTEXT_LINES;
				start < i + CODE_FRAME_CONTEXT_LINES;
				start++
			) {
				shownLines.add(start);

				if (start > lastVisibleLine) {
					lastVisibleLine = start;
				}
			}
		}
	}

	const lineLength = String(lastVisibleLine).length;

	// Don't output a gutter if there's only a single line
	const singleLine = diffsByLine.length === 1;

	const {legend} = item;
	const frame = [];
	let displayedLines = 0;
	let truncated = false;
	let lastDisplayedLine = -1;

	const skippedLine = `<emphasis>${CODE_FRAME_INDENT}${'.'.repeat(lineLength)}${GUTTER}</emphasis>`;

	// Build the actual frame
	for (let i = 0; i < diffsByLine.length; i++) {
		if (shownLines.has(i) === false) {
			continue;
		}

		displayedLines++;

		if (!verbose && displayedLines > MAX_PATCH_LINES) {
			truncated = true;
			continue;
		}

		const diffs = diffsByLine[i];
		const lineNo = i + 1;

		const deletions: Diffs = [];
		const addition: Diffs = [];

		let hasDeletions = false;
		let hasAddition = false;

		for (const tuple of diffs) {
			let [type] = tuple;

			if (type === diffConstants.DELETE) {
				hasDeletions = true;
				deletions.push(tuple);
			}

			if (type === diffConstants.ADD) {
				hasAddition = true;
				addition.push(tuple);
			}

			if (type === diffConstants.EQUAL) {
				addition.push(tuple);
				deletions.push(tuple);
			}
		}

		if (lastDisplayedLine !== lineNo - 1 && lastDisplayedLine !== -1) {
			frame.push(skippedLine);
		}

		let gutterWithLine = '';
		let gutterNoLine = '';
		let deleteMarker = DELETE_MARKER;
		let addMarker = ADD_MARKER;

		if (!singleLine) {
			gutterWithLine = `<emphasis>${CODE_FRAME_INDENT}<pad align="right" width="${lineLength}">${lineNo}</pad>${GUTTER}</emphasis>`;
			gutterNoLine = `<emphasis>${CODE_FRAME_INDENT}${' '.repeat(lineLength)}${GUTTER}</emphasis>`;
		}

		if (singleLine && legend !== undefined) {
			addMarker = formatSingleLineMarker(legend.add);
			deleteMarker = formatSingleLineMarker(legend.delete);
		}

		if (hasDeletions) {
			const gutter = hasAddition ? gutterNoLine : gutterWithLine;
			frame.push(`${gutter}${deleteMarker} ${formatDiffLine(deletions)}`);
		}

		if (hasAddition) {
			frame.push(`${gutterWithLine}${addMarker} ${formatDiffLine(addition)}`);
		}

		if (!hasAddition && !hasDeletions) {
			// Output one of the lines, they're the same
			frame.push(`${gutterWithLine}  ${formatDiffLine(addition)}`);
		}

		lastDisplayedLine = lineNo;
	}

	if (truncated) {
		frame.push(
			`${skippedLine} <dim><number>${displayedLines - MAX_PATCH_LINES}</number> more lines truncated</dim>`,
		);
	}

	if (legend !== undefined && !singleLine) {
		frame.push('');
		frame.push(`<error>- ${escapeMarkup(legend.delete)}</error>`);
		frame.push(`<success>+ ${escapeMarkup(legend.add)}</success>`);
		frame.push('');
	}

	return {
		truncated,
		frame: joinNoBreak(frame),
	};
}
