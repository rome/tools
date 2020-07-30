/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {RangeNode} from "./types";

// Remove all
function compactRight(
	parts: Array<undefined | number>,
): Array<undefined | number> {
	for (let i = parts.length - 1; i >= 0; i--) {
		const part = parts[i];
		if (part !== undefined) {
			return parts.slice(0, i + 1);
		}
	}

	return [];
}

const WILDCARD = "*";

export default function stringify(node: RangeNode): string {
	switch (node.type) {
		case "WildcardVersion":
		case "AbsoluteVersion": {
			// Build up x.x.x format
			let str = compactRight([node.major, node.minor, node.patch]).map((part) =>
				part ?? WILDCARD
			).join(".");

			// add on qualifiers
			if (node.prerelease.length > 0) {
				str += `-${node.prerelease.join(".")}`;
			}
			if (node.build.length > 0) {
				str += `+${node.build.join(".")}`;
			}
			return str;
		}

		case "Wildcard":
			return WILDCARD;

		case "Comparator":
			return `${node.operator}${stringify(node.version)}`;

		case "LogicalAnd":
			return `${stringify(node.left)} ${stringify(node.right)}`;

		case "LogicalOr":
			return `${stringify(node.left)} || ${stringify(node.right)}`;

		case "VersionRange":
			return `${stringify(node.left)} - ${stringify(node.right)}`;
	}
}
