import {test} from "rome";
import {
	canBeLeftFlankingDelimiter,
	canBeRightFlankingDelimiter,
} from "@internal/markdown-parser/utils";
import {ZeroIndexed} from "@internal/numbers";

test(
	"correctly check the left-flanking delimiters",
	(t) => {
		t.is(
			canBeLeftFlankingDelimiter({
				input: ' **"abc" ',
				endIndex: new ZeroIndexed(2),
				startIndex: new ZeroIndexed(1),
			}),
			true,
		);

		t.is(
			canBeLeftFlankingDelimiter({
				input: " ***abc ",
				endIndex: new ZeroIndexed(2),
				startIndex: new ZeroIndexed(1),
			}),
			true,
		);
		t.is(
			canBeLeftFlankingDelimiter({
				input: " _abc",
				endIndex: new ZeroIndexed(1),
				startIndex: new ZeroIndexed(1),
			}),
			true,
		);

		t.is(
			canBeLeftFlankingDelimiter({
				input: ' _"abc"',
				endIndex: new ZeroIndexed(1),
				startIndex: new ZeroIndexed(1),
			}),
			true,
		);
	},
);

test(
	"correctly checks the right-flanking delimiters",
	(t) => {
		t.is(
			canBeRightFlankingDelimiter({
				input: "abc*** ",
				endIndex: new ZeroIndexed(4),
				startIndex: new ZeroIndexed(5),
			}),
			true,
		);

		t.is(
			canBeRightFlankingDelimiter({
				input: "abc_ ",
				endIndex: new ZeroIndexed(3),
				startIndex: new ZeroIndexed(3),
			}),
			true,
		);

		t.is(
			canBeRightFlankingDelimiter({
				input: '"abc"** ',
				endIndex: new ZeroIndexed(6),
				startIndex: new ZeroIndexed(5),
			}),
			true,
		);

		t.is(
			canBeRightFlankingDelimiter({
				input: '"abc"_ ',
				endIndex: new ZeroIndexed(5),
				startIndex: new ZeroIndexed(5),
			}),
			true,
		);
	},
);
