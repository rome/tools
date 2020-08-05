/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import SourceMapConsumer from "./SourceMapConsumer";
import {Number0, Number1} from "@internal/ob1";
import {ResolvedLocation} from "./types";

export default class SourceMapConsumerCollection {
	constructor() {
		this.maps = new Map();
	}

	private maps: Map<string, SourceMapConsumer>;

	public hasAny(): boolean {
		return this.maps.size > 0;
	}

	public has(file: undefined | string): boolean {
		return file !== undefined && this.maps.has(file);
	}

	public add(file: string, map: SourceMapConsumer) {
		this.maps.set(file, map);
	}

	private get(file: string): undefined | SourceMapConsumer {
		return this.maps.get(file);
	}

	private normalizeResolved(
		source: string,
		line: Number1,
		column: Number0,
		loc: undefined | ResolvedLocation,
	): ResolvedLocation {
		if (loc === undefined) {
			return {
				found: false,
				source,
				line,
				column,
				name: undefined,
			};
		} else {
			return loc;
		}
	}

	public assertApproxOriginalPositionFor(
		file: string,
		line: Number1,
		column: Number0,
	): ResolvedLocation {
		return this.normalizeResolved(
			file,
			line,
			column,
			this.approxOriginalPositionFor(file, line, column),
		);
	}

	public assertExactOriginalPositionFor(
		file: string,
		line: Number1,
		column: Number0,
	): ResolvedLocation {
		return this.normalizeResolved(
			file,
			line,
			column,
			this.exactOriginalPositionFor(file, line, column),
		);
	}

	public approxOriginalPositionFor(
		file: string,
		line: Number1,
		column: Number0,
	): undefined | ResolvedLocation {
		const map = this.get(file);
		if (map === undefined) {
			return undefined;
		} else {
			return map.approxOriginalPositionFor(line, column);
		}
	}

	public exactOriginalPositionFor(
		file: string,
		line: Number1,
		column: Number0,
	): undefined | ResolvedLocation {
		const map = this.get(file);
		if (map === undefined) {
			return undefined;
		} else {
			return map.exactOriginalPositionFor(line, column);
		}
	}
}
