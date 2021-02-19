/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MappedKeyMap, MappedSet} from "@internal/collections";
import {
	AbsoluteFilePath,
	AnyPath,
	RelativePath,
	UIDPath,
	URLPath,
	createAbsoluteFilePath,
	createAnyPath,
	createRelativePath,
	createUIDPath,
	createURLPath,
} from "./index";

function concat<FilePath extends AnyPath>(
	items: Iterable<FilePath>[],
): FilePath[] {
	let paths: FilePath[] = [];
	for (const iterable of items) {
		paths = paths.concat(Array.from(iterable));
	}
	return paths;
}

// Sometimes we don't want to have to deal with what a FilePath serializes into
// For those purposes we have these wrappers around Map and Set. Here we can add some custom logic
// to speed up the usage of FilePaths in these scenarios.
// The API here attempts to match what is expected from the native classes, however we may deviate from it
// to avoid the usage of getters and generator/symbol indirection for iteration.
export abstract class BasePathMap<Path extends AnyPath, Value>
	extends MappedKeyMap<Path, string, Value> {
	constructor(entries?: [Path, Value][]) {
		super(
			(path) => {
				const uniq = path.getUnique();
				return [uniq.join(), uniq as Path];
			},
			entries,
		);
		this[Symbol.toStringTag] = "BasePathMap";
	}

	public abstract createKey(str: string): Path;

	public [Symbol.toStringTag]: string;

	public setString(path: string, value: Value) {
		this.set(this.createKey(path), value);
	}
}

export class AbsoluteFilePathMap<Value>
	extends BasePathMap<AbsoluteFilePath, Value> {
	constructor(entries?: [AbsoluteFilePath, Value][]) {
		super(entries);
		this[Symbol.toStringTag] = "AbsoluteFilePathMap";
	}

	public createKey(str: string): AbsoluteFilePath {
		return createAbsoluteFilePath(str);
	}

	public keysToSet(): AbsoluteFilePathSet {
		return new AbsoluteFilePathSet(this.keys());
	}
}

export class RelativePathMap<Value> extends BasePathMap<RelativePath, Value> {
	constructor(entries?: [RelativePath, Value][]) {
		super(entries);
		this[Symbol.toStringTag] = "RelativePathMap";
	}

	public createKey(str: string): RelativePath {
		return createRelativePath(str);
	}

	public keysToSet(): RelativePathSet {
		return new RelativePathSet(this.keys());
	}
}

export class URLPathMap<Value> extends BasePathMap<URLPath, Value> {
	constructor(entries?: [URLPath, Value][]) {
		super(entries);
		this[Symbol.toStringTag] = "URLPathMap";
	}

	public createKey(str: string): URLPath {
		return createURLPath(str);
	}

	public keysToSet(): URLPathSet {
		return new URLPathSet(this.keys());
	}
}

export class UIDPathMap<Value> extends BasePathMap<UIDPath, Value> {
	constructor(entries?: [UIDPath, Value][]) {
		super(entries);
		this[Symbol.toStringTag] = "UIDPathMap";
	}

	public createKey(str: string): UIDPath {
		return createUIDPath(str);
	}

	public keysToSet(): UIDPathSet {
		return new UIDPathSet(this.keys());
	}
}

export class MixedPathMap<Value> extends BasePathMap<AnyPath, Value> {
	constructor(entries?: [AnyPath, Value][]) {
		super(entries);
		this[Symbol.toStringTag] = "MixedPathMap";
	}

	public createKey(str: string): AnyPath {
		return createAnyPath(str);
	}

	public keysToSet(): MixedPathSet {
		return new MixedPathSet(this.keys());
	}
}

abstract class BasePathSet<
	Path extends AnyPath,
	PathMap extends BasePathMap<Path, Path> = BasePathMap<Path, Path>
> extends MappedSet<Path, string> {
	constructor(entries?: Iterable<Path>) {
		super(
			(path) => {
				const uniq = path.getUnique();
				return [path.join(), uniq as Path];
			},
			entries,
		);
	}

	abstract addString(str: string): void;

	public addSet(set: BasePathSet<Path, PathMap>): this {
		for (const path of set) {
			this.add(path);
		}
		return this;
	}
}

export class AbsoluteFilePathSet extends BasePathSet<AbsoluteFilePath> {
	constructor(entries?: Iterable<AbsoluteFilePath>) {
		super(entries);
		this[Symbol.toStringTag] = "AbsoluteFilePathMap";
	}

	public addString(str: string): void {
		this.add(createAbsoluteFilePath(str));
	}

	public concat(...items: Iterable<AbsoluteFilePath>[]): AbsoluteFilePathSet {
		return new AbsoluteFilePathSet(concat(items));
	}
}

export class RelativePathSet extends BasePathSet<RelativePath> {
	constructor(entries?: Iterable<RelativePath>) {
		super(entries);
		this[Symbol.toStringTag] = "RelativePathSet";
	}

	public addString(str: string): void {
		this.add(createRelativePath(str));
	}

	public concat(...items: Iterable<RelativePath>[]): RelativePathSet {
		return new RelativePathSet(concat(items));
	}
}

export class URLPathSet extends BasePathSet<URLPath> {
	constructor(entries?: Iterable<URLPath>) {
		super(entries);
		this[Symbol.toStringTag] = "URLPathSet";
	}

	public addString(str: string): void {
		this.add(createURLPath(str));
	}

	public concat(...items: Iterable<URLPath>[]): URLPathSet {
		return new URLPathSet(concat(items));
	}
}

export class UIDPathSet extends BasePathSet<UIDPath> {
	constructor(entries?: Iterable<UIDPath>) {
		super(entries);
		this[Symbol.toStringTag] = "UIDPathSet";
	}

	public addString(str: string): void {
		this.add(createUIDPath(str));
	}

	public concat(...items: Iterable<UIDPath>[]): UIDPathSet {
		return new UIDPathSet(concat(items));
	}
}

export class MixedPathSet extends BasePathSet<AnyPath> {
	constructor(entries?: Iterable<AnyPath>) {
		super(entries);
		this[Symbol.toStringTag] = "MixedPathSet";
	}

	public addString(str: string): void {
		this.add(createAnyPath(str));
	}

	public concat(...items: Iterable<AnyPath>[]): MixedPathSet {
		return new MixedPathSet(concat(items));
	}
}

export type PathSet =
	| AbsoluteFilePathSet
	| RelativePathSet
	| URLPathSet
	| UIDPathSet
	| MixedPathSet;

export function isPathSet(val: unknown): val is PathSet {
	return (
		val instanceof AbsoluteFilePathSet ||
		val instanceof RelativePathSet ||
		val instanceof URLPathSet ||
		val instanceof UIDPathSet ||
		val instanceof MixedPathSet
	);
}

export type PathMap<Value> =
	| AbsoluteFilePathMap<Value>
	| RelativePathMap<Value>
	| URLPathMap<Value>
	| UIDPathMap<Value>
	| MixedPathMap<Value>;

export function isPathMap(val: unknown): val is PathMap<unknown> {
	return (
		val instanceof AbsoluteFilePathMap ||
		val instanceof RelativePathMap ||
		val instanceof URLPathMap ||
		val instanceof UIDPathMap ||
		val instanceof MixedPathMap
	);
}

export type PathMapValue<T> = T extends BasePathMap<AnyPath, infer V>
	? V
	: never;
