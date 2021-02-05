import {test} from "rome";
import {
	comparePositions,
	extractSourceLocationRangeFromNodes,
	isAlpha,
	isDigit,
	isESIdentifierChar,
	isESIdentifierStart,
	isHexDigit,
	isntLineBreak,
} from "./utils";
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

		t.true(isntLineBreak("a"));
		t.false(isntLineBreak("\n"));

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
			"SourceLocation unknown 1:0-6:9",
		);
	},
);
