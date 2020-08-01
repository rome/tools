/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AbsoluteFilePath, RelativeFilePath, UnknownFilePath} from ".";

// Sometimes we don't want to have to deal with what a FilePath serializes into
// For those purposes we have these wrappers around Map and Set. Here we can add some custom logic
// to speed up the usage of FilePaths in these scenarios.
// The API here attempts to match what is expected from the native classes, however we may deviate from it
// to avoid the usage of getters and generator/symbol indirection for iteration.
class FilePathMap<FilePath extends UnknownFilePath, Value> {
	constructor(entries?: Array<[FilePath, Value]>) {
		this.joinedToValue = new Map();
		this.joinedToPath = new Map();
		this.size = 0;

		if (entries !== undefined) {
			for (const [key, value] of entries) {
				this.set(key, value);
			}
		}
	}

	joinedToValue: Map<string, Value>;
	joinedToPath: Map<string, FilePath>;
	size: number;

	_updateSize() {
		this.size = this.joinedToValue.size;
	}

	*[Symbol.iterator](): Iterator<[FilePath, Value]> {
		for (const [joined, value] of this.joinedToValue) {
			const path = this.joinedToPath.get(joined)!;
			yield [path, value];
		}
	}

	clear() {
		this.joinedToValue.clear();
		this.joinedToPath.clear();
		this._updateSize();
	}

	keys(): Iterable<FilePath> {
		return this.joinedToPath.values();
	}

	values(): Iterable<Value> {
		return this.joinedToValue.values();
	}

	delete(path: FilePath) {
		const joined = path.getUnique().join();
		this.joinedToValue.delete(joined);
		this.joinedToPath.delete(joined);
		this._updateSize();
	}

	has(path: FilePath): boolean {
		return this.joinedToValue.has(path.getUnique().join());
	}

	get(path: FilePath): undefined | Value {
		return this.joinedToValue.get(path.getUnique().join());
	}

	set(path: FilePath, value: Value) {
		const uniq = (path.getUnique() as FilePath);
		const joined = uniq.join();
		this.joinedToValue.set(joined, value);
		this.joinedToPath.set(joined, uniq);
		this._updateSize();
	}
}

class FilePathSet<FilePath extends UnknownFilePath> {
	constructor(entries?: Iterable<FilePath>) {
		this.map = new FilePathMap();
		this.size = 0;

		if (entries !== undefined) {
			for (const path of entries) {
				this.add(path);
			}
		}
	}

	map: FilePathMap<FilePath, void>;
	size: number;

	_updateSize() {
		this.size = this.map.size;
	}

	[Symbol.iterator](): Iterator<FilePath> {
		return this.map.keys()[Symbol.iterator]();
	}

	toJoined(): Array<string> {
		return Array.from(this.map.joinedToPath.keys());
	}

	has(path: FilePath) {
		return this.map.has(path);
	}

	add(path: FilePath) {
		this.map.set(path);
		this._updateSize();
	}

	delete(path: FilePath) {
		this.map.delete(path);
		this._updateSize();
	}

	clear() {
		this.map.clear();
		this._updateSize();
	}
}

export class AbsoluteFilePathMap<Value>
	extends FilePathMap<AbsoluteFilePath, Value> {
	type: "absolute" = "absolute";
}

export class RelativeFilePathMap<Value>
	extends FilePathMap<RelativeFilePath, Value> {
	type: "relative" = "relative";
}

export class UnknownFilePathMap<Value>
	extends FilePathMap<UnknownFilePath, Value> {
	type: "unknown" = "unknown";
}

export class AbsoluteFilePathSet extends FilePathSet<AbsoluteFilePath> {
	type: "absolute" = "absolute";
}

export class RelativeFilePathSet extends FilePathSet<RelativeFilePath> {
	type: "relative" = "relative";
}

export class UnknownFilePathSet extends FilePathSet<UnknownFilePath> {
	type: "unknown" = "unknown";
}
