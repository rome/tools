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

/**
 * The data structure representing a diff is an array of tuples:
 * [[DiffTypes.DELETE, 'Hello'], [DiffTypes.INSERT, 'Goodbye'], [DiffTypes.EQUAL, ' world.']]
 * which means: delete 'Hello', add 'Goodbye' and keep ' world.'
 */

export type Diff = [
	DiffTypes.DELETE | DiffTypes.EQUAL | DiffTypes.INSERT,
	string
];

export type CompressedDiff = Diff | [DiffTypes.EQUAL_COMPRESSED_LINES, number];

type HalfMatch = undefined | [string, string, string, string, string];

export enum DiffTypes {
	DELETE,
	EQUAL,
	INSERT,
	EQUAL_COMPRESSED_LINES,
}
export type UnifiedDiff = {
	diffsByLine: GroupDiffsLine[];
	beforeLineCount: number;
	afterLineCount: number;
};

export type GroupDiffsLine = {
	beforeLine?: number;
	afterLine?: number;
	diffs: Diff[];
};

export const COMPRESSED_DIFFS_CONTEXT_LINES = 2;

export function generateLineKey(beforeLine?: number, afterLine?: number) {
	return `${beforeLine || ""}:${afterLine || ""}`;
}

export function stringDiffUnified(rawDiffs: CompressedDiff[]): UnifiedDiff {
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
			case DiffTypes.INSERT: {
				getLine(undefined, afterLine).diffs.push(diff);
				modifiedLines.add(generateLineKey(undefined, afterLine));
				break;
			}

			case DiffTypes.DELETE: {
				getLine(beforeLine, undefined).diffs.push(diff);
				modifiedLines.add(generateLineKey(beforeLine));
				break;
			}

			case DiffTypes.EQUAL: {
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

	for (let i = 0; i < rawDiffs.length; i++) {
		const tuple = rawDiffs[i];

		if (tuple[0] === DiffTypes.EQUAL_COMPRESSED_LINES) {
			const isFirstTuple = i === 0;
			for (let i = 0; i < tuple[1]; i++) {
				// Don't increment the first line if we are the first tuple marking the beginning of the file
				if (!(i === 0 && isFirstTuple)) {
					afterLine++;
					beforeLine++;
				}
				beforeLineToAfter.set(beforeLine, afterLine);
				pushToLine([DiffTypes.EQUAL, ""]);
			}
			continue;
		}

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
				case DiffTypes.EQUAL: {
					afterLine++;
					beforeLine++;
					break;
				}

				case DiffTypes.DELETE: {
					beforeLine++;
					break;
				}

				case DiffTypes.INSERT: {
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
		if (!(hasModifiedLine(beforeLine) || hasModifiedLine(undefined, afterLine))) {
			const line = getLine(beforeLine);
			insertedLines.delete(generateLineKey(beforeLine, undefined));
			insertedLines.delete(generateLineKey(undefined, afterLine));
			insertedLines.set(
				generateLineKey(beforeLine, afterLine),
				{
					beforeLine,
					afterLine,
					diffs: line.diffs,
				},
			);
		}
	}

	const diffsByLineWithBeforeAndShared: GroupDiffsLine[] = [];

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
	let diffsByLine: GroupDiffsLine[] = [];

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

export default function stringDiff(a: string, b: string): Diff[] {
	// only pass fix_unicode=true at the top level, not when main is
	// recursively invoked
	return main(a, b, true);
}

export function stringDiffCompressed(a: string, b: string): CompressedDiff[] {
	const diffs = stringDiff(a, b);
	const compressed: CompressedDiff[] = [];

	for (let i = 0; i < diffs.length; i++) {
		const diff = diffs[i];

		if (diff[0] !== DiffTypes.EQUAL) {
			compressed.push(diff);
			continue;
		}

		const lines = diff[1].split("\n");

		const startLines = lines.slice(0, 1 + COMPRESSED_DIFFS_CONTEXT_LINES);
		const endLines = lines.slice(-(1 + COMPRESSED_DIFFS_CONTEXT_LINES));

		const compressedCount = lines.length - startLines.length - endLines.length;
		if (compressedCount <= 0) {
			compressed.push(diff);
			continue;
		}

		if (startLines.length > 0) {
			compressed.push([DiffTypes.EQUAL, startLines.join("\n")]);
		}

		compressed.push([DiffTypes.EQUAL_COMPRESSED_LINES, compressedCount]);

		if (endLines.length > 0) {
			compressed.push([DiffTypes.EQUAL, endLines.join("\n")]);
		}
	}

	return compressed;
}

/**
 * Find the differences between two texts.  Simplifies the problem by stripping
 * any common prefix or suffix off the texts before diffing.
 * @param {string} a Old string to be diffed.
 * @param {string} b New string to be diffed.
 * @param {Int|Object} [cursor_pos] Edit position in a or object with more info
 * @return {Array} Array of diff tuples.
 */
export function main(a: string, b: string, fixUnicode: boolean = false): Diff[] {
	// Check for equality
	if (a === b) {
		if (a) {
			return [[DiffTypes.EQUAL, a]];
		}
		return [];
	}

	// Extract common prefix
	const prefixLength = getCommonPrefix(a, b);
	const prefix = a.substring(0, prefixLength);
	a = a.substring(prefixLength);
	b = b.substring(prefixLength);

	// Extract common suffix
	const suffixLength = getCommonSuffix(a, b);
	const suffix = a.substring(a.length - suffixLength);
	a = a.substring(0, a.length - suffixLength);
	b = b.substring(0, b.length - suffixLength);

	// Compute the diff on the middle block.
	const diffs = compute(a, b);

	// Restore the prefix and suffix.
	if (prefix) {
		diffs.unshift([DiffTypes.EQUAL, prefix]);
	}
	if (suffix) {
		diffs.push([DiffTypes.EQUAL, suffix]);
	}
	cleanupMerge(diffs, fixUnicode);
	return diffs;
}

/**
 * Find the differences between two texts.  Assumes that the texts do not
 * have any common prefix or suffix.
 * @param {string} a Old string to be diffed.
 * @param {string} b New string to be diffed.
 * @return {Array} Array of diff tuples.
 */
function compute(a: string, b: string): Diff[] {
	let diffs: Diff[] = [];

	if (!a) {
		// Just add some text (speedup).
		return [[DiffTypes.INSERT, b]];
	}

	if (!b) {
		// Just delete some text (speedup).
		return [[DiffTypes.DELETE, a]];
	}

	let longtext = a.length > b.length ? a : b;
	let shorttext = a.length > b.length ? b : a;
	let i = longtext.indexOf(shorttext);
	if (i !== -1) {
		// Shorter text is inside the longer text (speedup).
		diffs = [
			[DiffTypes.INSERT, longtext.substring(0, i)],
			[DiffTypes.EQUAL, shorttext],
			[DiffTypes.INSERT, longtext.substring(i + shorttext.length)],
		];
		// Swap insertions for deletions if diff is reversed.
		if (a.length > b.length) {
			diffs[0][0] = diffs[2][0] = DiffTypes.DELETE;
		}
		return diffs;
	}

	if (shorttext.length === 1) {
		// Single character string.
		// After the previous speedup, the character can't be an equality.
		return [[DiffTypes.DELETE, a], [DiffTypes.INSERT, b]];
	}

	// Check to see if the problem can be split in two.
	let hm = halfMatch(a, b);
	if (hm) {
		// A half-match was found, sort out the return data.
		let aA = hm[0];
		let aB = hm[1];
		let bA = hm[2];
		let bB = hm[3];
		let midCommon = hm[4];
		// Send both pairs off for separate processing.
		let diffsA: Diff[] = main(aA, bA);
		let diffsB: Diff[] = main(aB, bB);
		// Merge the results.
		return diffsA.concat([[DiffTypes.EQUAL, midCommon]], diffsB);
	}

	return bisect(a, b);
}

/**
 * Find the 'middle snake' of a diff, split the problem in two
 * and return the recursively constructed diff.
 * See Myers 1986 paper: An O(ND) Difference Algorithm and Its Variations.
 * @param {string} a Old string to be diffed.
 * @param {string} b New string to be diffed.
 * @return {Array} Array of diff tuples.
 * @private
 */
export function bisect(a: string, b: string): Diff[] {
	// Cache the text lengths to prevent multiple calls.
	let aLength = a.length;
	let bLength = b.length;
	let maxD = Math.ceil((aLength + bLength) / 2);
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
	let delta = aLength - bLength;

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
			while (x1 < aLength && y1 < bLength && a.charAt(x1) === b.charAt(y1)) {
				x1++;
				y1++;
			}
			v1[k1Offset] = x1;
			if (x1 > aLength) {
				// Ran off the right of the graph.
				k1End += 2;
			} else if (y1 > bLength) {
				// Ran off the bottom of the graph.
				k1Start += 2;
			} else if (front) {
				let k2Offset = vOffset + delta - k1;
				if (k2Offset >= 0 && k2Offset < vLength && v2[k2Offset] !== -1) {
					// Mirror x2 onto top-left coordinate system.
					let x2 = aLength - v2[k2Offset];
					if (x1 >= x2) {
						// Overlap detected.
						return bisectSplit(a, b, x1, y1);
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
				x2 < aLength &&
				y2 < bLength &&
				a.charAt(aLength - x2 - 1) === b.charAt(bLength - y2 - 1)
			) {
				x2++;
				y2++;
			}
			v2[k2Offset] = x2;
			if (x2 > aLength) {
				// Ran off the left of the graph.
				k2End += 2;
			} else if (y2 > bLength) {
				// Ran off the top of the graph.
				k2Start += 2;
			} else if (!front) {
				let k1Offset = vOffset + delta - k2;
				if (k1Offset >= 0 && k1Offset < vLength && v1[k1Offset] !== -1) {
					let x1 = v1[k1Offset];
					let y1 = vOffset + x1 - k1Offset;
					// Mirror x2 onto top-left coordinate system.
					x2 = aLength - x2;
					if (x1 >= x2) {
						// Overlap detected.
						return bisectSplit(a, b, x1, y1);
					}
				}
			}
		}
	}

	// Diff took too long and hit the deadline or
	// number of diffs equals number of characters, no commonality at all.
	return [[DiffTypes.DELETE, a], [DiffTypes.INSERT, b]];
}

/**
 * Given the location of the 'middle snake', split the diff in two parts
 * and recurse.
 * @param {string} a Old string to be diffed.
 * @param {string} b New string to be diffed.
 * @param {number} x Index of split point in a.
 * @param {number} y Index of split point in b.
 * @return {Array} Array of diff tuples.
 */
function bisectSplit(a: string, b: string, x: number, y: number): Diff[] {
	let aA = a.substring(0, x);
	let bA = b.substring(0, y);
	let aB = a.substring(x);
	let bB = b.substring(y);

	// Compute both diffs serially.
	let diffs = main(aA, bA);
	let diffsb = main(aB, bB);

	return diffs.concat(diffsb);
}

/**
 * Determine the common prefix of two strings.
 * @param {string} a First string.
 * @param {string} b Second string.
 * @return {number} The number of characters common to the start of each string.
 */
export function getCommonPrefix(a: string, b: string): number {
	// Quick check for common null cases.
	if (!(a && b) || a.charAt(0) !== b.charAt(0)) {
		return 0;
	}

	// Binary search.
	// Performance analysis: http://neil.fraser.name/news/2007/10/09/
	let pointermin = 0;
	let pointermax = Math.min(a.length, b.length);
	let pointermid = pointermax;
	let pointerstart = 0;
	while (pointermin < pointermid) {
		if (
			a.substring(pointerstart, pointermid) ===
			b.substring(pointerstart, pointermid)
		) {
			pointermin = pointermid;
			pointerstart = pointermin;
		} else {
			pointermax = pointermid;
		}
		pointermid = Math.floor((pointermax - pointermin) / 2 + pointermin);
	}

	if (isSurrogatePairStart(a.charCodeAt(pointermid - 1))) {
		pointermid--;
	}

	return pointermid;
}

/**
 * Determine the common suffix of two strings.
 * @param {string} a First string.
 * @param {string} b Second string.
 * @return {number} The number of characters common to the end of each string.
 */
export function getCommonSuffix(a: string, b: string): number {
	// Quick check for common null cases.
	if (!(a && b) || a.slice(-1) !== b.slice(-1)) {
		return 0;
	}

	// Binary search.

	// Performance analysis: http://neil.fraser.name/news/2007/10/09/
	let pointermin = 0;
	let pointermax = Math.min(a.length, b.length);
	let pointermid = pointermax;
	let pointerend = 0;
	while (pointermin < pointermid) {
		if (
			a.substring(a.length - pointermid, a.length - pointerend) ===
			b.substring(b.length - pointermid, b.length - pointerend)
		) {
			pointermin = pointermid;
			pointerend = pointermin;
		} else {
			pointermax = pointermid;
		}
		pointermid = Math.floor((pointermax - pointermin) / 2 + pointermin);
	}

	if (isSurrogatePairEnd(a.charCodeAt(a.length - pointermid))) {
		pointermid--;
	}

	return pointermid;
}

/**
 * Do the two texts share a substring which is at least half the length of the
 * longer text?
 * This speedup can produce non-minimal diffs.
 * @param {string} a First string.
 * @param {string} b Second string.
 * @return {Array.<string>} Five element Array, containing the prefix of
 *     a, the suffix of a, the prefix of b, the suffix of
 *     b and the common middle.  Or null if there was no match.
 */
export function halfMatch(a: string, b: string): undefined | HalfMatch {
	let longtext = a.length > b.length ? a : b;
	let shorttext = a.length > b.length ? b : a;
	if (longtext.length < 4 || shorttext.length * 2 < longtext.length) {
		return undefined; // Pointless.
	}

	// First check if the second quarter is the seed for a half-match.
	let hm1 = halfMatchI(longtext, shorttext, Math.ceil(longtext.length / 4));

	// Check again based on the third quarter.
	let hm2 = halfMatchI(longtext, shorttext, Math.ceil(longtext.length / 2));

	let hm: undefined | HalfMatch;
	if (hm1 || hm2) {
		if (hm2) {
			if (hm1) {
				// Both matched.  Select the longest.
				hm = hm1[4].length > hm2[4].length ? hm1 : hm2;
			} else {
				hm = hm2;
			}
		} else {
			hm = hm1;
		}
	} else {
		return undefined;
	}

	if (hm === undefined) {
		throw new Error("Expected half match");
	}

	// A half-match was found, sort out the return data.
	let aA;
	let aB;
	let bA;
	let bB;
	if (a.length > b.length) {
		aA = hm[0];
		aB = hm[1];
		bA = hm[2];
		bB = hm[3];
	} else {
		bA = hm[0];
		bB = hm[1];
		aA = hm[2];
		aB = hm[3];
	}
	let midCommon = hm[4];
	return [aA, aB, bA, bB, midCommon];
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
		let prefixLength = getCommonPrefix(
			longtext.substring(i),
			shorttext.substring(j),
		);
		let suffixLength = getCommonSuffix(
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
export function cleanupMerge(diffs: Diff[], fixUnicode: boolean) {
	diffs.push([DiffTypes.EQUAL, ""]); // Add a dummy entry at the end.
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
			case DiffTypes.INSERT: {
				countInsert++;
				textInsert += diffs[pointer][1];
				pointer++;
				break;
			}
			case DiffTypes.DELETE: {
				countDelete++;
				textDelete += diffs[pointer][1];
				pointer++;
				break;
			}
			case DiffTypes.EQUAL: {
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
							if (diffs[k] && diffs[k][0] === DiffTypes.INSERT) {
								countInsert++;
								textInsert = diffs[k][1] + textInsert;
								k--;
							}
							if (diffs[k] && diffs[k][0] === DiffTypes.DELETE) {
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
						commonlength = getCommonPrefix(textInsert, textDelete);
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
									[DiffTypes.EQUAL, textInsert.substring(0, commonlength)],
								);
								pointer++;
							}
							textInsert = textInsert.substring(commonlength);
							textDelete = textDelete.substring(commonlength);
						}

						// Factor out any common suffixes.
						commonlength = getCommonSuffix(textInsert, textDelete);
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
						diffs.splice(pointer - n, n, [DiffTypes.INSERT, textInsert]);
						pointer = pointer - n + 1;
					} else if (textInsert.length === 0) {
						diffs.splice(pointer - n, n, [DiffTypes.DELETE, textDelete]);
						pointer = pointer - n + 1;
					} else {
						diffs.splice(
							pointer - n,
							n,
							[DiffTypes.DELETE, textDelete],
							[DiffTypes.INSERT, textInsert],
						);
						pointer = pointer - n + 2;
					}
				}
				if (pointer !== 0 && diffs[pointer - 1][0] === DiffTypes.EQUAL) {
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
			diffs[pointer - 1][0] === DiffTypes.EQUAL &&
			diffs[pointer + 1][0] === DiffTypes.EQUAL
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
