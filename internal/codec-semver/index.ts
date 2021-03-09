/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {SemverVersion, SemverRange, SemverModifier} from "./types";
import {satisfiesFromAst} from "./satisfies";
import {compareFromAst} from "./compare";
import {
	parseSemverRange,
	parseSemverVersion,
} from "./parse";
import {DiagnosticsError} from "@internal/diagnostics";

export {SemverVersion, SemverRange, SemverModifier} from "./types";

export {parseSemverRange, parseSemverVersion};

export {default as stringifySemver} from "./stringify";

export function sortSemverVersions(
	versions: SemverVersion[],
): SemverVersion[] {
	return versions.sort((a, b) => compareFromAst(a, b));
}

export function maxSatisfyingSemver(
	unsortedVersions: SemverVersion[],
	range: SemverRange,
): undefined | SemverVersion {
	const versions = sortSemverVersions(unsortedVersions).reverse();

	for (const version of versions) {
		if (satisfiesFromAst(version, range)) {
			return version;
		}
	}

	return undefined;
}

export function minSatisfyingSemver(
	unsortedVersions: SemverVersion[],
	range: SemverRange,
): undefined | SemverVersion {
	const versions = sortSemverVersions(unsortedVersions);

	for (const version of versions) {
		if (satisfiesFromAst(version, range)) {
			return version;
		}
	}

	return undefined;
}

export function satisfiesSemver(
	version: SemverVersion,
	range: SemverRange,
): boolean {
	try {
		return satisfiesFromAst(version, range);
	} catch (err) {
		if (err instanceof DiagnosticsError) {
			return false;
		} else {
			throw err;
		}
	}
}

export function incrementSemver(version: SemverVersion, modifier: SemverModifier): SemverVersion {
	switch (modifier) {
		case SemverModifier.MAJOR:
			return {
				type: "SemverAbsoluteVersion",
				major: version.major + 1,
				minor: 0,
				patch: 0,
				// TODO should we actually be removing these?
				prerelease: [],
				build: [],
			};

		case SemverModifier.MINOR:
			return {
				...version,
				minor: version.minor + 1,
				patch: 0,
				prerelease: [],
				build: [],
			};

		case SemverModifier.PATCH:
			return {
				...version,
				patch: version.patch + 1,
				prerelease: [],
				build: [],
			};
	}
}
