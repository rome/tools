import {test} from "rome";
import {
	comparePositions,
	extractSourceLocationRangeFromNodes,
	isAlpha,
	isDigit,
	isESIdentifierChar,
	isESIdentifierStart,
	isHexDigit,
	readUntilLineBreak,
} from "@internal/parser-core/utils";
import {ob1Coerce0, ob1Coerce1} from "@internal/ob1";
import {dedent} from "@internal/string-utils";
import {parseJS} from "@internal/js-parser";
import {
	jsBlockStatement,
	jsIfStatement,
	jsNumericLiteral,
	jsReturnStatement,
} from "@internal/ast";

test(
	"",
	(t) => {
		t.true(isDigit("4"));
		t.false(isDigit("o"));

		t.true(isAlpha("H"));
		t.false(isAlpha("?"));

		t.true(isHexDigit("9"));
		t.false(isHexDigit("z"));

		t.true(isESIdentifierChar("4"));
		t.false(isESIdentifierChar("/"));

		t.true(isESIdentifierStart("_"));
		t.false(isESIdentifierStart("4"));

		t.true(readUntilLineBreak("a"));
		t.false(readUntilLineBreak("\n"));

		t.is(
			comparePositions(
				{
					column: ob1Coerce0(4),
					line: ob1Coerce1(2),
				},
				{
					column: ob1Coerce0(4),
					line: ob1Coerce1(3),
				},
			),
			-1,
		);

		t.is(
			comparePositions(
				{
					column: ob1Coerce0(4),
					line: ob1Coerce1(9),
				},
				{
					column: ob1Coerce0(4),
					line: ob1Coerce1(9),
				},
			),
			0,
		);

		t.is(
			comparePositions(
				{
					column: ob1Coerce0(9),
					line: ob1Coerce1(4),
				},
				{
					column: ob1Coerce0(6),
					line: ob1Coerce1(4),
				},
			),
			1,
		);

		const js = parseJS({
			path: "unknown",
			input: dedent`
				const foo = bar;

				hello();

				if (foo === bar) {
					return 4;
				}

				return 0;
			`,
		});

		t.inlineSnapshot(
			extractSourceLocationRangeFromNodes([
				js.body[0],
				js.body[1],
				jsNumericLiteral.assert(
					jsReturnStatement.assert(
						jsBlockStatement.assert(jsIfStatement.assert(js.body[2]).consequent).body,
					).argument,
				),
			]),
			'Object {\n\tfilename: "unknown"\n\tend: Object {\n\t\tcolumn: 9\n\t\tline: 6\n\t}\n\tstart: Object {\n\t\tcolumn: 0\n\t\tline: 1\n\t}\n}',
		);
	},
);
