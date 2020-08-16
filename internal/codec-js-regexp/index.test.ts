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
			'JSRegExpSubExpression {\n\tloc: Object {\n\t\tfilename: "unknown"\n\t\tend: Object {\n\t\t\tcolumn: 7\n\t\t\tline: 1\n\t\t}\n\t\tstart: Object {\n\t\t\tcolumn: 0\n\t\t\tline: 1\n\t\t}\n\t}\n\tbody: Array [\n\t\tJSRegExpCharSet {\n\t\t\tinvert: false\n\t\t\tloc: Object {\n\t\t\t\tfilename: "unknown"\n\t\t\t\tend: Object {\n\t\t\t\t\tcolumn: 5\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t\tstart: Object {\n\t\t\t\t\tcolumn: 0\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t}\n\t\t\tbody: Array [\n\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\tvalue: "a"\n\t\t\t\t\tloc: Object {\n\t\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\t\tend: Object {\n\t\t\t\t\t\t\tcolumn: 2\n\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t}\n\t\t\t\t\t\tstart: Object {\n\t\t\t\t\t\t\tcolumn: 1\n\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\tvalue: "b"\n\t\t\t\t\tloc: Object {\n\t\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\t\tend: Object {\n\t\t\t\t\t\t\tcolumn: 3\n\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t}\n\t\t\t\t\t\tstart: Object {\n\t\t\t\t\t\t\tcolumn: 2\n\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\tvalue: "c"\n\t\t\t\t\tloc: Object {\n\t\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\t\tend: Object {\n\t\t\t\t\t\t\tcolumn: 4\n\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t}\n\t\t\t\t\t\tstart: Object {\n\t\t\t\t\t\t\tcolumn: 3\n\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tJSRegExpNonWordCharacter {\n\t\t\tloc: Object {\n\t\t\t\tfilename: "unknown"\n\t\t\t\tend: Object {\n\t\t\t\t\tcolumn: 7\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t\tstart: Object {\n\t\t\t\t\tcolumn: 5\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t]\n}',
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
			'JSRegExpSubExpression {\n\tloc: Object {\n\t\tfilename: "unknown"\n\t\tend: Object {\n\t\t\tcolumn: 10\n\t\t\tline: 1\n\t\t}\n\t\tstart: Object {\n\t\t\tcolumn: 0\n\t\t\tline: 1\n\t\t}\n\t}\n\tbody: Array [\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "f"\n\t\t\tloc: Object {\n\t\t\t\tfilename: "unknown"\n\t\t\t\tend: Object {\n\t\t\t\t\tcolumn: 1\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t\tstart: Object {\n\t\t\t\t\tcolumn: 0\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tJSRegExpQuantified {\n\t\t\tlazy: false\n\t\t\tmax: 2\n\t\t\tmin: 2\n\t\t\tloc: Object {\n\t\t\t\tfilename: "unknown"\n\t\t\t\tend: Object {\n\t\t\t\t\tcolumn: 5\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t\tstart: Object {\n\t\t\t\t\tcolumn: 1\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t}\n\t\t\ttarget: JSRegExpCharacter {\n\t\t\t\tvalue: "o"\n\t\t\t\tloc: Object {\n\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\tend: Object {\n\t\t\t\t\t\tcolumn: 2\n\t\t\t\t\t\tline: 1\n\t\t\t\t\t}\n\t\t\t\t\tstart: Object {\n\t\t\t\t\t\tcolumn: 1\n\t\t\t\t\t\tline: 1\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tJSRegExpQuantified {\n\t\t\tlazy: false\n\t\t\tmax: 1\n\t\t\tmin: 0\n\t\t\tloc: Object {\n\t\t\t\tfilename: "unknown"\n\t\t\t\tend: Object {\n\t\t\t\t\tcolumn: 10\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t\tstart: Object {\n\t\t\t\t\tcolumn: 5\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t}\n\t\t\ttarget: JSRegExpGroupCapture {\n\t\t\t\tname: undefined\n\t\t\t\tloc: Object {\n\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\tend: Object {\n\t\t\t\t\t\tcolumn: 10\n\t\t\t\t\t\tline: 1\n\t\t\t\t\t}\n\t\t\t\t\tstart: Object {\n\t\t\t\t\t\tcolumn: 5\n\t\t\t\t\t\tline: 1\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t\texpression: JSRegExpSubExpression {\n\t\t\t\t\tloc: Object {\n\t\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\t\tend: Object {\n\t\t\t\t\t\t\tcolumn: 9\n\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t}\n\t\t\t\t\t\tstart: Object {\n\t\t\t\t\t\t\tcolumn: 6\n\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\tbody: Array [\n\t\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\t\tvalue: "b"\n\t\t\t\t\t\t\tloc: Object {\n\t\t\t\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\t\t\t\tend: Object {\n\t\t\t\t\t\t\t\t\tcolumn: 7\n\t\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t\tstart: Object {\n\t\t\t\t\t\t\t\t\tcolumn: 6\n\t\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t}\n\t\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\t\tvalue: "a"\n\t\t\t\t\t\t\tloc: Object {\n\t\t\t\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\t\t\t\tend: Object {\n\t\t\t\t\t\t\t\t\tcolumn: 8\n\t\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t\tstart: Object {\n\t\t\t\t\t\t\t\t\tcolumn: 7\n\t\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t}\n\t\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\t\tvalue: "r"\n\t\t\t\t\t\t\tloc: Object {\n\t\t\t\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\t\t\t\tend: Object {\n\t\t\t\t\t\t\t\t\tcolumn: 9\n\t\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t\tstart: Object {\n\t\t\t\t\t\t\t\t\tcolumn: 8\n\t\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t}\n\t\t\t\t\t]\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t]\n}',
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
			'JSRegExpSubExpression {\n\tloc: Object {\n\t\tfilename: "unknown"\n\t\tend: Object {\n\t\t\tcolumn: 12\n\t\t\tline: 1\n\t\t}\n\t\tstart: Object {\n\t\t\tcolumn: 0\n\t\t\tline: 1\n\t\t}\n\t}\n\tbody: Array [\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "l"\n\t\t\tloc: Object {\n\t\t\t\tfilename: "unknown"\n\t\t\t\tend: Object {\n\t\t\t\t\tcolumn: 1\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t\tstart: Object {\n\t\t\t\t\tcolumn: 0\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "o"\n\t\t\tloc: Object {\n\t\t\t\tfilename: "unknown"\n\t\t\t\tend: Object {\n\t\t\t\t\tcolumn: 2\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t\tstart: Object {\n\t\t\t\t\tcolumn: 1\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "o"\n\t\t\tloc: Object {\n\t\t\t\tfilename: "unknown"\n\t\t\t\tend: Object {\n\t\t\t\t\tcolumn: 3\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t\tstart: Object {\n\t\t\t\t\tcolumn: 2\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tJSRegExpCharacter {\n\t\t\tvalue: "k"\n\t\t\tloc: Object {\n\t\t\t\tfilename: "unknown"\n\t\t\t\tend: Object {\n\t\t\t\t\tcolumn: 4\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t\tstart: Object {\n\t\t\t\t\tcolumn: 3\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tJSRegExpGroupNonCapture {\n\t\t\tkind: "positive-lookahead"\n\t\t\tloc: Object {\n\t\t\t\tfilename: "unknown"\n\t\t\t\tend: Object {\n\t\t\t\t\tcolumn: 12\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t\tstart: Object {\n\t\t\t\t\tcolumn: 4\n\t\t\t\t\tline: 1\n\t\t\t\t}\n\t\t\t}\n\t\t\texpression: JSRegExpSubExpression {\n\t\t\t\tloc: Object {\n\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\tend: Object {\n\t\t\t\t\t\tcolumn: 12\n\t\t\t\t\t\tline: 1\n\t\t\t\t\t}\n\t\t\t\t\tstart: Object {\n\t\t\t\t\t\tcolumn: 7\n\t\t\t\t\t\tline: 1\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t\tbody: Array [\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "a"\n\t\t\t\t\t\tloc: Object {\n\t\t\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\t\t\tend: Object {\n\t\t\t\t\t\t\t\tcolumn: 8\n\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\tstart: Object {\n\t\t\t\t\t\t\t\tcolumn: 7\n\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "h"\n\t\t\t\t\t\tloc: Object {\n\t\t\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\t\t\tend: Object {\n\t\t\t\t\t\t\t\tcolumn: 9\n\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\tstart: Object {\n\t\t\t\t\t\t\t\tcolumn: 8\n\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "e"\n\t\t\t\t\t\tloc: Object {\n\t\t\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\t\t\tend: Object {\n\t\t\t\t\t\t\t\tcolumn: 10\n\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\tstart: Object {\n\t\t\t\t\t\t\t\tcolumn: 9\n\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "a"\n\t\t\t\t\t\tloc: Object {\n\t\t\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\t\t\tend: Object {\n\t\t\t\t\t\t\t\tcolumn: 11\n\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\tstart: Object {\n\t\t\t\t\t\t\t\tcolumn: 10\n\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\tJSRegExpCharacter {\n\t\t\t\t\t\tvalue: "d"\n\t\t\t\t\t\tloc: Object {\n\t\t\t\t\t\t\tfilename: "unknown"\n\t\t\t\t\t\t\tend: Object {\n\t\t\t\t\t\t\t\tcolumn: 12\n\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\tstart: Object {\n\t\t\t\t\t\t\t\tcolumn: 11\n\t\t\t\t\t\t\t\tline: 1\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t]\n\t\t\t}\n\t\t}\n\t]\n}',
		);
	},
);
