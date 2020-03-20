/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AbsoluteFilePath, AbsoluteFilePathSet} from '@romejs/path';

import fs = require('fs');

// This file contains some wrappers around Node's fs module. Except here we support passing in AbsoluteFilePath instances.

// NOTE We don't bother using Node's built-in fs promise functions at all. They already contain a level of indirection to callbacks.

// Helpers
type DataCallback<Data> = (err: null | Error, data: Data) => void;

function promisifyData<Data>(
  path: AbsoluteFilePath,
  factory: (path: string, callback: DataCallback<Data>) => void,
): Promise<Data> {
  return new Promise((resolve, reject) => {
    factory(path.join(), (err, data) => {
      if (err === null) {
        resolve(data);
      } else {
        reject(err);
      }
    });
  });
}

type VoidCallback = (err: null | Error) => void;

function promisifyVoid(
  path: AbsoluteFilePath,
  factory: (path: string, callback: VoidCallback) => void,
): Promise<void> {
  return new Promise((resolve, reject) => {
    factory(path.join(), (err) => {
      if (err === null) {
        resolve();
      } else {
        reject(err);
      }
    });
  });
}

// watch
export function watch(
  path: AbsoluteFilePath,
  options: 
      | {
        encoding?: BufferEncoding | null;
        persistent?: boolean;
        recursive?: boolean;
      }
      | undefined,

  listener?: (event: string, filename: null | string) => void,
) {
  return fs.watch(path.join(), options, listener);
}

// readFile
export function readFile(path: AbsoluteFilePath): Promise<Buffer> {
  return promisifyData(path, (filename, callback) => 
    fs.readFile(filename, callback)
  );
}

export function readFileSync(path: AbsoluteFilePath): Buffer {
  return fs.readFileSync(path.join());
}

// readFileText
export async function readFileText(path: AbsoluteFilePath): Promise<string> {
  return (await readFile(path)).toString();
}

export function readFileTextSync(path: AbsoluteFilePath): string {
  return fs.readFileSync(path.join(), 'utf8');
}

// writeFile
export function writeFile(
  path: AbsoluteFilePath,
  content: string | Buffer,
): Promise<void> {
  return promisifyVoid(path, (filename, callback) => 
    fs.writeFile(filename, content, callback)
  );
}

export function writeFileSync(
  path: AbsoluteFilePath,
  content: Buffer | string,
): void {
  return fs.writeFileSync(path.join(), content);
}

// readdir
function createReaddirReturn(
  folder: AbsoluteFilePath,
  files: Array<string>,
): AbsoluteFilePathSet {
  return new AbsoluteFilePathSet(files.map((basename) => {
    return folder.append(basename);
  }));
}

export function readdir(path: AbsoluteFilePath): Promise<AbsoluteFilePathSet> {
  return new Promise((resolve, reject) => {
    fs.readdir(path.join(), (err, files) => {
      if (err === null) {
        resolve(createReaddirReturn(path, files));
      } else {
        reject(err);
      }
    });
  });
}

export function readdirSync(path: AbsoluteFilePath): AbsoluteFilePathSet {
  return createReaddirReturn(path, fs.readdirSync(path.join()));
}

// lstat
export function lstat(path: AbsoluteFilePath): Promise<fs.Stats> {
  return promisifyData(
    path,
    (filename, callback) => fs.lstat(filename, callback),
  );
}

export function lstatSync(path: AbsoluteFilePath): fs.Stats {
  return fs.lstatSync(path.join());
}

// exists
export function exists(path: AbsoluteFilePath): Promise<boolean> {
  return new Promise((resolve) => {
    fs.exists(path.join(), (exists) => {
      resolve(exists);
    });
  });
}

export function existsSync(path: AbsoluteFilePath): boolean {
  return fs.existsSync(path.join());
}

// unlink
export function unlink(path: AbsoluteFilePath): Promise<void> {
  return promisifyVoid(path, (filename, callback) => 
    fs.unlink(filename, (err) => {
      if (err != null && err.code !== 'ENOENT') {
        callback(err);
      } else {
        callback(null);
      }
    })
  );
}

export function unlinkSync(path: AbsoluteFilePath): void {
  try {
    fs.unlinkSync(path.join());
  } catch (err) {
    if (err.code !== 'ENOENT') {
      throw err;
    }
  }
}

// createDirectory
export function createDirectory(
  path: AbsoluteFilePath,
  opts: CreateDirectoryOptions = {},
): Promise<void> {
  return promisifyVoid(path, (filename, callback) => 
    fs.mkdir(filename, {recursive: opts.recursive}, callback)
  );
}

export function createDirectorySync(
  path: AbsoluteFilePath,
  opts: CreateDirectoryOptions = {},
): void {
  fs.mkdirSync(path.join(), {recursive: opts.recursive});
}

type CreateDirectoryOptions = {recursive?: boolean};
