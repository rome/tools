import {test} from "rome";
import {SourceLocation} from "@internal/parser-core";
import {inheritLoc} from "@internal/js-ast-utils/inheritLoc";
import {jsCommentLine, jsIdentifier} from "@internal/ast";
import {UNKNOWN_PATH} from "@internal/path";
import {OneIndexed, ZeroIndexed} from "@internal/numbers";

test(
	"returns the node's source location with it's name",
	async (t) => {
		const commentLoc: SourceLocation = {
			path: UNKNOWN_PATH,
			start: {
				line: new OneIndexed(),
				column: new ZeroIndexed(),
			},
			end: {
				line: new OneIndexed(),
				column: new ZeroIndexed(13),
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
				line: new OneIndexed(3),
				column: new ZeroIndexed(),
			},
			end: {
				line: new OneIndexed(3),
				column: new ZeroIndexed(15),
			},
		};

		t.looksLike(
			inheritLoc(jsIdentifier.create({name: "bar", loc: identifierLoc})),
			{identifierName: "bar", ...identifierLoc},
		);
	},
);
