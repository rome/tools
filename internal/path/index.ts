/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import os = require("os");
import AbsoluteFilePath from "./classes/AbsoluteFilePath";
import RelativePath from "./classes/RelativePath";
import UIDPath from "./classes/UIDPath";
import URLPath from "./classes/URLPath";
import {PathTypeHint, ParsedPath, parsePathSegments, splitPathSegments} from "./parse";
import { AnyFilePath, AnyPath } from "./types";

export {default as AbsoluteFilePath} from "./classes/AbsoluteFilePath";
export {default as RelativePath} from "./classes/RelativePath";
export {default as UIDPath} from "./classes/UIDPath";
export {default as URLPath} from "./classes/URLPath";

export * from "./collections";
export * from "./types";

export const HOME_PATH = createAbsoluteFilePath(os.userInfo().homedir);
export const TEMP_PATH = createAbsoluteFilePath(os.tmpdir());
export const CWD_PATH = createAbsoluteFilePath(process.cwd());
export const UNKNOWN_PATH = createUIDPath("unknown");

type CreationArg = AnyPath | string;

function createPathFromParsed(parsed: ParsedPath): AnyPath {
	switch (parsed.absoluteType) {
		case "windows-drive":
		case "windows-unc":
		case "posix": {
			if (parsed.absoluteTarget !== undefined) {
				return new AbsoluteFilePath(parsed);
			}
			break;
		}

		case "url":
			return new URLPath(parsed);

		case "uid":
			return new UIDPath(parsed);
	}

	return new RelativePath(parsed);
}

export function createPathFromSegments(
	segments: string[],
	hint: PathTypeHint,
): AnyPath {
	const parsed = parsePathSegments(segments, hint);
	return createPathFromParsed(parsed);
}

export function createRelativePath(filename: CreationArg): RelativePath {
	return createAnyPath(filename, "relative").assertRelative();
}

export function createURLPath(filename: CreationArg): URLPath {
	return createAnyPath(filename, "auto").assertURL();
}

export function createAbsoluteFilePath(filename: CreationArg): AbsoluteFilePath {
	return createAnyPath(filename, "absolute").assertAbsolute();
}

export function createUIDPath(filename: CreationArg): UIDPath {
	return createAnyPath(filename, "uid").assertUID();
}

export function createFilePath(filename: CreationArg): AnyFilePath {
	return createAnyPath(filename, "absolute").assertFilePath();
}

export function createAnyPath(
	param: CreationArg,
	hint: PathTypeHint = "auto",
): AnyPath {
	// Allows using the create methods above to be used in places where strings are more ergonomic (eg. in third-party code)
	if (isPath(param)) {
		return param;
	}

	const segments = splitPathSegments(param);
	const parsed = parsePathSegments(segments, hint);
	return createPathFromParsed(parsed);
}

// These are some utility methods so you can pass in `undefined | string`
export function maybeCreateURLPath(
	filename: undefined | CreationArg,
): undefined | URLPath {
	if (filename !== undefined) {
		return createURLPath(filename);
	} else {
		return undefined;
	}
}

export function maybeCreateRelativePath(
	filename: undefined | CreationArg,
): undefined | RelativePath {
	if (filename !== undefined) {
		return createRelativePath(filename);
	} else {
		return undefined;
	}
}

export function maybeCreateAbsoluteFilePath(
	filename: undefined | CreationArg,
): undefined | AbsoluteFilePath {
	if (filename !== undefined) {
		return createAbsoluteFilePath(filename);
	} else {
		return undefined;
	}
}

export function maybeCreateAnyPath(
	filename: undefined | CreationArg,
): undefined | AnyPath {
	if (filename !== undefined) {
		return createAnyPath(filename, "auto");
	} else {
		return undefined;
	}
}

export function maybeCreateUIDPath(
	filename: undefined | CreationArg,
): undefined | UIDPath {
	if (filename !== undefined) {
		return createUIDPath(filename);
	} else {
		return undefined;
	}
}

export function equalPaths(
	a: undefined | AnyPath,
	b: undefined | AnyPath,
): boolean {
	if (a === b) {
		return true;
	}

	if (a !== undefined && b !== undefined) {
		return a.equal(b);
	}

	return false;
}

export function isPath(val: unknown): val is AnyPath {
	return (
		val instanceof RelativePath ||
		val instanceof AbsoluteFilePath ||
		val instanceof URLPath ||
		val instanceof UIDPath
	);
}
