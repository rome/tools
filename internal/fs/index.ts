/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AbsoluteFilePath,
	AbsoluteFilePathSet,
	createAbsoluteFilePath,
} from "@internal/path";
import {
	NodeSystemError,
	convertPossibleNodeErrorToDiagnostic,
} from "@internal/node";
import {getEnvVar} from "@internal/cli-environment";
import {
	getErrorStructure,
	setErrorFrames,
	setNodeErrorProps,
} from "@internal/v8";
import fs = require("fs");
import {FileNotFound} from "@internal/fs/FileNotFound";

// Most fs errors don't have a stack trace. This is due to the way that node queues file operations.
// Capturing a stacktrace would be very expensive.
// So here we just alternatively do it ourselves if we were passed the ROME_FS_ERRORS=1 env var
// https://github.com/nodejs/node/issues/30944

const debugErrors = getEnvVar("ROME_FS_ERRORS").type === "ENABLED";
function wrapReject<T>(promise: Promise<T>, addFrames: number): Promise<T> {
	const callError = debugErrors ? new Error() : undefined;

	return promise.catch((err: NodeSystemError) => {
		if (callError !== undefined) {
			// Remove wrapReject frame and custom addFrames to get to the real callsite
			setErrorFrames(
				err,
				getErrorStructure(callError).frames.slice(1 + addFrames),
			);
		}

		// Convert ENOENT to FileNotFound errors, if we want this to be a pretty node error then it can be converted later
		if (err.code === "ENOENT" && err.path !== undefined) {
			const err2 = new FileNotFound(createAbsoluteFilePath(err.path));
			setNodeErrorProps(err2, err);
			setErrorFrames(err2, getErrorStructure(err).frames);
			return Promise.reject(err2);
		}

		// If the error has no stacktrace then we'll recommend adding the envvar
		return Promise.reject(convertPossibleNodeErrorToDiagnostic(err));
	});
}

export {FileNotFound} from "./FileNotFound";

// Reexported types: Only file that ever imports the fs module is this one
export type FileHandle = fs.promises.FileHandle;
export type WriteStream = fs.WriteStream;
export type ReadStream = fs.ReadStream;
export type FSWatcher = fs.FSWatcher;

// This file contains some wrappers around Node's fs module. Except here we support passing in AbsoluteFilePath instances.
// NOTE We don't bother using Node's built-in fs promise functions at all. They already contain a level of indirection to callbacks.

// Helpers
type DataCallback<Data> = (err: null | Error, data: Data) => void;

function promisifyData<Data>(
	path: AbsoluteFilePath,
	factory: (path: string, callback: DataCallback<Data>) => void,
): Promise<Data> {
	return wrapReject(
		new Promise((resolve, reject) => {
			factory(
				path.join(),
				(err, data) => {
					if (err === null) {
						resolve(data);
					} else {
						reject(err);
					}
				},
			);
		}),
		2,
	);
}

type VoidCallback = (err: null | Error) => void;

function promisifyVoid(
	path: AbsoluteFilePath,
	factory: (path: string, callback: VoidCallback) => void,
): Promise<void> {
	return wrapReject(
		new Promise((resolve, reject) => {
			factory(
				path.join(),
				(err) => {
					if (err === null) {
						resolve();
					} else {
						reject(err);
					}
				},
			);
		}),
		2,
	);
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

// readFileText
export async function readFileText(path: AbsoluteFilePath): Promise<string> {
	return (await readFile(path)).toString();
}

// Return value is meant to be consumed via ParserOptions
export async function readFileTextMeta(
	path: AbsoluteFilePath,
): Promise<{
	path: AbsoluteFilePath;
	input: string;
}> {
	return {
		input: (await readFile(path)).toString(),
		path,
	};
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
	return wrapReject(
		new Promise((resolve, reject) => {
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
		}),
		1,
	);
}

// lstat
export function lstat(path: AbsoluteFilePath): Promise<fs.Stats> {
	return promisifyData(
		path,
		(filename, callback) => fs.lstat(filename, callback),
	);
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

// We previously just use fs.rmdir with the `recursive: true` flag but it was added in Node 12.10 and we need to support 12.8.1
// NB: There are probably race conditions, we could switch to openFile and openDirectory if it's a problem
// https://github.com/romefrontend/rome/issues/1001
export async function removeDirectory(path: AbsoluteFilePath): Promise<void> {
	// Delete all inner files
	for (const subpath of await readDirectory(path)) {
		const stats = await lstat(subpath);
		if (stats.isDirectory()) {
			await removeDirectory(subpath);
		} else {
			await removeFile(subpath);
		}
	}

	// Remove directory with all files deleted
	return promisifyVoid(
		path,
		(filename, callback) => fs.rmdir(filename, callback),
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

// open
export function openFile(
	path: AbsoluteFilePath,
	flags: fs.OpenMode = "r",
	mode?: fs.Mode,
): Promise<fs.promises.FileHandle> {
	return fs.promises.open(path.join(), flags, mode);
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

// createWriteStream
export function createWriteStream(
	path: AbsoluteFilePath,
	opts?: Parameters<typeof fs.createWriteStream>[1],
): fs.WriteStream {
	return fs.createWriteStream(path.join(), opts);
}

// createReadStream
export function createReadStream(
	path: AbsoluteFilePath,
	opts?: Parameters<typeof fs.createReadStream>[1],
): fs.ReadStream {
	return fs.createReadStream(path.join(), opts);
}

// Super special sync methods that we should only use sparingly if there's absolutely no way to do them async

export function readFileTextSync(path: AbsoluteFilePath): string {
	return fs.readFileSync(path.join(), "utf8");
}

export function lstatSync(path: AbsoluteFilePath): fs.Stats {
	return fs.lstatSync(path.join());
}
