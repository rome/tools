/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ParsedMapping,
	ParsedMappings,
	ResolvedLocation,
	SourceMap,
} from "./types";
import {decodeVLQ} from "./base64";
import {OneIndexed, ZeroIndexed} from "@internal/numbers";
import {Dict} from "@internal/typescript-helpers";
import {Path, createPath} from "@internal/path";

export function getParsedMappingKey(
	line: OneIndexed,
	column: ZeroIndexed,
): string {
	return `${String(line.valueOf())}:${String(column.valueOf())}`;
}

type GetMappings = () => ParsedMappings;

export default class SourceMapConsumer {
	constructor(path: Path, getMappings: GetMappings) {
		this.path = path;
		this._getMappings = getMappings;
		this.mappings = undefined;
	}

	private path: Path;
	private _getMappings: GetMappings;
	private mappings: undefined | ParsedMappings;

	private static charIsMappingSeparator(str: string, index: number): boolean {
		const c = str.charAt(index);
		return c === ";" || c === ",";
	}

	public static fromJSON(sourceMap: SourceMap): SourceMapConsumer {
		return new SourceMapConsumer(
			createPath(sourceMap.file),
			() => SourceMapConsumer.parseMappings(sourceMap),
		);
	}

	public static fromJSONLazy(
		path: Path,
		getSourceMap: () => SourceMap,
	): SourceMapConsumer {
		return new SourceMapConsumer(
			path,
			() => SourceMapConsumer.parseMappings(getSourceMap()),
		);
	}

	private static parseMappings(sourceMap: SourceMap): ParsedMappings {
		const rawStr: string = sourceMap.mappings;
		const map: ParsedMappings = new Map();

		let generatedLine = new OneIndexed();
		let previousGeneratedColumn = new ZeroIndexed();
		let previousOriginalLine = new OneIndexed();
		let previousOriginalColumn = new ZeroIndexed();
		let previousSource = 0;
		let previousName = 0;
		let length = rawStr.length;
		let index: number = 0;
		let cachedSegments: Dict<number[]> = {};
		let value;

		const sources: Path[] = sourceMap.sources.map((source) => {
			return createPath(source);
		});

		while (index < length) {
			const char = rawStr[index];
			if (char === ";") {
				generatedLine = generatedLine.increment();
				index++;
				previousGeneratedColumn = new ZeroIndexed();
			} else if (char === ",") {
				index++;
			} else {
				const mapping: ParsedMapping = {
					generated: {
						line: generatedLine,
						column: new ZeroIndexed(),
					},
					original: {
						line: new OneIndexed(),
						column: new ZeroIndexed(),
					},
					source: undefined,
					name: undefined,
				};

				// Because each offset is encoded relative to the previous one,
				// many segments often have the same encoding. We can exploit this
				// fact by caching the parsed variable length fields of each segment,
				// allowing us to avoid a second parse if we encounter the same
				// segment again.
				let end = index;
				for (; end < length; end++) {
					if (SourceMapConsumer.charIsMappingSeparator(rawStr, end)) {
						break;
					}
				}
				const str = rawStr.slice(index, end);

				let segment = cachedSegments[str];
				if (segment) {
					index += str.length;
				} else {
					segment = [];
					while (index < end) {
						[value, index] = decodeVLQ(rawStr, index);
						segment.push(value);
					}

					if (segment.length === 2) {
						throw new Error("Found a source, but no line and column");
					}

					if (segment.length === 3) {
						throw new Error("Found a source and line, but no column");
					}

					cachedSegments[str] = segment;
				}

				// Generated column
				mapping.generated.column = previousGeneratedColumn.add(segment[0]);
				previousGeneratedColumn = mapping.generated.column;

				if (segment.length > 1) {
					// Original source
					mapping.source = sources[previousSource + segment[1]];
					previousSource += segment[1];

					// Original line
					const newOriginalLine = previousOriginalLine.add(segment[2]);
					previousOriginalLine = newOriginalLine;

					// Lines are stored 0-based
					mapping.original.line = newOriginalLine.increment();

					// Original column
					const newOriginalColumn = previousOriginalColumn.add(segment[3]);
					mapping.original.column = newOriginalColumn;
					previousOriginalColumn = newOriginalColumn;

					if (segment.length > 4) {
						// Original name
						mapping.name = sourceMap.names[previousName + segment[4]];
						previousName += segment[4];
					}
				}

				map.set(
					getParsedMappingKey(mapping.generated.line, mapping.generated.column),
					mapping,
				);
			}
		}

		return map;
	}

	public clearCache() {
		this.mappings = undefined;
	}

	private getMappings(): ParsedMappings {
		if (this.mappings === undefined) {
			const mappings = this._getMappings();
			this.mappings = mappings;
			return mappings;
		} else {
			return this.mappings;
		}
	}

	public approxOriginalPositionFor(
		line: OneIndexed,
		column: ZeroIndexed,
	): undefined | ResolvedLocation {
		while (column.valueOf() >= 0) {
			const mapping = this.exactOriginalPositionFor(line, column);
			if (mapping === undefined) {
				column = column.decrement();
			} else {
				return mapping;
			}
		}

		return undefined;
	}

	public exactOriginalPositionFor(
		line: OneIndexed,
		column: ZeroIndexed,
	): undefined | ResolvedLocation {
		const key = getParsedMappingKey(line, column);
		const mapping = this.getMappings().get(key);
		if (mapping === undefined) {
			return undefined;
		}

		const source = mapping.source ?? this.path;
		if (source === undefined) {
			throw new Error("Mapping provided unknown source");
		}

		return {
			found: true,
			source,
			line: mapping.original.line,
			column: mapping.original.column,
			name: mapping.name,
		};
	}
}
