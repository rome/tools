import {test} from "rome";
import {parseRegExp} from "@internal/codec-js-regexp/index";

test(
	"verify the parser recognizes nodes correctly",
	async (t) => {
		const node1 = parseRegExp({
			// Should be a JSRegExpCharSet containing a JSRegExpCharacter for a, b and c
			input: "[abc]" +
			// Should be a JSRegExpNonWordCharacter
			"\\W",
			unicode: false,
		}).expression;

		t.inlineSnapshot(
			node1,
			'JSRegExpSubExpression {\n\tbody: [\n\t\tJSRegExpCharSet {\n\t\t\tbody: [\n\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\tvalue: "a"\n\t\t\t\t\tloc: SourceLocation unknown 1:1-1:2\n\t\t\t\t}\n\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\tvalue: "b"\n\t\t\t\t\tloc: SourceLocation unknown 1:2-1:3\n\t\t\t\t}\n\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\tvalue: "c"\n\t\t\t\t\tloc: SourceLocation unknown 1:3-1:4\n\t\t\t\t}\n\t\t\t]\n\t\t\tinvert: false\n\t\t\tloc: SourceLocation unknown 1:0-1:5\n\t\t}\n\t\tJSRegExpNonWordCharacter {\n\t\t\tloc: SourceLocation unknown 1:5-1:7\n\t\t}\n\t]\n\tloc: SourceLocation unknown 1:0-1:7\n}',
		);

		const node2 = parseRegExp({
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
			'JSRegExpSubExpression {\n\tbody: [\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "f"\n\t\t\tloc: SourceLocation unknown 1:0-1:1\n\t\t}\n\t\tJSRegExpQuantified {\n\t\t\tlazy: false\n\t\t\tmax: 2\n\t\t\tmin: 2\n\t\t\ttarget: JSRegExpCharacter {\n\t\t\t\tvalue: "o"\n\t\t\t\tloc: SourceLocation unknown 1:1-1:2\n\t\t\t}\n\t\t\tloc: SourceLocation unknown 1:1-1:5\n\t\t}\n\t\tJSRegExpQuantified {\n\t\t\tlazy: false\n\t\t\tmax: 1\n\t\t\tmin: 0\n\t\t\ttarget: JSRegExpGroupCapture {\n\t\t\t\texpression: JSRegExpSubExpression {\n\t\t\t\t\tbody: [\n\t\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\t\tvalue: "b"\n\t\t\t\t\t\t\tloc: SourceLocation unknown 1:6-1:7\n\t\t\t\t\t\t}\n\t\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\t\tvalue: "a"\n\t\t\t\t\t\t\tloc: SourceLocation unknown 1:7-1:8\n\t\t\t\t\t\t}\n\t\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\t\tvalue: "r"\n\t\t\t\t\t\t\tloc: SourceLocation unknown 1:8-1:9\n\t\t\t\t\t\t}\n\t\t\t\t\t]\n\t\t\t\t\tloc: SourceLocation unknown 1:6-1:9\n\t\t\t\t}\n\t\t\t\tloc: SourceLocation unknown 1:5-1:10\n\t\t\t}\n\t\t\tloc: SourceLocation unknown 1:5-1:10\n\t\t}\n\t]\n\tloc: SourceLocation unknown 1:0-1:10\n}',
		);

		const node3 = parseRegExp({
			input: "look" +
			// Should be a JSRegExpGroupNonCapture of kind positive-lookahead
			"(?=ahead)",
			unicode: false,
		}).expression;

		t.inlineSnapshot(
			node3,
			'JSRegExpSubExpression {\n\tbody: [\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "l"\n\t\t\tloc: SourceLocation unknown 1:0-1:1\n\t\t}\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "o"\n\t\t\tloc: SourceLocation unknown 1:1-1:2\n\t\t}\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "o"\n\t\t\tloc: SourceLocation unknown 1:2-1:3\n\t\t}\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "k"\n\t\t\tloc: SourceLocation unknown 1:3-1:4\n\t\t}\n\t\tJSRegExpGroupNonCapture {\n\t\t\tkind: "positive-lookahead"\n\t\t\texpression: JSRegExpSubExpression {\n\t\t\t\tbody: [\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "a"\n\t\t\t\t\t\tloc: SourceLocation unknown 1:7-1:8\n\t\t\t\t\t}\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "h"\n\t\t\t\t\t\tloc: SourceLocation unknown 1:8-1:9\n\t\t\t\t\t}\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "e"\n\t\t\t\t\t\tloc: SourceLocation unknown 1:9-1:10\n\t\t\t\t\t}\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "a"\n\t\t\t\t\t\tloc: SourceLocation unknown 1:10-1:11\n\t\t\t\t\t}\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "d"\n\t\t\t\t\t\tloc: SourceLocation unknown 1:11-1:12\n\t\t\t\t\t}\n\t\t\t\t]\n\t\t\t\tloc: SourceLocation unknown 1:7-1:12\n\t\t\t}\n\t\t\tloc: SourceLocation unknown 1:4-1:12\n\t\t}\n\t]\n\tloc: SourceLocation unknown 1:0-1:12\n}',
		);
	},
);

test(
	"parses escape characters correctly",
	(t) => {
		const node1 = parseRegExp({
			input: "f(o)\\1",
			unicode: false,
		}).expression;

		t.snapshot(node1);

		const node2 = parseRegExp({
			input: "f(o)\\0",
			unicode: false,
		}).expression;

		t.snapshot(node2);

		const node3 = parseRegExp({
			input: "f(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)\\11",
			unicode: false,
		}).expression;

		t.snapshot(node3);

		const node4 = parseRegExp({
			input: "f(a)\\2",
			unicode: false,
		}).expression;

		t.snapshot(node4);

		const node5 = parseRegExp({
			input: "[\u0400-\u04ff]+\\0",
			unicode: false,
		}).expression;

		t.snapshot(node5);

		const node6 = parseRegExp({
			input: '(?<quote>[\'"])\\k<quote>',
			unicode: true,
		}).expression;

		t.snapshot(node6);

		const result1 = parseRegExp({
			input: '(?<quote>[\'"])\\k<quote',
			unicode: true,
		});

		t.snapshot(result1);
	},
);
