import {test} from "rome";
import {SourceLocation} from "@internal/parser-core";
import {ob1Coerce0, ob1Coerce1} from "@internal/ob1";
import {inheritLoc} from "@internal/js-ast-utils/inheritLoc";
import {jsCommentLine, jsIdentifier} from "@internal/ast";
import {UNKNOWN_PATH} from "@internal/path";

test(
	"returns the node's source location with it's name",
	async (t) => {
		const commentLoc: SourceLocation = {
			path: UNKNOWN_PATH,
			start: {
				line: ob1Coerce1(1),
				column: ob1Coerce0(0),
			},
			end: {
				line: ob1Coerce1(1),
				column: ob1Coerce0(13),
			},
		};

		t.is(
			inheritLoc(
				jsCommentLine.create({id: "1", value: "hello world", loc: commentLoc}),
			),
			commentLoc,
		);

		t.looksLike(
			inheritLoc(
				jsCommentLine.create({id: "1", value: "hello world", loc: commentLoc}),
				"foo",
			),
			{identifierName: "foo", ...commentLoc},
		);

		const identifierLoc: SourceLocation = {
			path: UNKNOWN_PATH,
			start: {
				line: ob1Coerce1(3),
				column: ob1Coerce0(0),
			},
			end: {
				line: ob1Coerce1(3),
				column: ob1Coerce0(15),
			},
		};

		t.looksLike(
			inheritLoc(jsIdentifier.create({name: "bar", loc: identifierLoc})),
			{identifierName: "bar", ...identifierLoc},
		);
	},
);
