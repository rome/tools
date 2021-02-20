/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {PathPattern, PathPatternNode, PathPatterns} from "./types";
import {parsePattern, parsePatternsFile} from "./parse";
import match from "./match";
import {AbsoluteFilePath, PathSegments} from "@internal/path";

export {PathPattern, PathPatterns};

export {
	parsePattern as parsePathPattern,
	parsePatternsFile as parsePathPatternsFile,
};

export {stringifyPathPattern} from "./stringify";

export function flipPathPatterns(patterns: PathPatterns): PathPatterns {
	return patterns.map((pattern) => {
		if (pattern.type === "Comment") {
			return pattern;
		} else {
			return {
				...pattern,
				negate: !pattern.negate,
			};
		}
	});
}

export function matchPath(
	path: AbsoluteFilePath,
	patternNode: PathPattern,
	cwdSegs?: PathSegments,
): boolean {
	if (patternNode.type === "Comment") {
		return false;
	}

	const matches = match(path.getSegments(), patternNode, cwdSegs);

	if (patternNode.negate) {
		return !matches;
	} else {
		return matches;
	}
}

function getGreater(
	pattern: PathPatternNode,
	matches: undefined | Matches,
): Matches {
	if (matches === undefined || pattern.segments.length > matches.segments) {
		return {
			segments: pattern.segments.length,
			pattern,
		};
	} else {
		return matches;
	}
}

type MatchPatternResult =
	| {
			type: "NO_MATCH";
		}
	| {
			type: "IMPLICIT_MATCH" | "EXPLICIT_MATCH";
			pattern: PathPattern;
		};

type Matches = {
	segments: number;
	pattern: PathPattern;
};

export function matchPathPatterns(
	path: AbsoluteFilePath,
	patterns: PathPatterns,
	cwd?: AbsoluteFilePath,
): MatchPatternResult {
	// Bail out if there are no patterns
	if (patterns.length === 0) {
		return {type: "NO_MATCH"};
	}

	let matches: undefined | Matches;
	let negateMatches: undefined | Matches;

	const pathSegments = path.getSegments();
	const cwdSegs = cwd === undefined ? undefined : cwd.getSegments();

	for (const pattern of patterns) {
		// No point in matching an empty pattern, could just contain a comment
		if (pattern.type === "Comment" || pattern.segments.length === 0) {
			continue;
		}

		if (pattern.negate) {
			if (match(pathSegments, {...pattern, negate: false}, cwdSegs)) {
				negateMatches = getGreater(pattern, negateMatches);
			}
		} else {
			if (match(pathSegments, pattern, cwdSegs)) {
				matches = getGreater(pattern, matches);
			}
		}
	}

	if (matches !== undefined) {
		// If we have a negate pattern, then we need to match more segments than it in order to qualify as a match
		if (negateMatches !== undefined) {
			if (negateMatches.segments > matches.segments) {
				return {type: "NO_MATCH"};
			} else if (matches.segments > negateMatches.segments) {
				return {type: "EXPLICIT_MATCH", pattern: matches.pattern};
			} else {
				return {type: "IMPLICIT_MATCH", pattern: matches.pattern};
			}
		}

		if (matches.segments > 0) {
			return {type: "EXPLICIT_MATCH", pattern: matches.pattern};
		}
	}

	return {type: "NO_MATCH"};
}
