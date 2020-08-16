/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	PathPattern,
	PatternPartNode,
	PatternParts,
	PatternSegmentNode,
	PatternSegments,
	Tokens,
} from "./types";
import {ParserCore, ParserOptions, createParser} from "@internal/parser-core";
import {Number0, ob1Add, ob1Get0, ob1Number0} from "@internal/ob1";
import {descriptions} from "@internal/diagnostics";

function isntNewline(char: string): boolean {
	return char !== "\n";
}

type PatchMatchParserTypes = {
	tokens: Tokens;
	options: ParserOptions;
	state: {};
	meta: void;
};

type PatchMatchParser = ParserCore<PatchMatchParserTypes>;

const createPathMatchParser = createParser<PatchMatchParserTypes>({
	diagnosticCategory: "parse/patchMatch",
	tokenize(parser, index) {
		const char = parser.getInputCharOnly(index);
		const nextChar = parser.getInputCharOnly(index, 1);

		if (char === "*") {
			if (nextChar === "*") {
				return parser.finishToken("DoubleStar", ob1Add(index, 2));
			} else {
				return parser.finishToken("Star");
			}
		} else if (index === ob1Number0 && char === "!") {
			return parser.finishToken("Exclamation");
		} else if (
			char === "#" &&
			parser.getPositionFromIndex(index).column === ob1Number0
		) {
			const [value, end] = parser.readInputFrom(index, isntNewline);
			return parser.finishValueToken("Comment", value, end);
		}

		if (char === "\n") {
			return parser.finishToken("EOL");
		}

		if (char === "/") {
			return parser.finishToken("Separator");
		} else if (char === "\\" && nextChar === "\\") {
			return parser.finishToken("Separator", ob1Add(index, 2));
		}

		const [value, end] = parser.readInputFrom(
			index,
			isWordCharacter.bind(undefined, parser),
		);
		return parser.finishValueToken("Word", value, end);
	},
});

function isWordCharacter(
	parser: PatchMatchParser,
	char: string,
	index: Number0,
	input: string,
): boolean {
	const prevChar = input[ob1Get0(index) - 1];
	const nextChar = input[ob1Get0(index) + 1];

	if (char === "\n") {
		return false;
	}

	// Windows separator
	if (char === "\\" && nextChar === "\\") {
		return false;
	}

	// Any escaped character is a word character
	if (prevChar === "\\") {
		return true;
	}

	// Unix separator and wildcard
	if (char === "/") {
		return false;
	}

	// Wildcard
	if (char === "*") {
		return false;
	}

	return true;
}

function eatSeparators(parser: PatchMatchParser): boolean {
	let ate = false;
	while (parser.eatToken("Separator") !== undefined) {
		ate = true;
	}
	return ate;
}

//# Pattern parsing
function parsePatternSegmentPart(parser: PatchMatchParser): PatternPartNode {
	const startPos = parser.getPosition();
	const token = parser.getToken();
	parser.nextToken();

	switch (token.type) {
		case "Star":
			return {
				type: "Wildcard",
				loc: parser.finishLoc(startPos),
			};

		case "Word":
			return {
				type: "Word",
				loc: parser.finishLoc(startPos),
				value: token.value,
			};

		default:
			throw parser.unexpected({
				start: startPos,
				description: descriptions.PATH_MATCH.INVALID_PATTERN_SEGMENT_PART,
			});
	}
}

function parseSegment(parser: PatchMatchParser): PatternSegmentNode {
	const startPos = parser.getPosition();
	const parts: PatternParts = [];

	// A ** token is only allowed as the only part of a segment
	if (parser.matchToken("DoubleStar")) {
		const lookahead = parser.lookaheadToken();
		if (
			lookahead.type === "Separator" ||
			lookahead.type === "EOF" ||
			lookahead.type === "EOL"
		) {
			parser.eatToken("DoubleStar");
			eatSeparators(parser);
			return {
				type: "WildcardSegment",
				loc: parser.finishLoc(startPos),
			};
		}
	}

	// Keep consuming tokens until we hit a separator or a comment
	while (
		!parser.matchToken("Comment") &&
		!parser.matchToken("EOF") &&
		!eatSeparators(parser) &&
		!parser.matchToken("EOL")
	) {
		parts.push(parsePatternSegmentPart(parser));
	}

	return {
		loc: parser.finishLoc(startPos),
		type: "Segment",
		parts,
	};
}

function isWildcardOnlySegment(
	parser: PatchMatchParser,
	segment: undefined | PatternSegmentNode,
): boolean {
	if (segment === undefined) {
		return false;
	}

	if (segment.type === "WildcardSegment") {
		return true;
	}

	if (segment.parts.length === 1 && segment.parts[0].type === "Wildcard") {
		return true;
	}

	return false;
}

// Normalize all path segments, removing empty segments and wildcards from the start and end
// These could also be parse errors but let's allow them
function normalizePatternSegments(
	parser: PatchMatchParser,
	segments: PatternSegments,
): PatternSegments {
	const normalized: PatternSegments = [];

	// Never normalize it if there's a single segment. This is to support writing a pattern that's just "*"
	if (segments.length === 1) {
		return segments;
	}

	for (const seg of segments) {
		// Remove all wildcard-only segments from 'beginning
		if (normalized.length === 0 && isWildcardOnlySegment(parser, seg)) {
			continue;
		}

		// Remove all empty segments
		if (seg.type === "Segment" && seg.parts.length === 0) {
			continue;
		}

		normalized.push(seg);
	}

	// TODO Remove duplicate wildcard segments
	// - Multiple WildcardSegment
	// - Wildcard next to a WildcardSegment
	// Remove all wildcard-only segments from end
	while (isWildcardOnlySegment(parser, normalized[normalized.length - 1])) {
		normalized.pop();
	}

	return normalized;
}

function eatEOL(parser: PatchMatchParser) {
	while (parser.eatToken("EOL")) {
		// empty
	}
}

function parsePatternNode(parser: PatchMatchParser): PathPattern {
	const startPos = parser.getPosition();
	const segments: PatternSegments = [];
	const negate = parser.eatToken("Exclamation") !== undefined;

	// Keep parsing segments until we hit the end of the input or a comment
	while (
		!parser.matchToken("Comment") &&
		!parser.matchToken("EOF") &&
		!parser.matchToken("EOL")
	) {
		segments.push(parseSegment(parser));
	}

	// Get a trailing comment
	if (parser.matchToken("Comment")) {
		const {value} = parser.expectToken("Comment");
		return {
			type: "Comment",
			loc: parser.finishLoc(startPos),
			value,
		};
	}

	let root = false;
	if (segments.length > 0) {
		const firstSeg = segments[0];
		root = firstSeg.type === "Segment" && firstSeg.parts.length === 0;
	}

	return {
		type: "PathPattern",
		loc: parser.finishLoc(startPos),
		root,
		negate,
		segments: normalizePatternSegments(parser, segments),
	};
}

export function parsePattern(opts: ParserOptions): PathPattern {
	const parser = createPathMatchParser(opts);
	const pattern = parsePatternNode(parser);
	eatEOL(parser);
	parser.finalize();
	return pattern;
}

export function parsePatternsFile(opts: ParserOptions): Array<PathPattern> {
	const parser = createPathMatchParser(opts);
	const patterns: Array<PathPattern> = [];

	while (true) {
		eatEOL(parser);
		if (parser.matchToken("EOF")) {
			break;
		}

		patterns.push(parsePatternNode(parser));
	}

	parser.finalize();
	return patterns;
}
