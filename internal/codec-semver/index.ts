/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AbsoluteVersionNode,
	RangeNode,
	UserRange,
	UserVersion,
	UserVersions,
} from "./types";
import {satisfiesFromAst} from "./satisfies";
import {compareFromAst} from "./compare";
import {
	SemverParserOptions,
	parseSemverRange,
	parseSemverVersion,
} from "./parse";
import {normalizeUserRange, normalizeUserVersion} from "./utils";
import {DiagnosticsError} from "@internal/diagnostics";

// export some simple types that don't expose too much internal terminology
export type SemverRangeNode = RangeNode;

export type SemverVersionNode = AbsoluteVersionNode;

export {parseSemverRange, parseSemverVersion};

export {default as stringifySemver} from "./stringify";

export function sortSemverVersions(
	rawVersions: UserVersions,
	opts?: SemverParserOptions,
): Array<AbsoluteVersionNode> {
	const versions = rawVersions.map((ver) => normalizeUserVersion(ver, opts));
	return versions.sort((a, b) => compareFromAst(a, b));
}

export function maxSatisfyingSemver(
	rawVersions: UserVersions,
	rawRange: UserRange,
	opts: SemverParserOptions,
): undefined | AbsoluteVersionNode {
	const versions = sortSemverVersions(rawVersions, opts).reverse();
	const range = normalizeUserRange(rawRange, opts);

	for (const version of versions) {
		if (satisfiesFromAst(version, range)) {
			return version;
		}
	}

	return undefined;
}

export function minSatisfyingSemver(
	rawVersions: UserVersions,
	rawRange: UserRange,
	opts?: SemverParserOptions,
): undefined | AbsoluteVersionNode {
	const versions = sortSemverVersions(rawVersions, opts);
	const range = normalizeUserRange(rawRange, opts);

	for (const version of versions) {
		if (satisfiesFromAst(version, range)) {
			return version;
		}
	}

	return undefined;
}

export function satisfiesSemver(
	rawVersion: UserVersion,
	rawRange: UserRange,
	opts?: SemverParserOptions,
) {
	try {
		const version = normalizeUserVersion(rawVersion, opts);
		const range = normalizeUserRange(rawRange, opts);
		return satisfiesFromAst(version, range);
	} catch (err) {
		if (err instanceof DiagnosticsError) {
			return false;
		} else {
			throw err;
		}
	}
}
