/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/**
 * This library modifies the diff-patch-match library by Neil Fraser
 * by removing the patch and match functionality and certain advanced
 * options in the diff function. The original license is as follows:
 *
 * ===
 *
 * Diff Match and Patch
 *
 * Copyright 2006 Google Inc.
 * http://code.google.com/p/google-diff-match-patch/
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

export type Diff = [-1 | 0 | 1, string];

export type Diffs = Array<Diff>;

type HalfMatch = undefined | [string, string, string, string, string];

/**
 * The data structure representing a diff is an array of tuples:
 * [[DIFF_DELETE, 'Hello'], [DIFF_INSERT, 'Goodbye'], [DIFF_EQUAL, ' world.']]
 * which means: delete 'Hello', add 'Goodbye' and keep ' world.'
 */
const DIFF_DELETE: -1 = -1;
const DIFF_INSERT: 1 = 1;
const DIFF_EQUAL: 0 = 0;
export const diffConstants = {
	DELETE: DIFF_DELETE,
	EQUAL: DIFF_EQUAL,
	ADD: DIFF_INSERT,
};

export type UnifiedDiff = {
	diffsByLine: Array<GroupDiffsLine>;
	beforeLineCount: number;
	afterLineCount: number;
};

export type GroupDiffsLine = {
	beforeLine?: number;
	afterLine?: number;
	diffs: Diffs;
};

function generateLineKey(beforeLine?: number, afterLine?: number) {
	return `${beforeLine || ""}:${afterLine || ""}`;
}

export function stringDiffUnified(rawDiffs: Diffs): UnifiedDiff {
	const modifiedLines: Set<string> = new Set();
	const insertedLines: Map<string, GroupDiffsLine> = new Map();
	const beforeLineToAfter: Map<number, number> = new Map();

	let beforeLine = 1;
	let afterLine = 1;

	function hasModifiedLine(beforeLine?: number, afterLine?: number): boolean {
		return modifiedLines.has(generateLineKey(beforeLine, afterLine));
	}

	function maybeGetLine(
		beforeLine?: number,
		afterLine?: number,
	): undefined | GroupDiffsLine {
		return insertedLines.get(generateLineKey(beforeLine, afterLine));
	}

	function getLine(beforeLine?: number, afterLine?: number): GroupDiffsLine {
		const key = generateLineKey(beforeLine, afterLine);

		const existing = insertedLines.get(key);
		if (existing !== undefined) {
			return existing;
		}

		const line: GroupDiffsLine = {
			beforeLine,
			afterLine,
			diffs: [],
		};
		insertedLines.set(key, line);
		return line;
	}

	function pushToLine(diff: Diff) {
		switch (diff[0]) {
			case diffConstants.ADD: {
				getLine(undefined, afterLine).diffs.push(diff);
				modifiedLines.add(generateLineKey(undefined, afterLine));
				break;
			}

			case diffConstants.DELETE: {
				getLine(beforeLine, undefined).diffs.push(diff);
				modifiedLines.add(generateLineKey(beforeLine));
				break;
			}

			case diffConstants.EQUAL: {
				// We are still on the first line
				if (beforeLine === 1 && afterLine === 1) {
					beforeLineToAfter.set(1, 1);
				}

				getLine(undefined, afterLine).diffs.push(diff);
				getLine(beforeLine, undefined).diffs.push(diff);
				break;
			}
		}
	}

	for (const tuple of rawDiffs) {
		const [type, text] = tuple;

		// Get all the lines
		const parts = text.split("\n");

		// Doesn't contain a newline
		if (parts.length <= 1) {
			pushToLine(tuple);
			continue;
		}

		// Deconstruct each text chunk
		const [currentLine, ...futureLines] = parts;

		// The first chunk belongs to the current line
		if (currentLine !== "") {
			pushToLine([type, currentLine]);
		}

		// Create unique lines for each other chunk
		for (const newLine of futureLines) {
			switch (type) {
				case diffConstants.EQUAL: {
					afterLine++;
					beforeLine++;
					break;
				}

				case diffConstants.DELETE: {
					beforeLine++;
					break;
				}

				case diffConstants.ADD: {
					afterLine++;
					break;
				}
			}

			beforeLineToAfter.set(beforeLine, afterLine);
			pushToLine([type, newLine]);
		}
	}

	const beforeLineCount = beforeLine;
	const afterLineCount = afterLine;

	// Merge identical lines
	for (let beforeLine = 1; beforeLine <= beforeLineCount; beforeLine++) {
		let afterLine = beforeLineToAfter.get(beforeLine)!;
		if (!hasModifiedLine(beforeLine) && !hasModifiedLine(undefined, afterLine)) {
			const line = getLine(beforeLine);
			insertedLines.delete(generateLineKey(beforeLine, undefined));
			insertedLines.delete(generateLineKey(undefined, afterLine));
			insertedLines.set(
				generateLineKey(beforeLine, afterLine),
				{beforeLine, afterLine, diffs: line.diffs},
			);
		}
	}

	const diffsByLineWithBeforeAndShared: Array<GroupDiffsLine> = [];

	// Print before lines, including those that are shared
	for (let beforeLine = 1; beforeLine <= beforeLineCount; beforeLine++) {
		const line = maybeGetLine(beforeLine);
		if (line !== undefined) {
			diffsByLineWithBeforeAndShared.push(line);
		}

		// If we have a shared line then add it
		const afterLine = beforeLineToAfter.get(beforeLine);
		if (afterLine !== undefined) {
			const line = maybeGetLine(beforeLine, afterLine);
			if (line !== undefined) {
				diffsByLineWithBeforeAndShared.push(line);
			}
		}
	}

	let lastPrintedAfter = 0;
	let diffsByLine: Array<GroupDiffsLine> = [];

	function catchUpAfter(afterLine: number) {
		for (let i = lastPrintedAfter + 1; i <= afterLine; i++) {
			const line = maybeGetLine(undefined, i);
			if (line !== undefined) {
				diffsByLine.push(line);
			}
		}
		lastPrintedAfter = afterLine;
	}

	// Catch up after lines when we hit a shared line
	for (const line of diffsByLineWithBeforeAndShared) {
		const {afterLine} = line;
		if (afterLine !== undefined) {
			catchUpAfter(afterLine);
		}
		diffsByLine.push(line);
	}

	// Push on remaining lines at the end
	catchUpAfter(afterLineCount);

	return {
		diffsByLine,
		beforeLineCount,
		afterLineCount,
	};
}

export default function stringDiff(text1: string, text2: string): Diffs {
	// only pass fix_unicode=true at the top level, not when main is
	// recursively invoked
	return main(text1, text2, true);
}

/**
 * Find the differences between two texts.  Simplifies the problem by stripping
 * any common prefix or suffix off the texts before diffing.
 * @param {string} text1 Old string to be diffed.
 * @param {string} text2 New string to be diffed.
 * @param {Int|Object} [cursor_pos] Edit position in text1 or object with more info
 * @return {Array} Array of diff tuples.
 */
function main(text1: string, text2: string, fixUnicode: boolean = false): Diffs {
	// Check for equality
	if (text1 === text2) {
		if (text1) {
			return [[DIFF_EQUAL, text1]];
		}
		return [];
	}

	// Trim off common prefix (speedup).
	let commonlength = commonPrefix(text1, text2);
	let commonprefix = text1.substring(0, commonlength);
	text1 = text1.substring(commonlength);
	text2 = text2.substring(commonlength);

	// Trim off common suffix (speedup).
	commonlength = commonSuffix(text1, text2);
	let commonsuffix = text1.substring(text1.length - commonlength);
	text1 = text1.substring(0, text1.length - commonlength);
	text2 = text2.substring(0, text2.length - commonlength);

	// Compute the diff on the middle block.
	let diffs = compute(text1, text2);

	// Restore the prefix and suffix.
	if (commonprefix) {
		diffs.unshift([DIFF_EQUAL, commonprefix]);
	}
	if (commonsuffix) {
		diffs.push([DIFF_EQUAL, commonsuffix]);
	}
	cleanupMerge(diffs, fixUnicode);
	return diffs;
}

/**
 * Find the differences between two texts.  Assumes that the texts do not
 * have any common prefix or suffix.
 * @param {string} text1 Old string to be diffed.
 * @param {string} text2 New string to be diffed.
 * @return {Array} Array of diff tuples.
 */
function compute(text1: string, text2: string): Diffs {
	let diffs: Diffs = [];

	if (!text1) {
		// Just add some text (speedup).
		return [[DIFF_INSERT, text2]];
	}

	if (!text2) {
		// Just delete some text (speedup).
		return [[DIFF_DELETE, text1]];
	}

	let longtext = text1.length > text2.length ? text1 : text2;
	let shorttext = text1.length > text2.length ? text2 : text1;
	let i = longtext.indexOf(shorttext);
	if (i !== -1) {
		// Shorter text is inside the longer text (speedup).
		diffs = [
			[DIFF_INSERT, longtext.substring(0, i)],
			[DIFF_EQUAL, shorttext],
			[DIFF_INSERT, longtext.substring(i + shorttext.length)],
		];
		// Swap insertions for deletions if diff is reversed.
		if (text1.length > text2.length) {
			diffs[0][0] = diffs[2][0] = DIFF_DELETE;
		}
		return diffs;
	}

	if (shorttext.length === 1) {
		// Single character string.
		// After the previous speedup, the character can't be an equality.
		return [[DIFF_DELETE, text1], [DIFF_INSERT, text2]];
	}

	// Check to see if the problem can be split in two.
	let hm = halfMatch(text1, text2);
	if (hm) {
		// A half-match was found, sort out the return data.
		let text1A = hm[0];
		let text1B = hm[1];
		let text2A = hm[2];
		let text2B = hm[3];
		let midCommon = hm[4];
		// Send both pairs off for separate processing.
		let diffsA: Diffs = main(text1A, text2A);
		let diffsB: Diffs = main(text1B, text2B);
		// Merge the results.
		return diffsA.concat([[DIFF_EQUAL, midCommon]], diffsB);
	}

	return bisect(text1, text2);
}

/**
 * Find the 'middle snake' of a diff, split the problem in two
 * and return the recursively constructed diff.
 * See Myers 1986 paper: An O(ND) Difference Algorithm and Its Variations.
 * @param {string} text1 Old string to be diffed.
 * @param {string} text2 New string to be diffed.
 * @return {Array} Array of diff tuples.
 * @private
 */
function bisect(text1: string, text2: string): Diffs {
	// Cache the text lengths to prevent multiple calls.
	let text1Length = text1.length;
	let text2Length = text2.length;
	let maxD = Math.ceil((text1Length + text2Length) / 2);
	let vOffset = maxD;
	let vLength = 2 * maxD;
	let v1 = new Array(vLength);
	let v2 = new Array(vLength);

	// Setting all elements to -1 is faster in Chrome & Firefox than mixing
	// integers and undefined.
	for (let x = 0; x < vLength; x++) {
		v1[x] = -1;
		v2[x] = -1;
	}
	v1[vOffset + 1] = 0;
	v2[vOffset + 1] = 0;
	let delta = text1Length - text2Length;

	// If the total number of characters is odd, then the front path will collide
	// with the reverse path.
	let front = delta % 2 !== 0;

	// Offsets for start and end of k loop.
	// Prevents mapping of space beyond the grid.
	let k1Start = 0;
	let k1End = 0;
	let k2Start = 0;
	let k2End = 0;
	for (let d = 0; d < maxD; d++) {
		// Walk the front path one step.
		for (let k1 = -d + k1Start; k1 <= d - k1End; k1 += 2) {
			let k1Offset = vOffset + k1;
			let x1;
			if (k1 === -d || (k1 !== d && v1[k1Offset - 1] < v1[k1Offset + 1])) {
				x1 = v1[k1Offset + 1];
			} else {
				x1 = v1[k1Offset - 1] + 1;
			}
			let y1 = x1 - k1;
			while (
				x1 < text1Length &&
				y1 < text2Length &&
				text1.charAt(x1) === text2.charAt(y1)
			) {
				x1++;
				y1++;
			}
			v1[k1Offset] = x1;
			if (x1 > text1Length) {
				// Ran off the right of the graph.
				k1End += 2;
			} else if (y1 > text2Length) {
				// Ran off the bottom of the graph.
				k1Start += 2;
			} else if (front) {
				let k2Offset = vOffset + delta - k1;
				if (k2Offset >= 0 && k2Offset < vLength && v2[k2Offset] !== -1) {
					// Mirror x2 onto top-left coordinate system.
					let x2 = text1Length - v2[k2Offset];
					if (x1 >= x2) {
						// Overlap detected.
						return bisectSplit(text1, text2, x1, y1);
					}
				}
			}
		}

		// Walk the reverse path one step.
		for (let k2 = -d + k2Start; k2 <= d - k2End; k2 += 2) {
			let k2Offset = vOffset + k2;
			let x2: number;
			if (k2 === -d || (k2 !== d && v2[k2Offset - 1] < v2[k2Offset + 1])) {
				x2 = v2[k2Offset + 1];
			} else {
				x2 = v2[k2Offset - 1] + 1;
			}
			let y2 = x2 - k2;
			while (
				x2 < text1Length &&
				y2 < text2Length &&
				text1.charAt(text1Length - x2 - 1) ===
				text2.charAt(text2Length - y2 - 1)
			) {
				x2++;
				y2++;
			}
			v2[k2Offset] = x2;
			if (x2 > text1Length) {
				// Ran off the left of the graph.
				k2End += 2;
			} else if (y2 > text2Length) {
				// Ran off the top of the graph.
				k2Start += 2;
			} else if (!front) {
				let k1Offset = vOffset + delta - k2;
				if (k1Offset >= 0 && k1Offset < vLength && v1[k1Offset] !== -1) {
					let x1 = v1[k1Offset];
					let y1 = vOffset + x1 - k1Offset;
					// Mirror x2 onto top-left coordinate system.
					x2 = text1Length - x2;
					if (x1 >= x2) {
						// Overlap detected.
						return bisectSplit(text1, text2, x1, y1);
					}
				}
			}
		}
	}

	// Diff took too long and hit the deadline or
	// number of diffs equals number of characters, no commonality at all.
	return [[DIFF_DELETE, text1], [DIFF_INSERT, text2]];
}

/**
 * Given the location of the 'middle snake', split the diff in two parts
 * and recurse.
 * @param {string} text1 Old string to be diffed.
 * @param {string} text2 New string to be diffed.
 * @param {number} x Index of split point in text1.
 * @param {number} y Index of split point in text2.
 * @return {Array} Array of diff tuples.
 */
function bisectSplit(text1: string, text2: string, x: number, y: number): Diffs {
	let text1A = text1.substring(0, x);
	let text2A = text2.substring(0, y);
	let text1B = text1.substring(x);
	let text2B = text2.substring(y);

	// Compute both diffs serially.
	let diffs = main(text1A, text2A);
	let diffsb = main(text1B, text2B);

	return diffs.concat(diffsb);
}

/**
 * Determine the common prefix of two strings.
 * @param {string} text1 First string.
 * @param {string} text2 Second string.
 * @return {number} The number of characters common to the start of each
 *     string.
 */
function commonPrefix(text1: string, text2: string): number {
	// Quick check for common null cases.
	if (!text1 || !text2 || text1.charAt(0) !== text2.charAt(0)) {
		return 0;
	}

	// Binary search.
	// Performance analysis: http://neil.fraser.name/news/2007/10/09/
	let pointermin = 0;
	let pointermax = Math.min(text1.length, text2.length);
	let pointermid = pointermax;
	let pointerstart = 0;
	while (pointermin < pointermid) {
		if (
			text1.substring(pointerstart, pointermid) ===
			text2.substring(pointerstart, pointermid)
		) {
			pointermin = pointermid;
			pointerstart = pointermin;
		} else {
			pointermax = pointermid;
		}
		pointermid = Math.floor((pointermax - pointermin) / 2 + pointermin);
	}

	if (isSurrogatePairStart(text1.charCodeAt(pointermid - 1))) {
		pointermid--;
	}

	return pointermid;
}

/**
 * Determine the common suffix of two strings.
 * @param {string} text1 First string.
 * @param {string} text2 Second string.
 * @return {number} The number of characters common to the end of each string.
 */
function commonSuffix(text1: string, text2: string): number {
	// Quick check for common null cases.
	if (!text1 || !text2 || text1.slice(-1) !== text2.slice(-1)) {
		return 0;
	}

	// Binary search.

	// Performance analysis: http://neil.fraser.name/news/2007/10/09/
	let pointermin = 0;
	let pointermax = Math.min(text1.length, text2.length);
	let pointermid = pointermax;
	let pointerend = 0;
	while (pointermin < pointermid) {
		if (
			text1.substring(text1.length - pointermid, text1.length - pointerend) ===
			text2.substring(text2.length - pointermid, text2.length - pointerend)
		) {
			pointermin = pointermid;
			pointerend = pointermin;
		} else {
			pointermax = pointermid;
		}
		pointermid = Math.floor((pointermax - pointermin) / 2 + pointermin);
	}

	if (isSurrogatePairEnd(text1.charCodeAt(text1.length - pointermid))) {
		pointermid--;
	}

	return pointermid;
}

/**
 * Do the two texts share a substring which is at least half the length of the
 * longer text?
 * This speedup can produce non-minimal diffs.
 * @param {string} text1 First string.
 * @param {string} text2 Second string.
 * @return {Array.<string>} Five element Array, containing the prefix of
 *     text1, the suffix of text1, the prefix of text2, the suffix of
 *     text2 and the common middle.  Or null if there was no match.
 */
function halfMatch(text1: string, text2: string): undefined | HalfMatch {
	let longtext = text1.length > text2.length ? text1 : text2;
	let shorttext = text1.length > text2.length ? text2 : text1;
	if (longtext.length < 4 || shorttext.length * 2 < longtext.length) {
		return undefined; // Pointless.
	}

	// First check if the second quarter is the seed for a half-match.
	let hm1 = halfMatchI(longtext, shorttext, Math.ceil(longtext.length / 4));

	// Check again based on the third quarter.
	let hm2 = halfMatchI(longtext, shorttext, Math.ceil(longtext.length / 2));

	let hm: undefined | HalfMatch;
	if (!hm1 && !hm2) {
		return undefined;
	} else if (hm2) {
		if (hm1) {
			// Both matched.  Select the longest.
			hm = hm1[4].length > hm2[4].length ? hm1 : hm2;
		} else {
			hm = hm2;
		}
	} else {
		hm = hm1;
	}

	if (hm === undefined) {
		throw new Error("Expected half match");
	}

	// A half-match was found, sort out the return data.
	let text1A;
	let text1B;
	let text2A;
	let text2B;
	if (text1.length > text2.length) {
		text1A = hm[0];
		text1B = hm[1];
		text2A = hm[2];
		text2B = hm[3];
	} else {
		text2A = hm[0];
		text2B = hm[1];
		text1A = hm[2];
		text1B = hm[3];
	}
	let midCommon = hm[4];
	return [text1A, text1B, text2A, text2B, midCommon];
}

/**
 * Does a substring of shorttext exist within longtext such that the substring
 * is at least half the length of longtext?
 * Closure, but does not reference any external variables.
 * @param {string} longtext Longer string.
 * @param {string} shorttext Shorter string.
 * @param {number} i Start index of quarter length substring within longtext.
 * @return {Array.<string>} Five element Array, containing the prefix of
 *     longtext, the suffix of longtext, the prefix of shorttext, the suffix
 *     of shorttext and the common middle.  Or null if there was no match.
 * @private
 */
function halfMatchI(
	longtext: string,
	shorttext: string,
	i: number,
): undefined | [string, string, string, string, string] {
	// Start with a 1/4 length substring at position i as a seed.
	let seed = longtext.substring(i, i + Math.floor(longtext.length / 4));
	let j = -1;
	let bestCommon = "";
	let bestLongtextA = "";
	let bestLongtextB = "";
	let bestShorttextA = "";
	let bestShorttextB = "";
	while ((j = shorttext.indexOf(seed, j + 1)) !== -1) {
		let prefixLength = commonPrefix(
			longtext.substring(i),
			shorttext.substring(j),
		);
		let suffixLength = commonSuffix(
			longtext.substring(0, i),
			shorttext.substring(0, j),
		);
		if (bestCommon.length < suffixLength + prefixLength) {
			bestCommon =
				shorttext.substring(j - suffixLength, j) +
				shorttext.substring(j, j + prefixLength);
			bestLongtextA = longtext.substring(0, i - suffixLength);
			bestLongtextB = longtext.substring(i + prefixLength);
			bestShorttextA = shorttext.substring(0, j - suffixLength);
			bestShorttextB = shorttext.substring(j + prefixLength);
		}
	}

	if (bestCommon.length * 2 >= longtext.length) {
		return [
			bestLongtextA,
			bestLongtextB,
			bestShorttextA,
			bestShorttextB,
			bestCommon,
		];
	} else {
		return undefined;
	}
}

/**
 * Reorder and merge like edit sections.  Merge equalities.
 * Any edit section can move as long as it doesn't cross an equality.
 * @param {Array} diffs Array of diff tuples.
 * @param {boolean} fix_unicode Whether to normalize to a unicode-correct diff
 */
function cleanupMerge(diffs: Diffs, fixUnicode: boolean) {
	diffs.push([DIFF_EQUAL, ""]); // Add a dummy entry at the end.
	let pointer = 0;
	let countDelete = 0;
	let countInsert = 0;
	let textDelete = "";
	let textInsert = "";
	let commonlength;
	while (pointer < diffs.length) {
		if (pointer < diffs.length - 1 && !diffs[pointer][1]) {
			diffs.splice(pointer, 1);
			continue;
		}
		switch (diffs[pointer][0]) {
			case DIFF_INSERT: {
				countInsert++;
				textInsert += diffs[pointer][1];
				pointer++;
				break;
			}
			case DIFF_DELETE: {
				countDelete++;
				textDelete += diffs[pointer][1];
				pointer++;
				break;
			}
			case DIFF_EQUAL: {
				let previousEquality = pointer - countInsert - countDelete - 1;
				if (fixUnicode) {
					// prevent splitting of unicode surrogate pairs.  when fix_unicode is true,
					// we assume that the old and new text in the diff are complete and correct
					// unicode-encoded JS strings, but the tuple boundaries may fall between
					// surrogate pairs.  we fix this by shaving off stray surrogates from the end
					// of the previous equality and the beginning of this equality.  this may create
					// empty equalities or a common prefix or suffix.  for example, if AB and AC are
					// emojis, `[[0, 'A'], [-1, 'BA'], [0, 'C']]` would turn into deleting 'ABAC' and
					// inserting 'AC', and then the common suffix 'AC' will be eliminated.  in this
					// particular case, both equalities go away, we absorb any previous inequalities,
					// and we keep scanning for the next equality before rewriting the tuples.
					if (
						previousEquality >= 0 &&
						endsWithPairStart(diffs[previousEquality][1])
					) {
						let stray = diffs[previousEquality][1].slice(-1);
						diffs[previousEquality][1] = diffs[previousEquality][1].slice(0, -1);
						textDelete = stray + textDelete;
						textInsert = stray + textInsert;
						if (!diffs[previousEquality][1]) {
							// emptied out previous equality, so delete it and include previous delete/insert
							diffs.splice(previousEquality, 1);
							pointer--;
							let k = previousEquality - 1;
							if (diffs[k] && diffs[k][0] === DIFF_INSERT) {
								countInsert++;
								textInsert = diffs[k][1] + textInsert;
								k--;
							}
							if (diffs[k] && diffs[k][0] === DIFF_DELETE) {
								countDelete++;
								textDelete = diffs[k][1] + textDelete;
								k--;
							}
							previousEquality = k;
						}
					}
					if (startsWithPairEnd(diffs[pointer][1])) {
						let stray = diffs[pointer][1].charAt(0);
						diffs[pointer][1] = diffs[pointer][1].slice(1);
						textDelete += stray;
						textInsert += stray;
					}
				}
				if (pointer < diffs.length - 1 && !diffs[pointer][1]) {
					// for empty equality not at end, wait for next equality
					diffs.splice(pointer, 1);
					break;
				}
				if (textDelete.length > 0 || textInsert.length > 0) {
					// note that commonPrefix and commonSuffix are unicode-aware
					if (textDelete.length > 0 && textInsert.length > 0) {
						// Factor out any common prefixes.
						commonlength = commonPrefix(textInsert, textDelete);
						if (commonlength !== 0) {
							if (previousEquality >= 0) {
								diffs[previousEquality][1] += textInsert.substring(
									0,
									commonlength,
								);
							} else {
								diffs.splice(
									0,
									0,
									[DIFF_EQUAL, textInsert.substring(0, commonlength)],
								);
								pointer++;
							}
							textInsert = textInsert.substring(commonlength);
							textDelete = textDelete.substring(commonlength);
						}

						// Factor out any common suffixes.
						commonlength = commonSuffix(textInsert, textDelete);
						if (commonlength !== 0) {
							diffs[pointer][1] =
								textInsert.substring(textInsert.length - commonlength) +
								diffs[pointer][1];
							textInsert = textInsert.substring(
								0,
								textInsert.length - commonlength,
							);
							textDelete = textDelete.substring(
								0,
								textDelete.length - commonlength,
							);
						}
					}

					// Delete the offending records and add the merged ones.
					let n = countInsert + countDelete;
					if (textDelete.length === 0 && textInsert.length === 0) {
						diffs.splice(pointer - n, n);
						pointer = pointer - n;
					} else if (textDelete.length === 0) {
						diffs.splice(pointer - n, n, [DIFF_INSERT, textInsert]);
						pointer = pointer - n + 1;
					} else if (textInsert.length === 0) {
						diffs.splice(pointer - n, n, [DIFF_DELETE, textDelete]);
						pointer = pointer - n + 1;
					} else {
						diffs.splice(
							pointer - n,
							n,
							[DIFF_DELETE, textDelete],
							[DIFF_INSERT, textInsert],
						);
						pointer = pointer - n + 2;
					}
				}
				if (pointer !== 0 && diffs[pointer - 1][0] === DIFF_EQUAL) {
					// Merge this equality with the previous one.
					diffs[pointer - 1][1] += diffs[pointer][1];
					diffs.splice(pointer, 1);
				} else {
					pointer++;
				}
				countInsert = 0;
				countDelete = 0;
				textDelete = "";
				textInsert = "";
				break;
			}
		}
	}
	if (diffs[diffs.length - 1][1] === "") {
		// Remove the dummy entry at the end.
		diffs.pop();
	}

	// Second pass: look for single edits surrounded on both sides by equalities
	// which can be shifted sideways to eliminate an equality.
	// e.g: A<ins>BA</ins>C -> <ins>AB</ins>AC
	let changes = false;
	pointer = 1;

	// Intentionally ignore the first and last element (don't need checking).
	while (pointer < diffs.length - 1) {
		if (
			diffs[pointer - 1][0] === DIFF_EQUAL &&
			diffs[pointer + 1][0] === DIFF_EQUAL
		) {
			// This is a single edit surrounded by equalities.
			if (
				diffs[pointer][1].substring(
					diffs[pointer][1].length - diffs[pointer - 1][1].length,
				) ===
				diffs[pointer - 1][1]
			) {
				// Shift the edit over the previous equality.
				diffs[pointer][1] =
					diffs[pointer - 1][1] +
					diffs[pointer][1].substring(
						0,
						diffs[pointer][1].length - diffs[pointer - 1][1].length,
					);
				diffs[pointer + 1][1] = diffs[pointer - 1][1] + diffs[pointer + 1][1];
				diffs.splice(pointer - 1, 1);
				changes = true;
			} else if (
				diffs[pointer][1].substring(0, diffs[pointer + 1][1].length) ===
				diffs[pointer + 1][1]
			) {
				// Shift the edit over the next equality.
				diffs[pointer - 1][1] += diffs[pointer + 1][1];
				diffs[pointer][1] =
					diffs[pointer][1].substring(diffs[pointer + 1][1].length) +
					diffs[pointer + 1][1];
				diffs.splice(pointer + 1, 1);
				changes = true;
			}
		}
		pointer++;
	}

	// If shifts were made, the diff needs reordering and another shift sweep.
	if (changes) {
		cleanupMerge(diffs, fixUnicode);
	}
}

function isSurrogatePairStart(charCode: number): boolean {
	return charCode >= 55_296 && charCode <= 56_319;
}

function isSurrogatePairEnd(charCode: number): boolean {
	return charCode >= 56_320 && charCode <= 57_343;
}

function startsWithPairEnd(str: string): boolean {
	return isSurrogatePairEnd(str.charCodeAt(0));
}

function endsWithPairStart(str: string): boolean {
	return isSurrogatePairStart(str.charCodeAt(str.length - 1));
}
