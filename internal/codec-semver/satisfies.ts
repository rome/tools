/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AbsoluteVersionNode,
	ComparatorOperator,
	RangeNode,
	VersionNode,
	WildcardNode,
} from "./types";
import {compareFromAst} from "./compare";

function buildVersion(
	major: undefined | number,
	minor: undefined | number,
	patch: undefined | number,
): VersionNode {
	return {
		type: "WildcardVersion",
		major,
		minor,
		patch,
		prerelease: [],
		build: [],
	};
}

function compareOp(
	op: ComparatorOperator,
	version: AbsoluteVersionNode,
	range: WildcardNode | VersionNode,
): boolean {
	if (range.type === "Wildcard") {
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
					// @ts-ignore
					return compareOp("<", version, buildVersion(0, 0, patch + 1));
				} else {
					// ^0.2.3 := >=0.2.3 <0.3.0
					// @ts-ignore
					return compareOp("<", version, buildVersion(0, minor + 1, 0));
				}
			}

			// ^1.2.3 := >=1.2.3 <2.0.0

			// @ts-ignore
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
				// @ts-ignore
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
	version: AbsoluteVersionNode,
	left: WildcardNode | VersionNode,
	right: WildcardNode | VersionNode,
): boolean {
	if (left.type === "Wildcard" || right.type === "Wildcard") {
		return true;
	}

	return compareOp(">=", version, left) && compareOp("<=", version, right);
}

function collectVersions(range: RangeNode): Array<VersionNode> {
	switch (range.type) {
		case "AbsoluteVersion":
		case "WildcardVersion":
			return [range];

		case "Wildcard":
			return [];

		case "Comparator":
			return collectVersions(range.version);

		case "LogicalAnd":
		case "LogicalOr":
		case "VersionRange":
			return [...collectVersions(range.left), ...collectVersions(range.right)];

		default:
			throw new Error("Unknown range type");
	}
}

export function satisfiesFromAst(
	version: AbsoluteVersionNode,
	range: RangeNode,
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

function satisfiesSub(version: AbsoluteVersionNode, range: RangeNode): boolean {
	switch (range.type) {
		case "AbsoluteVersion":
		case "WildcardVersion":
			return compareOp("=", version, range);

		case "Wildcard":
			return true;

		case "Comparator":
			return compareOp(range.operator, version, range.version);

		case "LogicalAnd":
			return (
				satisfiesSub(version, range.left) && satisfiesSub(version, range.right)
			);

		case "LogicalOr":
			return (
				satisfiesSub(version, range.left) || satisfiesSub(version, range.right)
			);

		case "VersionRange":
			return (
				inRange(version, range.left, range.right) ||
				inRange(version, range.right, range.left)
			);
	}
}
