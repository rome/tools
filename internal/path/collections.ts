/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AbsoluteFilePath,
	AnyFilePath,
	RelativeFilePath,
	UnknownPath,
	createAbsoluteFilePath,
	createRelativeFilePath,
	createUnknownPath,
} from "./index";

// Sometimes we don't want to have to deal with what a FilePath serializes into
// For those purposes we have these wrappers around Map and Set. Here we can add some custom logic
// to speed up the usage of FilePaths in these scenarios.
// The API here attempts to match what is expected from the native classes, however we may deviate from it
// to avoid the usage of getters and generator/symbol indirection for iteration.
abstract class BaseFilePathMap<FilePath extends AnyFilePath, Value> {
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

	public abstract createKey(str: string): FilePath

	private joinedToValue: Map<string, Value>;
	public joinedToPath: Map<string, FilePath>;
	public size: number;

	public _updateSize() {
		this.size = this.joinedToValue.size;
	}

	public *[Symbol.iterator](): Iterator<[FilePath, Value]> {
		for (const [joined, value] of this.joinedToValue) {
			const path = this.joinedToPath.get(joined)!;
			yield [path, value];
		}
	}

	public clear() {
		this.joinedToValue.clear();
		this.joinedToPath.clear();
		this._updateSize();
	}

	public keys(): Iterable<FilePath> {
		return this.joinedToPath.values();
	}

	public values(): Iterable<Value> {
		return this.joinedToValue.values();
	}

	public delete(path: FilePath) {
		const joined = path.getUnique().join();
		this.joinedToValue.delete(joined);
		this.joinedToPath.delete(joined);
		this._updateSize();
	}

	public has(path: FilePath): boolean {
		return this.joinedToValue.has(path.getUnique().join());
	}

	public get(path: FilePath): undefined | Value {
		return this.joinedToValue.get(path.getUnique().join());
	}

	public setString(path: string, value: Value) {
		this.set(this.createKey(path), value);
	}

	public set(path: FilePath, value: Value) {
		const uniq = (path.getUnique() as FilePath);
		const joined = uniq.join();
		this.joinedToValue.set(joined, value);
		this.joinedToPath.set(joined, uniq);
		this._updateSize();
	}
}

abstract class BaseFilePathSet<
	FilePath extends AnyFilePath,
	FilePathMap extends BaseFilePathMap<FilePath, void>
> {
	constructor(entries?: Iterable<FilePath>) {
		this.map = this.createMap();
		this.size = 0;

		if (entries !== undefined) {
			for (const path of entries) {
				this.add(path);
			}
		}
	}

	abstract createMap(): FilePathMap

	private map: FilePathMap;
	public size: number;

	public createKey(str: string): FilePath {
		return this.map.createKey(str);
	}

	public _updateSize() {
		this.size = this.map.size;
	}

	public [Symbol.iterator](): Iterator<FilePath> {
		return this.map.keys()[Symbol.iterator]();
	}

	public toJoined(
		callback: (path: string) => string = (filename) => filename,
	): Array<string> {
		return Array.from(this.map.joinedToPath.keys(), callback);
	}

	public has(path: FilePath) {
		return this.map.has(path);
	}

	public add(path: FilePath) {
		this.map.set(path);
		this._updateSize();
	}

	public addString(str: string) {
		this.add(this.createKey(str));
	}

	public delete(path: FilePath) {
		this.map.delete(path);
		this._updateSize();
	}

	public clear() {
		this.map.clear();
		this._updateSize();
	}
}

export class AbsoluteFilePathMap<Value>
	extends BaseFilePathMap<AbsoluteFilePath, Value> {
	public type: "absolute" = "absolute";

	public createKey(str: string): AbsoluteFilePath {
		return createAbsoluteFilePath(str);
	}

	public keysToSet(): AbsoluteFilePathSet {
		return new AbsoluteFilePathSet(this.keys());
	}
}

export class RelativeFilePathMap<Value>
	extends BaseFilePathMap<RelativeFilePath, Value> {
	public type: "relative" = "relative";

	public createKey(str: string): RelativeFilePath {
		return createRelativeFilePath(str);
	}

	public keysToSet(): RelativeFilePathSet {
		return new RelativeFilePathSet(this.keys());
	}
}

export class UnknownPathMap<Value>
	extends BaseFilePathMap<AnyFilePath, Value> {
	public type: "unknown" = "unknown";

	public createKey(str: string): UnknownPath {
		return createUnknownPath(str);
	}

	public keysToSet(): UnknownPathSet {
		return new UnknownPathSet(this.keys());
	}
}

export class AbsoluteFilePathSet
	extends BaseFilePathSet<AbsoluteFilePath, AbsoluteFilePathMap<void>> {
	public type: "absolute" = "absolute";

	createMap(): AbsoluteFilePathMap<void> {
		return new AbsoluteFilePathMap();
	}
}

export class RelativeFilePathSet
	extends BaseFilePathSet<RelativeFilePath, RelativeFilePathMap<void>> {
	public type: "relative" = "relative";

	createMap(): RelativeFilePathMap<void> {
		return new RelativeFilePathMap();
	}
}

export class UnknownPathSet
	extends BaseFilePathSet<AnyFilePath, UnknownPathMap<void>> {
	public type: "unknown" = "unknown";

	createMap(): UnknownPathMap<void> {
		return new UnknownPathMap();
	}
}

export type AnyFilePathSet =
	| AbsoluteFilePathSet
	| RelativeFilePathSet
	| UnknownPathSet;
