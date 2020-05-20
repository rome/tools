/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	CommentNode,
	PathPatternNode,
	PatternPartNode,
	PatternSegmentNode,
} from "./types";

export function stringifyPathPattern(
	node: CommentNode | PathPatternNode | PatternPartNode | PatternSegmentNode,
): string {
	switch (node.type) {
		case "Comment":
			return `#${node.value}`;

		case "PathPattern":
			return node.segments.map((segment) => stringifyPathPattern(segment)).join(
				"/",
			);

		case "Segment":
			return node.parts.map((part) => stringifyPathPattern(part)).join("");

		case "WildcardSegment":
			return "**";

		case "Wildcard":
			return "*";

		case "Word":
			return node.value;
	}
}
