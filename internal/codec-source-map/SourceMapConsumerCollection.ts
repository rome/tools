/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import SourceMapConsumer from "./SourceMapConsumer";
import {OneIndexed, ZeroIndexed} from "@internal/numbers";
import {ResolvedLocation} from "./types";
import {MixedPathMap, Path} from "@internal/path";

export default class SourceMapConsumerCollection {
	constructor() {
		this.maps = new MixedPathMap();
	}

	private maps: MixedPathMap<SourceMapConsumer>;

	public hasAny(): boolean {
		return this.maps.size > 0;
	}

	public has(path: undefined | Path): boolean {
		return path !== undefined && this.maps.has(path);
	}

	public add(path: Path, map: SourceMapConsumer) {
		this.maps.set(path, map);
	}

	private get(path: Path): undefined | SourceMapConsumer {
		return this.maps.get(path);
	}

	private normalizeResolved(
		source: Path,
		line: OneIndexed,
		column: ZeroIndexed,
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
		path: Path,
		line: OneIndexed,
		column: ZeroIndexed,
	): ResolvedLocation {
		return this.normalizeResolved(
			path,
			line,
			column,
			this.approxOriginalPositionFor(path, line, column),
		);
	}

	public assertExactOriginalPositionFor(
		path: Path,
		line: OneIndexed,
		column: ZeroIndexed,
	): ResolvedLocation {
		return this.normalizeResolved(
			path,
			line,
			column,
			this.exactOriginalPositionFor(path, line, column),
		);
	}

	public approxOriginalPositionFor(
		path: Path,
		line: OneIndexed,
		column: ZeroIndexed,
	): undefined | ResolvedLocation {
		const map = this.get(path);
		if (map === undefined) {
			return undefined;
		} else {
			return map.approxOriginalPositionFor(line, column);
		}
	}

	public exactOriginalPositionFor(
		path: Path,
		line: OneIndexed,
		column: ZeroIndexed,
	): undefined | ResolvedLocation {
		const map = this.get(path);
		if (map === undefined) {
			return undefined;
		} else {
			return map.exactOriginalPositionFor(line, column);
		}
	}
}
