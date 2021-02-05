import {test} from "rome";
import {parseRegExp} from "@internal/codec-js-regexp/index";

test(
	"verify the parser recognizes nodes correctly",
	async (t) => {
		const node1 = parseRegExp({
			path: "unknown",
			// Should be a JSRegExpCharSet containing a JSRegExpCharacter for a, b and c
			input: "[abc]" +
			// Should be a JSRegExpNonWordCharacter
			"\\W",
			unicode: false,
		}).expression;

		t.inlineSnapshot(
			node1,
			'JSRegExpSubExpression {\n\tloc: SourceLocation unknown 1:0-1:7\n\tbody: Array [\n\t\tJSRegExpCharSet {\n\t\t\tinvert: false\n\t\t\tloc: SourceLocation unknown 1:0-1:5\n\t\t\tbody: Array [\n\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\tvalue: "a"\n\t\t\t\t\tloc: SourceLocation unknown 1:1-1:2\n\t\t\t\t}\n\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\tvalue: "b"\n\t\t\t\t\tloc: SourceLocation unknown 1:2-1:3\n\t\t\t\t}\n\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\tvalue: "c"\n\t\t\t\t\tloc: SourceLocation unknown 1:3-1:4\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tJSRegExpNonWordCharacter {loc: SourceLocation unknown 1:5-1:7}\n\t]\n}',
		);

		const node2 = parseRegExp({
			path: "unknown",
			// Should be a JSRegExpCharacter for f
			input: "f" +
			// Should be a JSRegExpQuantified with a min and max of 2
			"o{2}" +
			// Should be a JSRegExpQuantified with a min of 0 and max of 1 containing a JSRegExpSubExpression
			"(bar)?",
			unicode: false,
		}).expression;

		t.inlineSnapshot(
			node2,
			'JSRegExpSubExpression {\n\tloc: SourceLocation unknown 1:0-1:10\n\tbody: Array [\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "f"\n\t\t\tloc: SourceLocation unknown 1:0-1:1\n\t\t}\n\t\tJSRegExpQuantified {\n\t\t\tlazy: false\n\t\t\tmax: 2\n\t\t\tmin: 2\n\t\t\tloc: SourceLocation unknown 1:1-1:5\n\t\t\ttarget: JSRegExpCharacter {\n\t\t\t\tvalue: "o"\n\t\t\t\tloc: SourceLocation unknown 1:1-1:2\n\t\t\t}\n\t\t}\n\t\tJSRegExpQuantified {\n\t\t\tlazy: false\n\t\t\tmax: 1\n\t\t\tmin: 0\n\t\t\tloc: SourceLocation unknown 1:5-1:10\n\t\t\ttarget: JSRegExpGroupCapture {\n\t\t\t\tname: undefined\n\t\t\t\tloc: SourceLocation unknown 1:5-1:10\n\t\t\t\texpression: JSRegExpSubExpression {\n\t\t\t\t\tloc: SourceLocation unknown 1:6-1:9\n\t\t\t\t\tbody: Array [\n\t\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\t\tvalue: "b"\n\t\t\t\t\t\t\tloc: SourceLocation unknown 1:6-1:7\n\t\t\t\t\t\t}\n\t\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\t\tvalue: "a"\n\t\t\t\t\t\t\tloc: SourceLocation unknown 1:7-1:8\n\t\t\t\t\t\t}\n\t\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\t\tvalue: "r"\n\t\t\t\t\t\t\tloc: SourceLocation unknown 1:8-1:9\n\t\t\t\t\t\t}\n\t\t\t\t\t]\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t]\n}',
		);

		const node3 = parseRegExp({
			path: "unknown",
			input: "look" +
			// Should be a JSRegExpGroupNonCapture of kind positive-lookahead
			"(?=ahead)",
			unicode: false,
		}).expression;

		t.inlineSnapshot(
			node3,
			'JSRegExpSubExpression {\n\tloc: SourceLocation unknown 1:0-1:12\n\tbody: Array [\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "l"\n\t\t\tloc: SourceLocation unknown 1:0-1:1\n\t\t}\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "o"\n\t\t\tloc: SourceLocation unknown 1:1-1:2\n\t\t}\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "o"\n\t\t\tloc: SourceLocation unknown 1:2-1:3\n\t\t}\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "k"\n\t\t\tloc: SourceLocation unknown 1:3-1:4\n\t\t}\n\t\tJSRegExpGroupNonCapture {\n\t\t\tkind: "positive-lookahead"\n\t\t\tloc: SourceLocation unknown 1:4-1:12\n\t\t\texpression: JSRegExpSubExpression {\n\t\t\t\tloc: SourceLocation unknown 1:7-1:12\n\t\t\t\tbody: Array [\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "a"\n\t\t\t\t\t\tloc: SourceLocation unknown 1:7-1:8\n\t\t\t\t\t}\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "h"\n\t\t\t\t\t\tloc: SourceLocation unknown 1:8-1:9\n\t\t\t\t\t}\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "e"\n\t\t\t\t\t\tloc: SourceLocation unknown 1:9-1:10\n\t\t\t\t\t}\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "a"\n\t\t\t\t\t\tloc: SourceLocation unknown 1:10-1:11\n\t\t\t\t\t}\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "d"\n\t\t\t\t\t\tloc: SourceLocation unknown 1:11-1:12\n\t\t\t\t\t}\n\t\t\t\t]\n\t\t\t}\n\t\t}\n\t]\n}',
		);
	},
);
