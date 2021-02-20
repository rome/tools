/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import AbsoluteFilePath from "./classes/AbsoluteFilePath";
import RelativePath from "./classes/RelativePath";
import UIDPath from "./classes/UIDPath";
import URLPath from "./classes/URLPath";
import {AnyPath} from "./types";

export {default as AbsoluteFilePath} from "./classes/AbsoluteFilePath";
export {default as RelativePath} from "./classes/RelativePath";
export {default as UIDPath} from "./classes/UIDPath";
export {default as URLPath} from "./classes/URLPath";

export * from "./factories";
export * from "./collections";
export * from "./types";
export * from "./constants";

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
