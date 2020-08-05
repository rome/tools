/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	PathPatternNode,
	PatternPartNode,
	PatternSegmentNode,
	PatternSegments,
} from "./types";
import {PathSegments} from "@internal/path";

function matchSegment(path: string, patternSeg: PatternSegmentNode): boolean {
	if (patternSeg.type !== "Segment") {
		throw new Error("Expected only plain segment");
	}

	const parts = [...patternSeg.parts];
	let buffer: string = path;

	function matchPart(part: PatternPartNode): boolean {
		// If the buffer is empty then it's impossible for anything to match
		if (buffer.length === 0) {
			return false;
		}

		if (part.type === "Word") {
			if (buffer.startsWith(part.value)) {
				buffer = buffer.slice(part.value.length);
			} else {
				return false;
			}
		} else if (part.type === "Wildcard") {
			const nextPart = parts.shift();

			// If there's no other parts then a wildcard matches any buffer
			if (nextPart === undefined) {
				return buffer.length >= 0;
			}

			// Keep removing characters until we match the next part
			while (buffer.length > 0) {
				if (matchPart(nextPart)) {
					return true;
				}

				buffer = buffer.slice(1);
			}

			// We consumed the whole buffer and nothing matched
			return false;
		}

		return true;
	}

	while (parts.length > 0) {
		const part = parts.shift();
		if (part === undefined) {
			throw new Error("parts.length checked above");
		}

		if (!matchPart(part)) {
			return false;
		}
	}

	return true;
}

export default function match(
	pathSegs: PathSegments,
	pattern: PathPatternNode,
	cwdSegs: undefined | PathSegments,
): boolean {
	// Clone so we can freely mutate
	const patternSegs: PatternSegments = [...pattern.segments];
	pathSegs = [...pathSegs];

	// Check if the pattern is empty
	if (patternSegs.length === 0 || pathSegs.length === 0) {
		return false;
	}

	// Quick optimization, check if the path contains all of the absolute names in the pattern
	for (const seg of patternSegs) {
		if (seg.type !== "Segment" || seg.parts.length !== 1) {
			continue;
		}

		const part = seg.parts[0];
		if (part.type === "Word" && !pathSegs.includes(part.value)) {
			return false;
		}
	}

	if (pattern.root && cwdSegs !== undefined) {
		cwdSegs = [...cwdSegs];

		// If this is a root pattern, then remove all the starting path segments that match the cwd
		for (const cwdSeg of cwdSegs) {
			const pathSeg = pathSegs.shift();
			if (cwdSeg !== pathSeg) {
				return false;
			}
		}
	} else {
		// Start removing all the path segments until we find one that matches the first pattern segment
		const firstPatternSeg = patternSegs.shift();
		if (firstPatternSeg === undefined) {
			throw new Error("patternSegs.length already validated above");
		}

		while (pathSegs.length > 0) {
			const pathSeg = pathSegs.shift();
			if (pathSeg === undefined) {
				throw new Error("pathSegs.length already validated above");
			}

			if (matchSegment(pathSeg, firstPatternSeg)) {
				if (pathSegs.length === 0 && patternSegs.length === 0) {
					// If there's no more path or pattern segments then this was a successfully match!
					return true;
				} else {
					// Make sure the rest match
					break;
				}
			}
		}
	}

	// If we consumed all the path segments then we didn't match anything
	if (pathSegs.length === 0) {
		return false;
	}

	// Match the rest of the path segments
	for (let i = 0; i < patternSegs.length; i++) {
		const patternSeg = patternSegs[i];

		// If we have no more path segments then it's impossible for this to match
		if (pathSegs.length === 0) {
			return false;
		}

		// When given a wildcard segment, keep popping off all the path segments until we find one that matches the next pattern segment
		if (patternSeg.type === "WildcardSegment") {
			const nextPattern = patternSegs[i + 1];
			while (pathSegs.length > 0 && !matchSegment(pathSegs[0], nextPattern)) {
				pathSegs.shift();
			}
			continue;
		}

		// Basic match
		const pathSeg = pathSegs.shift();
		if (pathSeg === undefined) {
			throw new Error("pathSegs.length already validated above");
		}
		if (matchSegment(pathSeg, patternSeg)) {
			continue;
		} else {
			return false;
		}
	}

	return true;
}
