/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {SemverVersion, SemverWildcardVersion} from "./types";

// -1: Lesser
// 0: Equal
// 1: Greater
type CompareRet = -1 | 0 | 1;

function compareIdentifiers(
	a: number | string,
	b: undefined | string | number,
): CompareRet {
	// Equal
	if (b === undefined) {
		return 0;
	}

	if (typeof a === "string" || typeof b === "string") {
		// @ts-expect-error: built-in def is not restrictive enough
		return String(a).localeCompare(String(b));
	}

	// Less than
	if (a < b) {
		return -1;
	}

	// Greater than
	if (a > b) {
		return 1;
	}

	// Equal
	return 0;
}

function compareMain(
	version: SemverVersion,
	range: SemverWildcardVersion | SemverVersion,
): CompareRet {
	return (
		compareIdentifiers(version.major, range.major) ||
		compareIdentifiers(version.minor, range.minor) ||
		compareIdentifiers(version.patch, range.patch)
	);
}

function comparePre(
	version: SemverVersion,
	range: SemverWildcardVersion | SemverVersion,
): CompareRet {
	// NOT having a prerelease is > having one
	if (version.prerelease.length > 0 && range.prerelease.length === 0) {
		return -1;
	} else if (version.prerelease.length === 0 && range.prerelease.length > 0) {
		return 1;
	} else if (version.prerelease.length === 0 && range.prerelease.length === 0) {
		return 0;
	}

	let i = 0;
	do {
		const a = version.prerelease[i];
		const b = range.prerelease[i];

		if (a === undefined && b === undefined) {
			return 0;
		} else if (b === undefined) {
			return 1;
		} else if (a === undefined) {
			return -1;
		} else if (a === b) {
			continue;
		} else {
			return compareIdentifiers(a, b);
		}
	} while (++i);

	throw new Error("Unreachable");
}

export function compareFromAst(
	version: SemverVersion,
	range: SemverWildcardVersion | SemverVersion,
): CompareRet {
	return compareMain(version, range) || comparePre(version, range);
}
