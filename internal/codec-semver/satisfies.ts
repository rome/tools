/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	SemverComparatorOperator,
	SemverRange,
	SemverVersion,
	SemverWildcard,
	SemverWildcardVersion,
} from "./types";
import {compareFromAst} from "./compare";

function buildVersion(
	major: undefined | number,
	minor: undefined | number,
	patch: undefined | number,
): SemverWildcardVersion | SemverVersion {
	return {
		type: "SemverWildcardVersion",
		major,
		minor,
		patch,
		prerelease: [],
		build: [],
	};
}

function compareOp(
	op: SemverComparatorOperator,
	version: SemverVersion,
	range: SemverWildcard | SemverWildcardVersion | SemverVersion,
): boolean {
	if (range.type === "SemverWildcard") {
		return true;
	}

	switch (op) {
		case "=":
			return compareFromAst(version, range) === 0;

		case "<":
			return compareFromAst(version, range) < 0;

		case ">":
			return compareFromAst(version, range) > 0;

		case ">=":
			return compareFromAst(version, range) >= 0;

		case "<=":
			return compareFromAst(version, range) <= 0;

		case "^": {
			// Make sure that the version isn't less than the range
			if (!compareOp(">=", version, range)) {
				return false;
			}

			// Deconstruct the range
			const {major, minor, patch} = range;

			if (major === 0) {
				if (minor === 0) {
					// ^0.0.3 := >=0.0.3 <0.0.4
					// @ts-expect-error
					return compareOp("<", version, buildVersion(0, 0, patch + 1));
				} else {
					// ^0.2.3 := >=0.2.3 <0.3.0
					// @ts-expect-error
					return compareOp("<", version, buildVersion(0, minor + 1, 0));
				}
			}

			// ^1.2.3 := >=1.2.3 <2.0.0

			// @ts-expect-error
			return compareOp("<", version, buildVersion(major + 1, 0, 0));
		}

		case "~>":
		case "~": {
			// Make sure that the version isn't less than the range
			if (!compareOp(">=", version, range)) {
				return false;
			}

			// Deconstruct the range
			const {major, minor} = range;

			if (minor === undefined) {
				// ~1 := >=1.0.0 <(1+1).0.0 := >=1.0.0 <2.0.0 (Same as 1.x)
				// @ts-expect-error
				return compareOp("<", version, buildVersion(major + 1, minor, 0));
			}

			// ~1.2.3 := >=1.2.3 <1.(2+1).0 := >=1.2.3 <1.3.0
			return compareOp("<", version, buildVersion(major, minor + 1, 0));
		}

		default:
			throw new Error(`Unknown operator ${op}`);
	}
}

function inRange(
	version: SemverVersion,
	left: SemverWildcard | SemverWildcardVersion | SemverVersion,
	right: SemverWildcard | SemverWildcardVersion | SemverVersion,
): boolean {
	if (left.type === "SemverWildcard" || right.type === "SemverWildcard") {
		return true;
	}

	return compareOp(">=", version, left) && compareOp("<=", version, right);
}

function collectVersions(
	range: SemverRange,
): (SemverWildcardVersion | SemverVersion)[] {
	switch (range.type) {
		case "SemverAbsoluteVersion":
		case "SemverWildcardVersion":
			return [range];

		case "SemverWildcard":
			return [];

		case "SemverComparator":
			return collectVersions(range.version);

		case "SemverLogicalAnd":
		case "SemverLogicalOr":
		case "SemverVersionRange":
			return [...collectVersions(range.left), ...collectVersions(range.right)];

		default:
			throw new Error("Unknown range type");
	}
}

export function satisfiesFromAst(
	version: SemverVersion,
	range: SemverRange,
): boolean {
	const res = satisfiesSub(version, range);
	if (!res) {
		return false;
	}

	if (version.prerelease.length > 0) {
		// Find the set of versions that are allowed to have prereleases
		// For example, ^1.2.3-pr.1 desugars to >=1.2.3-pr.1 <2.0.0
		// That should allow `1.2.3-pr.2` to pass.
		// However, `1.2.4-alpha.notready` should NOT be allowed,
		// even though it's within the range set by the comparators.
		const versions = collectVersions(range);

		for (const comparator of versions) {
			if (comparator.prerelease.length > 0) {
				if (
					comparator.major === version.major &&
					comparator.minor === version.minor &&
					comparator.patch === version.patch
				) {
					return true;
				}
			}
		}

		// Version has a -pre, but it's not one of the ones we like.
		return false;
	}

	return true;
}

function satisfiesSub(version: SemverVersion, range: SemverRange): boolean {
	switch (range.type) {
		case "SemverAbsoluteVersion":
		case "SemverWildcardVersion":
			return compareOp("=", version, range);

		case "SemverWildcard":
			return true;

		case "SemverComparator":
			return compareOp(range.operator, version, range.version);

		case "SemverLogicalAnd":
			return (
				satisfiesSub(version, range.left) && satisfiesSub(version, range.right)
			);

		case "SemverLogicalOr":
			return (
				satisfiesSub(version, range.left) || satisfiesSub(version, range.right)
			);

		case "SemverVersionRange":
			return (
				inRange(version, range.left, range.right) ||
				inRange(version, range.right, range.left)
			);
	}
}
