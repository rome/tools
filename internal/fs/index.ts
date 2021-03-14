/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import fs = require("fs");

export {default as FileNotFound} from "./FileNotFound";
export {default as CachedFileReader} from "./CachedFileReader";

// Reexported types: Only file that ever imports the fs module is this one
export type FSHandle = fs.promises.FileHandle;
export type FSWriteStream = fs.WriteStream;
export type FSReadStream = fs.ReadStream;
export type FSWatcher = fs.FSWatcher;
export type FSStats = fs.BigIntStats;

export function createFakeStats(
	{date, size, type}: {
		type: "directory" | "file";
		size: bigint;
		date: Date;
	},
): FSStats {
	const ms = BigInt(Math.floor(date.valueOf()));
	const ns = BigInt(date.valueOf()) * 1000000n;

	return {
		isFile: () => type === "file",
		isDirectory: () => type === "directory",
		isBlockDevice: () => false,
		isCharacterDevice: () => false,
		isSymbolicLink: () => false,
		isFIFO: () => false,
		isSocket: () => false,

		dev: 0n,
		ino: 0n,
		mode: 16895n,
		nlink: 0n,
		uid: 0n,
		gid: 0n,
		rdev: 0n,
		size,
		blksize: 0n,
		blocks: 0n,
		atimeMs: ms,
		mtimeMs: ms,
		ctimeMs: ms,
		birthtimeMs: ms,
		atime: date,
		mtime: date,
		ctime: date,
		birthtime: date,
		atimeNs: ns,
		mtimeNs: ns,
		ctimeNs: ns,
		birthtimeNs: ns,
	};
}
