import {test} from "rome";
import {
	canBeLeftFlankingDelimiter,
	canBeRightFlankingDelimiter,
} from "@internal/markdown-parser/utils";
import {ob1Coerce0} from "@internal/ob1";

test(
	"correctly check the left-flanking delimiters",
	(t) => {
		t.is(
			canBeLeftFlankingDelimiter({
				input: ' **"abc" ',
				endIndex: ob1Coerce0(2),
				startIndex: ob1Coerce0(1),
			}),
			true,
		);

		t.is(
			canBeLeftFlankingDelimiter({
				input: " ***abc ",
				endIndex: ob1Coerce0(2),
				startIndex: ob1Coerce0(1),
			}),
			true,
		);
		t.is(
			canBeLeftFlankingDelimiter({
				input: " _abc",
				endIndex: ob1Coerce0(1),
				startIndex: ob1Coerce0(1),
			}),
			true,
		);

		t.is(
			canBeLeftFlankingDelimiter({
				input: ' _"abc"',
				endIndex: ob1Coerce0(1),
				startIndex: ob1Coerce0(1),
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
				endIndex: ob1Coerce0(4),
				startIndex: ob1Coerce0(5),
			}),
			true,
		);

		t.is(
			canBeRightFlankingDelimiter({
				input: "abc_ ",
				endIndex: ob1Coerce0(3),
				startIndex: ob1Coerce0(3),
			}),
			true,
		);

		t.is(
			canBeRightFlankingDelimiter({
				input: '"abc"** ',
				endIndex: ob1Coerce0(6),
				startIndex: ob1Coerce0(5),
			}),
			true,
		);

		t.is(
			canBeRightFlankingDelimiter({
				input: '"abc"_ ',
				endIndex: ob1Coerce0(5),
				startIndex: ob1Coerce0(5),
			}),
			true,
		);
	},
);
