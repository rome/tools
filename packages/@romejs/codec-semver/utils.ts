/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AbsoluteVersionNode, RangeNode, UserRange, UserVersion} from "./types";
import {
	SemverParserOptions,
	parseSemverRange,
	parseSemverVersion,
} from "./parse";

export function normalizeUserVersion(
	ver: UserVersion,
	opts?: SemverParserOptions,
): AbsoluteVersionNode {
	if (typeof ver === "string") {
		return parseSemverVersion({...opts, input: ver});
	} else if (ver.type === "AbsoluteVersion") {
		return ver;
	} else {
		throw new Error(`Not a valid version: ${ver.type}`);
	}
}

export function normalizeUserRange(
	range: UserRange,
	opts?: SemverParserOptions,
): RangeNode {
	if (typeof range === "string") {
		return parseSemverRange({...opts, input: range});
	} else {
		return range;
	}
}
