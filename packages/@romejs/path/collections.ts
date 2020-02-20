/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AbsoluteFilePath, UnknownFilePath, RelativeFilePath} from '.';

// Sometimes we don't want to have to deal with what a FilePath serializes into
// For those purposes we have these wrappers around Map and Set. Here we can add some custom logic
// to speed up the usage of FilePaths in these scenarios.

// The API here attempts to match what is expected from the native classes, however we may deviate from it
// to avoid the usage of getters and generator/symbol indirection for iteration.

class FilePathMap<FilePath extends UnknownFilePath, Value> {
  constructor(entries?: Array<[FilePath, Value]>) {
    this.joinedToValue = new Map();
    this.joinedToPath = new Map();

    if (entries !== undefined) {
      for (const [key, value] of entries) {
        this.set(key, value);
      }
    }
  }

  joinedToValue: Map<string, Value>;
  joinedToPath: Map<string, FilePath>;

  get size(): number {
    return this.joinedToValue.size;
  }

  *[Symbol.iterator](): Iterator<[FilePath, Value]> {
    for (const [joined, value] of this.joinedToValue) {
      const path = this.joinedToPath.get(joined);
      if (path === undefined) {
        throw new Error('Impossible');
      }

      yield [path, value];
    }
  }

  clear() {
    this.joinedToValue.clear();
    this.joinedToPath.clear();
  }

  keys(): Iterable<FilePath> {
    return this.joinedToPath.values();
  }

  values(): Iterable<Value> {
    return this.joinedToValue.values();
  }

  delete(path: FilePath) {
    const joined = path.join();
    this.joinedToValue.delete(joined);
    this.joinedToPath.delete(joined);
  }

  has(path: FilePath): boolean {
    return this.joinedToValue.has(path.join());
  }

  get(path: FilePath): undefined | Value {
    return this.joinedToValue.get(path.join());
  }

  set(path: FilePath, value: Value) {
    const joined = path.join();
    this.joinedToValue.set(joined, value);
    this.joinedToPath.set(joined, path);
  }
}

class FilePathSet<FilePath extends UnknownFilePath> {
  constructor(entries?: Array<FilePath>) {
    this.map = new FilePathMap();

    if (entries !== undefined) {
      for (const path of entries) {
        this.add(path);
      }
    }
  }

  map: FilePathMap<FilePath, void>;

  [Symbol.iterator](): Iterator<FilePath> {
    return this.map.keys()[Symbol.iterator]();
  }

  has(path: FilePath) {
    return this.map.has(path);
  }

  add(path: FilePath) {
    this.map.set(path);
  }

  delete(path: FilePath) {
    this.map.delete(path);
  }

  clear() {
    this.map.clear();
  }
}

export class AbsoluteFilePathMap<Value> extends FilePathMap<
  AbsoluteFilePath,
  Value
> {}

export class RelativeFilePathMap<Value> extends FilePathMap<
  RelativeFilePath,
  Value
> {}

export class UnknownFilePathMap<Value> extends FilePathMap<
  UnknownFilePath,
  Value
> {}

export class AbsoluteFilePathSet extends FilePathSet<AbsoluteFilePath> {}

export class RelativeFilePathSet extends FilePathSet<RelativeFilePath> {}

export class UnknownFilePathSet extends FilePathSet<UnknownFilePath> {}
