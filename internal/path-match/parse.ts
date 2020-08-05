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
import {ParserOptions, createParser} from "@internal/parser-core";
import {Number0, ob1Add, ob1Get0, ob1Number0} from "@internal/ob1";
import {descriptions} from "@internal/diagnostics";

export type PathMatchParserOptions = ParserOptions;

function isntNewline(char: string): boolean {
	return char !== "\n";
}

const createPathMatchParser = createParser((ParserCore) =>
	class PathMatchParser extends ParserCore<Tokens> {
		constructor(opts: PathMatchParserOptions) {
			super(opts, "parse/patchMatch", {});
		}

		private isWordCharacter(
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

		protected tokenize(index: Number0) {
			const char = this.getInputCharOnly(index);
			const nextChar = this.getInputCharOnly(index, 1);

			if (char === "*") {
				if (nextChar === "*") {
					return this.finishToken("DoubleStar", ob1Add(index, 2));
				} else {
					return this.finishToken("Star");
				}
			} else if (index === ob1Number0 && char === "!") {
				return this.finishToken("Exclamation");
			} else if (
				char === "#" &&
				this.getPositionFromIndex(index).column === ob1Number0
			) {
				const [value, end] = this.readInputFrom(index, isntNewline);
				return this.finishValueToken("Comment", value, end);
			}

			if (char === "\n") {
				return this.finishToken("EOL");
			}

			if (char === "/") {
				return this.finishToken("Separator");
			} else if (char === "\\" && nextChar === "\\") {
				return this.finishToken("Separator", ob1Add(index, 2));
			}

			const [value, end] = this.readInputFrom(
				index,
				this.isWordCharacter.bind(this),
			);
			return this.finishValueToken("Word", value, end);
		}

		private eatSeparators(): boolean {
			let ate = false;
			while (this.eatToken("Separator") !== undefined) {
				ate = true;
			}
			return ate;
		}

		//# Pattern parsing
		private parsePatternSegmentPart(): PatternPartNode {
			const startPos = this.getPosition();
			const token = this.getToken();
			this.nextToken();

			switch (token.type) {
				case "Star":
					return {
						type: "Wildcard",
						loc: this.finishLoc(startPos),
					};

				case "Word":
					return {
						type: "Word",
						loc: this.finishLoc(startPos),
						value: token.value,
					};

				default:
					throw this.unexpected({
						start: startPos,
						description: descriptions.PATH_MATCH.INVALID_PATTERN_SEGMENT_PART,
					});
			}
		}

		private parseSegment(): PatternSegmentNode {
			const startPos = this.getPosition();
			const parts: PatternParts = [];

			// A ** token is only allowed as the only part of a segment
			if (this.matchToken("DoubleStar")) {
				const lookahead = this.lookaheadToken();
				if (
					lookahead.type === "Separator" ||
					lookahead.type === "EOF" ||
					lookahead.type === "EOL"
				) {
					this.eatToken("DoubleStar");
					this.eatSeparators();
					return {
						type: "WildcardSegment",
						loc: this.finishLoc(startPos),
					};
				}
			}

			// Keep consuming tokens until we hit a separator or a comment
			while (
				!this.matchToken("Comment") &&
				!this.matchToken("EOF") &&
				!this.eatSeparators() &&
				!this.matchToken("EOL")
			) {
				parts.push(this.parsePatternSegmentPart());
			}

			return {
				loc: this.finishLoc(startPos),
				type: "Segment",
				parts,
			};
		}

		private isWildcardOnlySegment(
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
		private normalizePatternSegments(segments: PatternSegments): PatternSegments {
			const normalized: PatternSegments = [];

			// Never normalize it if there's a single segment. This is to support writing a pattern that's just "*"
			if (segments.length === 1) {
				return segments;
			}

			for (const seg of segments) {
				// Remove all wildcard-only segments from 'beginning
				if (normalized.length === 0 && this.isWildcardOnlySegment(seg)) {
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
			while (this.isWildcardOnlySegment(normalized[normalized.length - 1])) {
				normalized.pop();
			}

			return normalized;
		}

		private eatEOL() {
			while (this.eatToken("EOL")) {
				// empty
			}
		}

		public parsePatternsFile(): Array<PathPattern> {
			const patterns: Array<PathPattern> = [];

			while (true) {
				this.eatEOL();
				if (this.matchToken("EOF")) {
					break;
				}

				patterns.push(this.parsePattern());
			}

			this.finalize();
			return patterns;
		}

		private parsePattern(): PathPattern {
			const startPos = this.getPosition();
			const segments: PatternSegments = [];
			const negate = this.eatToken("Exclamation") !== undefined;

			// Keep parsing segments until we hit the end of the input or a comment
			while (
				!this.matchToken("Comment") &&
				!this.matchToken("EOF") &&
				!this.matchToken("EOL")
			) {
				segments.push(this.parseSegment());
			}

			// Get a trailing comment
			if (this.matchToken("Comment")) {
				const {value} = this.expectToken("Comment");
				return {
					type: "Comment",
					loc: this.finishLoc(startPos),
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
				loc: this.finishLoc(startPos),
				root,
				negate,
				segments: this.normalizePatternSegments(segments),
			};
		}

		public parseSinglePattern(): PathPattern {
			const pattern = this.parsePattern();
			this.eatEOL();
			this.finalize();
			return pattern;
		}
	}
);

export function parsePattern(opts: PathMatchParserOptions): PathPattern {
	const parser = createPathMatchParser(opts);
	return parser.parseSinglePattern();
}

export function parsePatternsFile(
	opts: PathMatchParserOptions,
): Array<PathPattern> {
	const parser = createPathMatchParser(opts);
	return parser.parsePatternsFile();
}
