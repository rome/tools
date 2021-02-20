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
import {OneIndexed, ZeroIndexed} from "@internal/math";
import inspector = require("inspector");
import {MixedPathMap} from "@internal/path";

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
			ranges: CoverageRangeWithMetadata[];
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

	public addCoverage(entries: inspector.Profiler.ScriptCoverage[]) {
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

	public generate(): CoverageFile[] {
		const insertedLocs: Map<string, CoverageLocationRange> = new Map();
		const locs: CoverageLocationRange[] = [];

		for (const data of this.sourceMaps.values()) {
			const {ranges, code, map} = data;

			// Turn an index into a position in the compiled source
			let line: OneIndexed = new OneIndexed();
			let column: ZeroIndexed = new ZeroIndexed();
			let index: ZeroIndexed = new ZeroIndexed();
			const indexCache: Map<ZeroIndexed, Position> = new Map();
			function findIndex(newIndex: ZeroIndexed): Position {
				const cached = indexCache.get(newIndex);
				if (cached !== undefined) {
					return cached;
				}

				if (newIndex < index) {
					throw new Error(`Expected newIndex(${newIndex}) >= index(${index})`);
				}

				if (newIndex.valueOf() > code.length) {
					throw new Error(
						`Expected newIndex(${newIndex}) <= code.length(${code.length})`,
					);
				}

				while (index < newIndex) {
					const char = code[index.valueOf()];
					if (char === "\n") {
						line = line.increment();
						column = new ZeroIndexed();
					} else {
						column = column.increment();
					}
					index = index.increment();
				}

				const pos: Position = {
					line,
					column,
				};
				indexCache.set(newIndex, pos);
				return pos;
			}

			// Prefetch all sorted indexes
			const offsets: ZeroIndexed[] = [];
			for (const {startOffset, endOffset} of ranges) {
				offsets.push(new ZeroIndexed(startOffset));
				offsets.push(new ZeroIndexed(endOffset));
			}
			offsets.sort((a, b) => a.valueOf() - b.valueOf());
			for (const index of offsets) {
				findIndex(index);
			}

			//
			for (const {kind, startOffset, endOffset, count} of ranges) {
				const originalStart = findIndex(new ZeroIndexed(startOffset));
				const originalEnd = findIndex(new ZeroIndexed(endOffset));

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
					path: sourceStart.source,
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
		const rangesByFile: MixedPathMap<CoverageLocationRange[]> = new MixedPathMap();
		for (const loc of locs) {
			let ranges = rangesByFile.get(loc.path);
			if (ranges === undefined) {
				ranges = [];
				rangesByFile.set(loc.path, ranges);
			}
			ranges.push(loc);
		}

		const files: CoverageFile[] = [];
		for (const [path, ranges] of rangesByFile) {
			const coveredLines: Set<OneIndexed> = new Set();
			const uncoveredLines: Set<OneIndexed> = new Set();

			let uncoveredFunctions: Set<OneIndexed> = new Set();
			let coveredFunctions: Set<OneIndexed> = new Set();
			let uncoveredBranches: Set<string> = new Set();
			let coveredBranches: Set<string> = new Set();

			for (const {count, kind, start, end} of ranges) {
				// Fill in lines
				for (let i = start.line; i <= end.line; i = i.increment()) {
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
				path,
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
