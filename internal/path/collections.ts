/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MappedKeyMap, MappedSet} from "@internal/collections";
import {Path, ReadablePath} from "./types";
import AbsoluteFilePath from "./classes/AbsoluteFilePath";
import RelativePath from "./classes/RelativePath";
import UIDPath from "./classes/UIDPath";
import URLPath from "./classes/URLPath";
import DataURIPath from "./classes/DataURIPath";

// Sometimes we don't want to have to deal with what a FilePath serializes into
// For those purposes we have these wrappers around Map and Set. Here we can add some custom logic
// to speed up the usage of FilePaths in these scenarios.
// The API here attempts to match what is expected from the native classes, however we may deviate from it
// to avoid the usage of getters and generator/symbol indirection for iteration.
abstract class BasePathMap<PathT extends Path, Value>
	extends MappedKeyMap<PathT, string, Value> {
	constructor(entries?: [PathT, Value][]) {
		super(
			(path) => {
				const uniq = path.getUnique();
				return [uniq.join(), uniq as PathT];
			},
			entries,
		);
	}

	public abstract setValidated(key: PathT, value: Value): void;
}

export class ReadablePathMap<Value> extends BasePathMap<ReadablePath, Value> {
	public setValidated(key: Path, value: Value) {
		this.set(key.assertReadable(), value);
	}
}
ReadablePathMap.prototype[Symbol.toStringTag] = "ReadablePathMap";

export class AbsoluteFilePathMap<Value>
	extends BasePathMap<AbsoluteFilePath, Value> {
	public setValidated(key: Path, value: Value) {
		this.set(key.assertAbsolute(), value);
	}
}
AbsoluteFilePathMap.prototype[Symbol.toStringTag] = "AbsoluteFilePathMap";

export class RelativePathMap<Value> extends BasePathMap<RelativePath, Value> {
	public setValidated(key: Path, value: Value) {
		this.set(key.assertRelative(), value);
	}
}
RelativePathMap.prototype[Symbol.toStringTag] = "RelativePathMap";

export class URLPathMap<Value> extends BasePathMap<URLPath, Value> {
	public setValidated(key: Path, value: Value) {
		this.set(key.assertURL(), value);
	}
}
URLPathMap.prototype[Symbol.toStringTag] = "URLPathMap";

export class DataURIPathMap<Value> extends BasePathMap<DataURIPath, Value> {
	public setValidated(key: Path, value: Value) {
		this.set(key.assertDataURI(), value);
	}
}
DataURIPathMap.prototype[Symbol.toStringTag] = "DataURIPathMap";

export class UIDPathMap<Value> extends BasePathMap<UIDPath, Value> {
	public setValidated(key: Path, value: Value) {
		this.set(key.assertUID(), value);
	}
}
UIDPathMap.prototype[Symbol.toStringTag] = "UIDPathMap";

export class MixedPathMap<Value> extends BasePathMap<Path, Value> {
	public setValidated(key: Path, value: Value) {
		this.set(key, value);
	}
}
MixedPathMap.prototype[Symbol.toStringTag] = "MixedPathMap";

abstract class BasePathSet<
	PathT extends Path,
	PathSet extends BasePathSet<PathT, PathSet>
> extends MappedSet<PathT, string> {
	constructor(entries?: Iterable<PathT>) {
		super(
			(path) => {
				const uniq = path.getUnique();
				return [path.join(), uniq as PathT];
			},
			entries,
		);
	}

	public abstract addValidated(path: PathT): void;
}

export class ReadablePathSet extends BasePathSet<ReadablePath, ReadablePathSet> {
	public addValidated(path: Path) {
		this.add(path.assertReadable());
	}
}
ReadablePathSet.prototype[Symbol.toStringTag] = "ReadablePathSet";

export class AbsoluteFilePathSet
	extends BasePathSet<AbsoluteFilePath, AbsoluteFilePathSet> {
	public addValidated(path: Path) {
		this.add(path.assertAbsolute());
	}
}
AbsoluteFilePathSet.prototype[Symbol.toStringTag] = "AbsoluteFilePathSet";

export class RelativePathSet extends BasePathSet<RelativePath, RelativePathSet> {
	public addValidated(path: Path) {
		this.add(path.assertRelative());
	}
}
RelativePathSet.prototype[Symbol.toStringTag] = "RelativePathSet";

export class URLPathSet extends BasePathSet<URLPath, URLPathSet> {
	public addValidated(path: Path) {
		this.add(path.assertURL());
	}
}
URLPathSet.prototype[Symbol.toStringTag] = "URLPathSet";

export class DataURIPathSet extends BasePathSet<DataURIPath, DataURIPathSet> {
	public addValidated(path: Path) {
		this.add(path.assertDataURI());
	}
}
DataURIPathSet.prototype[Symbol.toStringTag] = "DataURIPathSet";

export class UIDPathSet extends BasePathSet<UIDPath, UIDPathSet> {
	public addValidated(path: Path) {
		this.add(path.assertUID());
	}
}
UIDPathSet.prototype[Symbol.toStringTag] = "UIDPathSet";

export class MixedPathSet extends BasePathSet<Path, MixedPathSet> {
	public addValidated(path: Path) {
		this.add(path);
	}
}
MixedPathSet.prototype[Symbol.toStringTag] = "MixedPathSet";

export type PathSet =
	| AbsoluteFilePathSet
	| RelativePathSet
	| URLPathSet
	| UIDPathSet
	| MixedPathSet
	| DataURIPathSet;

export function isPathSet(val: unknown): val is PathSet {
	return (
		val instanceof AbsoluteFilePathSet ||
		val instanceof RelativePathSet ||
		val instanceof URLPathSet ||
		val instanceof UIDPathSet ||
		val instanceof MixedPathSet ||
		val instanceof DataURIPathSet
	);
}

export type PathMap<Value> =
	| AbsoluteFilePathMap<Value>
	| RelativePathMap<Value>
	| URLPathMap<Value>
	| UIDPathMap<Value>
	| MixedPathMap<Value>
	| DataURIPathMap<Value>;

export function isPathMap(val: unknown): val is PathMap<unknown> {
	return (
		val instanceof AbsoluteFilePathMap ||
		val instanceof RelativePathMap ||
		val instanceof URLPathMap ||
		val instanceof UIDPathMap ||
		val instanceof MixedPathMap ||
		val instanceof DataURIPathMap
	);
}

export type PathMapValue<T> = T extends BasePathMap<Path, infer V> ? V : never;
