/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	CoverageFile,
	CoverageFileStats,
	CoverageLocationRange,
	CoverageRangeWithMetadata,
	LocationRangeKind,
} from "@internal/v8";
import {SourceMapConsumer} from "@internal/codec-source-map";
import {Position, derivePositionKey} from "@internal/parser-core";
import {urlToFilename} from "./utils";
import {
	Number0,
	Number1,
	ob1Coerce0,
	ob1Get0,
	ob1Inc,
	ob1Number0,
	ob1Number1,
} from "@internal/ob1";
import inspector = require("inspector");

function createCoverageFileStats(
	covered: number,
	uncovered: number,
): CoverageFileStats {
	const total = uncovered + covered;
	return {
		uncovered,
		covered,
		total,
		percent: total === 0 ? 100 : 100 / total * covered,
	};
}

export default class CoverageCollector {
	constructor() {
		this.sourceMaps = new Map();
	}

	private sourceMaps: Map<
		string,
		{
			code: string;
			ranges: Array<CoverageRangeWithMetadata>;
			map: SourceMapConsumer;
		}
	>;

	public addSourceMap(filename: string, code: string, map: SourceMapConsumer) {
		this.sourceMaps.set(
			filename,
			{
				ranges: [],
				map,
				code,
			},
		);
	}

	public addCoverage(entries: Array<inspector.Profiler.ScriptCoverage>) {
		for (const entry of entries) {
			const filename = urlToFilename(entry.url);

			const data = this.sourceMaps.get(filename);
			if (data === undefined) {
				continue;
			}

			for (const {ranges, functionName, isBlockCoverage} of entry.functions) {
				data.ranges = data.ranges.concat(
					ranges.map((range) => {
						let kind: LocationRangeKind = "expression";
						if (functionName !== "") {
							kind = "function";
						} else if (isBlockCoverage) {
							kind = "branch";
						}

						return {
							kind,
							...range,
						};
					}),
				);
			}
		}
	}

	public generate(): Array<CoverageFile> {
		const insertedLocs: Map<string, CoverageLocationRange> = new Map();
		const locs: Array<CoverageLocationRange> = [];

		for (const data of this.sourceMaps.values()) {
			const {ranges, code, map} = data;

			// Turn an index into a position in the compiled source
			let line: Number1 = ob1Number1;
			let column: Number0 = ob1Number0;
			let index: Number0 = ob1Number0;
			const indexCache: Map<Number0, Position> = new Map();
			function findIndex(newIndex: Number0): Position {
				const cached = indexCache.get(newIndex);
				if (cached !== undefined) {
					return cached;
				}

				if (newIndex < index) {
					throw new Error(`Expected newIndex(${newIndex}) >= index(${index})`);
				}

				if (ob1Get0(newIndex) > code.length) {
					throw new Error(
						`Expected newIndex(${newIndex}) <= code.length(${code.length})`,
					);
				}

				while (index < newIndex) {
					const char = code[ob1Get0(index)];
					if (char === "\n") {
						line = ob1Inc(line);
						column = ob1Number0;
					} else {
						column = ob1Inc(column);
					}
					index = ob1Inc(index);
				}

				const pos: Position = {
					line,
					column,
				};
				indexCache.set(newIndex, pos);
				return pos;
			}

			// Prefetch all sorted indexes
			const offsets: Array<Number0> = [];
			for (const {startOffset, endOffset} of ranges) {
				offsets.push(ob1Coerce0(startOffset));
				offsets.push(ob1Coerce0(endOffset));
			}
			offsets.sort((a, b) => ob1Get0(a) - ob1Get0(b));
			for (const index of offsets) {
				findIndex(index);
			}

			//
			for (const {kind, startOffset, endOffset, count} of ranges) {
				const originalStart = findIndex(ob1Coerce0(startOffset));
				const originalEnd = findIndex(ob1Coerce0(endOffset));

				const sourceStart = map.approxOriginalPositionFor(
					originalStart.line,
					originalStart.column,
				);
				if (sourceStart === undefined) {
					continue;
				}

				const sourceEnd = map.approxOriginalPositionFor(
					originalEnd.line,
					originalEnd.column,
				);
				if (sourceEnd === undefined) {
					continue;
				}

				if (sourceStart.source !== sourceEnd.source) {
					throw new Error(
						`Expected the same source for start and end: ${sourceStart.source} !== ${sourceEnd.source}`,
					);
				}

				const key = `${sourceStart.source}:${String(startOffset)}-${String(
					endOffset,
				)}`;
				const alreadyInserted = insertedLocs.get(key);
				if (alreadyInserted !== undefined) {
					alreadyInserted.count += count;
					continue;
				}

				const loc: CoverageLocationRange = {
					kind,
					filename: sourceStart.source,
					count,
					start: {
						line: sourceStart.line,
						column: sourceStart.column,
					},
					end: {
						line: sourceEnd.line,
						column: sourceEnd.column,
					},
				};
				insertedLocs.set(key, loc);
				locs.push(loc);
			}

			map.clearCache();
		}

		// Assemble files
		const rangesByFile: Map<string, Array<CoverageLocationRange>> = new Map();
		for (const loc of locs) {
			let ranges = rangesByFile.get(loc.filename);
			if (ranges === undefined) {
				ranges = [];
				rangesByFile.set(loc.filename, ranges);
			}
			ranges.push(loc);
		}

		const files: Array<CoverageFile> = [];
		for (const [filename, ranges] of rangesByFile) {
			const coveredLines: Set<Number1> = new Set();
			const uncoveredLines: Set<Number1> = new Set();

			let uncoveredFunctions: Set<Number1> = new Set();
			let coveredFunctions: Set<Number1> = new Set();
			let uncoveredBranches: Set<string> = new Set();
			let coveredBranches: Set<string> = new Set();

			for (const {count, kind, start, end} of ranges) {
				// Fill in lines
				for (let i = start.line; i <= end.line; i = ob1Inc(i)) {
					if (count === 0) {
						uncoveredLines.add(i);
					} else {
						coveredLines.add(i);
					}
				}

				// Mark covered kind
				if (kind === "function") {
					if (count === 0) {
						uncoveredBranches.add(derivePositionKey(start));
						uncoveredFunctions.add(start.line);
					} else {
						coveredFunctions.add(start.line);
						coveredBranches.add(derivePositionKey(start));
					}
				} else if (kind === "branch") {
					if (count === 0) {
						uncoveredBranches.add(derivePositionKey(start));
					} else {
						coveredBranches.add(derivePositionKey(start));
					}
				}
			}

			for (const line of coveredLines) {
				uncoveredLines.delete(line);
			}

			for (const index of coveredBranches) {
				uncoveredBranches.delete(index);
			}

			for (const index of coveredFunctions) {
				uncoveredFunctions.delete(index);
			}

			// No point showing fully covered files
			if (
				uncoveredLines.size === 0 &&
				uncoveredBranches.size === 0 &&
				uncoveredFunctions.size === 0
			) {
				continue;
			}

			files.push({
				filename,
				lines: createCoverageFileStats(coveredLines.size, uncoveredLines.size),
				branches: createCoverageFileStats(
					coveredBranches.size,
					uncoveredBranches.size,
				),
				functions: createCoverageFileStats(
					coveredFunctions.size,
					uncoveredFunctions.size,
				),
			});
		}
		return files;
	}
}
