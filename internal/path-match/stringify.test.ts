import {test} from "rome";
import {stringifyPathPattern} from "@internal/path-match/stringify";
import {
	CommentNode,
	PathPatternNode,
	PatternWildcardSegmentNode,
	PatternWordSegmentNode,
	WildcardNode,
	WordNode,
} from "./types";

test(
	"should stringify paths containing comments, wildcards and words",
	async (t) => {
		const comment = (<CommentNode>{
			type: "Comment",
			value: "lorem ipsum",
		});
		t.is(stringifyPathPattern(comment), "#lorem ipsum");

		const wildcardSegment = (<PatternWildcardSegmentNode>{
			type: "WildcardSegment",
		});
		t.is(stringifyPathPattern(wildcardSegment), "**");

		const wildcard = (<WildcardNode>{
			type: "Wildcard",
		});
		t.is(stringifyPathPattern(wildcard), "*");

		const word = (<WordNode>{
			type: "Word",
			value: "lorem",
		});
		t.is(stringifyPathPattern(word), "lorem");

		const segment = (<PatternWordSegmentNode>{
			type: "Segment",
			parts: [word, wildcard, word],
		});
		t.is(stringifyPathPattern(segment), "lorem*lorem");

		const path = (<PathPatternNode>{
			type: "PathPattern",
			root: false,
			negate: false,
			segments: [segment, segment],
		});
		t.is(stringifyPathPattern(path), "lorem*lorem/lorem*lorem");
	},
);
