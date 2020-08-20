/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/*
 * Copyright 2011 Mozilla Foundation and contributors
 * Licensed under the New BSD license. See LICENSE or:
 * http://opensource.org/licenses/BSD-3-Clause
 */

import {Mapping, Mappings, ParsedMappings, SourceMap} from "./types";
import * as base64 from "./base64";
import {compareByGeneratedPositionsInflated, toRelativeUrl} from "./util";
import ArraySet from "./ArraySet";
import MappingList from "./MappingList";
import {
	Number0,
	Number1,
	ob1Get0,
	ob1Get1,
	ob1Inc,
	ob1Number0,
	ob1Number1,
} from "@internal/ob1";
import SourceMapConsumer, {getParsedMappingKey} from "./SourceMapConsumer";
import {VoidCallback} from "@internal/typescript-helpers";
import {ExtendedMap} from "@internal/collections";

type MaterializeCallback = VoidCallback;

export default class SourceMapGenerator {
	constructor(
		args: {
			file: string;
			sourceRoot?: string;
		},
	) {
		this.file = args.file;
		this.sourceRoot = args.sourceRoot;

		this.sourcesContents = new ExtendedMap("sourcesContents");
		this.map = undefined;
		this.sources = new ArraySet();
		this.names = new ArraySet();
		this.mappings = new MappingList();
		this.materializeCallbacks = [];
	}

	public file: string;
	private materializeCallbacks: Array<MaterializeCallback>;
	private sourceRoot: undefined | string;
	private sources: ArraySet;
	private names: ArraySet;
	private mappings: MappingList;
	private sourcesContents: ExtendedMap<string, string>;
	private map: undefined | SourceMap;

	private assertUnlocked() {
		if (this.map !== undefined) {
			throw new Error(
				"Source map has already been materialized, serialize() should be your final call",
			);
		}
	}

	public addMaterializer(fn: MaterializeCallback) {
		this.materializeCallbacks.push(fn);
	}

	/**
   * Add a single mapping from 'original source line and column to the generated
   * source's line and column for this source map being created. The mapping
   * object should have the following properties:
   *
   *   - generated: An object with the generated line and column positions.
   *   - original: An object with the original line and column positions.
   *   - source: The original source file (relative to the sourceRoot).
   *   - name: An optional original token name for this mapping.
   */
	public addMapping(mapping: Mapping): void {
		this.assertUnlocked();

		const {name, source} = mapping;

		this.validatePosition(
			"generated",
			mapping.generated.line,
			mapping.generated.column,
		);

		if (mapping.original) {
			this.validatePosition(
				"original",
				mapping.original.line,
				mapping.original.column,
			);
		}

		if (source !== undefined) {
			this.sources.add(source);
		}

		if (name !== undefined) {
			this.names.add(name);
		}

		this.mappings.add(mapping);
	}

	/**
   * Set the source content for a source file.
   */
	public setSourceContent(
		source: string,
		sourceContent: undefined | string,
	): void {
		this.assertUnlocked();

		if (this.sourceRoot !== undefined) {
			source = toRelativeUrl(this.sourceRoot, source);
		}

		if (sourceContent !== undefined) {
			// Add the source content to the _sourcesContents map.
			this.sourcesContents.set(source, sourceContent);
		} else {
			// Remove the source file from the _sourcesContents map.
			this.sourcesContents.delete(source);
		}
	}

	private validatePosition(key: string, line: Number1, column: Number0): void {
		if (ob1Get1(line) <= 0) {
			throw new Error(`${key} line should be >= 1 but is ${line}`);
		}

		if (ob1Get0(column) < 0) {
			throw new Error(`${key} column should be >= 0 but is ${column}`);
		}
	}

	/**
   * Serialize the accumulated mappings in to the stream of base 64 VLQs
   * specified by the source map format.
   */
	private serializeMappings(): string {
		let previousGeneratedColumn: Number0 = ob1Number0;
		let previousGeneratedLine: Number1 = ob1Number1;
		let previousOriginalColumn: Number0 = ob1Number0;
		let previousOriginalLine: Number1 = ob1Number1;
		let previousName: number = 0;
		let previousSource: number = 0;
		let result: string = "";

		const mappings = this.mappings.toArray();
		for (let i = 0; i < mappings.length; i++) {
			const mapping = mappings[i];
			let next = "";

			if (mapping.generated.line !== previousGeneratedLine) {
				previousGeneratedColumn = ob1Number0;
				while (mapping.generated.line !== previousGeneratedLine) {
					next += ";";
					previousGeneratedLine = ob1Inc(previousGeneratedLine);
				}
			} else if (i > 0) {
				if (!compareByGeneratedPositionsInflated(mapping, mappings[i - 1])) {
					continue;
				}
				next += ",";
			}

			next += base64.encodeVLQ(
				ob1Get0(mapping.generated.column) - ob1Get0(previousGeneratedColumn),
			);
			previousGeneratedColumn = mapping.generated.column;

			if (mapping.source !== undefined) {
				const sourceIdx = this.sources.indexOf(mapping.source);
				next += base64.encodeVLQ(sourceIdx - previousSource);
				previousSource = sourceIdx;

				if (mapping.original) {
					next += base64.encodeVLQ(
						ob1Get1(mapping.original.line) - ob1Get1(previousOriginalLine),
					);
					previousOriginalLine = mapping.original.line;

					next += base64.encodeVLQ(
						ob1Get0(mapping.original.column) - ob1Get0(previousOriginalColumn),
					);
					previousOriginalColumn = mapping.original.column;

					if (mapping.name !== undefined) {
						const nameIdx = this.names.indexOf(mapping.name);
						next += base64.encodeVLQ(nameIdx - previousName);
						previousName = nameIdx;
					}
				}

				// TODO: else, assert mapping.name is undefined since it can't be encoded without an original position
			}

			// TODO: else, assert mapping.original is undefined since it can't be encoded without a source
			result += next;
		}

		return result;
	}

	private generateSourcesContent(
		sources: Array<string>,
		sourceRoot: undefined | string,
	): Array<string> {
		return sources.map((source) => {
			if (sourceRoot !== undefined) {
				source = toRelativeUrl(sourceRoot, source);
			}
			return this.sourcesContents.assert(source);
		});
	}

	private materialize() {
		for (const fn of this.materializeCallbacks) {
			fn();
		}
		this.materializeCallbacks = [];
	}

	/**
   * Externalize the source map.
   */
	public serialize(): SourceMap {
		if (this.map !== undefined) {
			return this.map;
		}

		this.materialize();

		const sources = this.sources.toArray();
		this.map = {
			version: 3,
			file: this.file,
			names: this.names.toArray(),
			mappings: this.serializeMappings(),
			sourceRoot: this.sourceRoot,
			sources,
			sourcesContent: this.generateSourcesContent(sources, this.sourceRoot),
		};
		return this.map;
	}

	public toComment(): string {
		const jsonMap = this.toJSON();
		const base64Map = Buffer.from(jsonMap).toString("base64");
		const comment = `//# sourceMappingURL=data:application/json;charset=utf-8;base64,${base64Map}`;
		return comment;
	}

	public toConsumer(): SourceMapConsumer {
		return new SourceMapConsumer(
			this.file,
			() => {
				const parsedMappings: ParsedMappings = new Map();

				for (const mapping of this.getMappings()) {
					parsedMappings.set(
						getParsedMappingKey(
							mapping.generated.line,
							mapping.generated.column,
						),
						mapping,
					);
				}

				return parsedMappings;
			},
		);
	}

	private getMappings(): Mappings {
		this.materialize();
		return this.mappings.toArray();
	}

	public toJSON(): string {
		return JSON.stringify(this.serialize());
	}
}
