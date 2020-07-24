/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AbsoluteFilePath, AbsoluteFilePathSet} from "@romefrontend/path";

import fs = require("fs");
import {convertPossibleNodeError} from "@romefrontend/node";

// This file contains some wrappers around Node's fs module. Except here we support passing in AbsoluteFilePath instances.
// NOTE We don't bother using Node's built-in fs promise functions at all. They already contain a level of indirection to callbacks.

// Helpers
type DataCallback<Data> = (err: null | Error, data: Data) => void;

function promisifyData<Data>(
	path: AbsoluteFilePath,
	factory: (path: string, callback: DataCallback<Data>) => void,
): Promise<Data> {
	return new Promise((resolve, reject) => {
		factory(
			path.join(),
			(err, data) => {
				if (err === null) {
					resolve(data);
				} else {
					reject(convertPossibleNodeError(err));
				}
			},
		);
	});
}

type VoidCallback = (err: null | Error) => void;

function promisifyVoid(
	path: AbsoluteFilePath,
	factory: (path: string, callback: VoidCallback) => void,
): Promise<void> {
	return new Promise((resolve, reject) => {
		factory(
			path.join(),
			(err) => {
				if (err === null) {
					resolve();
				} else {
					reject(convertPossibleNodeError(err));
				}
			},
		);
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
	return promisifyData(
		path,
		(filename, callback) => fs.readFile(filename, callback),
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
	return fs.readFileSync(path.join(), "utf8");
}

// writeFile
export function writeFile(
	path: AbsoluteFilePath,
	content: string | Buffer,
): Promise<void> {
	return promisifyVoid(
		path,
		(filename, callback) => fs.writeFile(filename, content, callback),
	);
}

export function writeFileSync(
	path: AbsoluteFilePath,
	content: Buffer | string,
): void {
	return fs.writeFileSync(path.join(), content);
}

// copyFile
export function copyFile(
	src: AbsoluteFilePath,
	dest: AbsoluteFilePath,
): Promise<void> {
	return promisifyVoid(
		src,
		(src, callback) => fs.copyFile(src, dest.join(), callback),
	);
}

export function copyFileSync(
	src: AbsoluteFilePath,
	dest: AbsoluteFilePath,
): void {
	return fs.copyFileSync(src.join(), dest.join());
}

// readdir
function createReaddirReturn(
	directory: AbsoluteFilePath,
	files: Array<string>,
): AbsoluteFilePathSet {
	return new AbsoluteFilePathSet(
		files.sort().map((basename) => {
			return directory.append(basename);
		}),
	);
}

export function readDirectory(
	path: AbsoluteFilePath,
): Promise<AbsoluteFilePathSet> {
	return new Promise((resolve, reject) => {
		fs.readdir(
			path.join(),
			(err, files) => {
				if (err === null) {
					resolve(createReaddirReturn(path, files));
				} else {
					reject(err);
				}
			},
		);
	});
}

export function readDirectorySync(path: AbsoluteFilePath): AbsoluteFilePathSet {
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
		fs.exists(
			path.join(),
			(exists) => {
				resolve(exists);
			},
		);
	});
}

export function existsSync(path: AbsoluteFilePath): boolean {
	return fs.existsSync(path.join());
}

// unlink
export function removeFile(path: AbsoluteFilePath): Promise<void> {
	return promisifyVoid(
		path,
		(filename, callback) =>
			fs.unlink(
				filename,
				(err) => {
					if (err != null && err.code !== "ENOENT") {
						callback(err);
					} else {
						callback(null);
					}
				},
			)
		,
	);
}

export function removeFileSync(path: AbsoluteFilePath): void {
	try {
		fs.unlinkSync(path.join());
	} catch (err) {
		if (err.code !== "ENOENT") {
			throw err;
		}
	}
}

// rmdir
export function removeDirectory(path: AbsoluteFilePath): Promise<void> {
	return promisifyVoid(
		path,
		(filename, callback) =>
			fs.rmdir(
				filename,
				{
					recursive: true,
				},
				callback,
			)
		,
	);
}

export function removeDirectorySync(path: AbsoluteFilePath): void {
	fs.rmdirSync(
		path.join(),
		{
			recursive: true,
		},
	);
}

// createDirectory
export function createDirectory(path: AbsoluteFilePath): Promise<void> {
	return promisifyVoid(
		path,
		(filename, callback) =>
			fs.mkdir(
				filename,
				{
					recursive: true,
				},
				callback,
			)
		,
	);
}

export function createDirectorySync(path: AbsoluteFilePath): void {
	fs.mkdirSync(path.join());
}

// open
export function openFile(
	path: AbsoluteFilePath,
	flags: fs.OpenMode = "r",
	mode?: fs.Mode,
): Promise<fs.promises.FileHandle> {
	return fs.promises.open(path.join(), flags, mode);
}

export function openFileSync(
	path: AbsoluteFilePath,
	flags: fs.OpenMode = "r",
	mode?: fs.Mode,
): number {
	return fs.openSync(path.join(), flags, mode);
}

// openDirectory
export function openDirectory(
	path: AbsoluteFilePath,
	opts: fs.OpenDirOptions = {},
): Promise<fs.Dir> {
	return promisifyData(
		path,
		(filename, callback) => fs.opendir(filename, opts, callback),
	);
}

export function openDirectorySync(
	path: AbsoluteFilePath,
	opts: fs.OpenDirOptions = {},
): fs.Dir {
	return fs.opendirSync(path.join(), opts);
}
