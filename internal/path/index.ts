/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import AbsoluteFilePath from "./classes/AbsoluteFilePath";
import RelativePath from "./classes/RelativePath";
import UIDPath from "./classes/UIDPath";
import DataURIPath from "./classes/DataURIPath";
import URLPath from "./classes/URLPath";
import {Path, Pathish} from "./types";
import { isObject } from "@internal/typescript-helpers";

export {default as AbsoluteFilePath} from "./classes/AbsoluteFilePath";
export {default as RelativePath} from "./classes/RelativePath";
export {default as UIDPath} from "./classes/UIDPath";
export {default as URLPath} from "./classes/URLPath";
export {default as DataURIPath} from "./classes/DataURIPath";

export {validateParsedPathWindowsDriveLetter} from "./parse";
export * from "./factories";
export * from "./collections";
export * from "./types";
export * from "./constants";

export function equalPaths(a: undefined | Path, b: undefined | Path): boolean {
	if (a === b) {
		return true;
	}

	if (a !== undefined && b !== undefined) {
		return a.equal(b);
	}

	return false;
}

export function isPath(val: unknown): val is Path {
	return (
		val instanceof RelativePath ||
		val instanceof AbsoluteFilePath ||
		val instanceof URLPath ||
		val instanceof UIDPath ||
		val instanceof DataURIPath
	);
}

export function isPathish(val: unknown): val is Pathish {
	return isObject(val) && typeof val.join === "function" && typeof val.join() === "string" && typeof val.format === "function" && typeof val.format() === "string";
}
