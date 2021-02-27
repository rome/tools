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
import {dedent} from "@internal/string-utils";
import {parseJS} from "@internal/js-parser";
import {
	jsBlockStatement,
	jsIfStatement,
	jsNumericLiteral,
	jsReturnStatement,
} from "@internal/ast";
import {OneIndexed, ZeroIndexed} from "@internal/numbers";

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
					column: new ZeroIndexed(4),
					line: new OneIndexed(2),
				},
				{
					column: new ZeroIndexed(4),
					line: new OneIndexed(3),
				},
			),
			-1,
		);

		t.is(
			comparePositions(
				{
					column: new ZeroIndexed(4),
					line: new OneIndexed(9),
				},
				{
					column: new ZeroIndexed(4),
					line: new OneIndexed(9),
				},
			),
			0,
		);

		t.is(
			comparePositions(
				{
					column: new ZeroIndexed(9),
					line: new OneIndexed(4),
				},
				{
					column: new ZeroIndexed(6),
					line: new OneIndexed(4),
				},
			),
			1,
		);

		const js = parseJS({
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
